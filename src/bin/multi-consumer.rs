extern crate amqp;
extern crate std_semaphore;

use std::{env,thread};
use std::sync::Arc;
use std::time::Duration;

use amqp::{Basic,Channel,protocol,Session,Table};
use std_semaphore::Semaphore;

struct MyConsumer;

impl amqp::Consumer for MyConsumer {
    fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, headers: protocol::basic::BasicProperties, body: Vec<u8>) {
        println!("New message: {}", String::from_utf8(body).unwrap());
        thread::sleep(Duration::from_secs(2));
        channel.basic_ack(deliver.delivery_tag, false).unwrap();
    }
}

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
        let consumer    = MyConsumer {};

        thread::spawn(move || {
            channel.basic_consume(consumer, "keruspe.bug.test", "", false, false, true, false, Table::new()).unwrap();
            channel.basic_prefetch(1).unwrap();
            channel.start_consuming();
        });
    }
}
