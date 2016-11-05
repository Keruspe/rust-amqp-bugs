extern crate amqp;
extern crate std_semaphore;

use std::{env,thread};
use std::sync::Arc;
use std::time::Duration;

use amqp::{Basic,Session};
use std_semaphore::Semaphore;

fn main() {
    let mut session = Session::open_url(env::var("AMQP_URL").unwrap().as_str()).unwrap();
    let mut channel = session.open_channel(1).unwrap();
    let semaphore   = Semaphore::new(5);
    let sem_shared  = Arc::new(semaphore);

    loop {
        for get_result in channel.basic_get("keruspe.bug.test", false) {
            let _sem = sem_shared.clone();
            _sem.acquire();
            thread::spawn(move || {
                println!("New message: {}", String::from_utf8_lossy(&get_result.body));
                thread::sleep(Duration::from_secs(2));
                get_result.ack();
                _sem.release();
            });
        }

        println!("No more messages, waiting for 60 seconds");
        thread::sleep(Duration::from_secs(60));
    }
}
