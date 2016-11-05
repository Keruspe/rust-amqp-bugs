extern crate amqp;

use std::env;

use amqp::{Basic,Session,Table};

fn main() {
    let mut session = Session::open_url(env::var("AMQP_URL").unwrap().as_str()).unwrap();
    let mut channel = session.open_channel(1).unwrap();

    channel.queue_declare("keruspe.bug.test", false, true, false, false, false, Table::new()).unwrap();

    for i in 1..100 {
        channel.basic_publish("", "keruspe.bug.test", true, false, Default::default(), format!("test {}", i).into_bytes()).unwrap();
    }

    channel.close(200, "Bye").unwrap();
    session.close(200, "Bye");
}
