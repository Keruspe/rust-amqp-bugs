extern crate amqp;
extern crate std_semaphore;

use std::{env,thread};
use std::sync::Arc;
use std::time::Duration;

use amqp::{Basic,Session};
use std_semaphore::Semaphore;

fn main() {
    let mut session = Session::open_url(env::var("AMQP_URL").unwrap().as_str()).unwrap();
    let mut chan    = 1;
    let semaphore   = Semaphore::new(5);
    let sem_shared  = Arc::new(semaphore);

    loop {
        let _sem = sem_shared.clone();
        _sem.acquire();

        let mut channel = session.open_channel(chan).unwrap();
        chan += 1;

        thread::spawn(move || {
            for get_result in channel.basic_get("keruspe.bug.test", false) {
                println!("New message: {}", String::from_utf8_lossy(&get_result.body));
                thread::sleep(Duration::from_secs(2));
                get_result.ack();
            }

            println!("No more messages, waiting for 60 seconds");
            thread::sleep(Duration::from_secs(60));
            _sem.release();
        });
    }
}
