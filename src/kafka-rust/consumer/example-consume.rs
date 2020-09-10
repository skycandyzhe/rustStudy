extern crate kafka;
extern crate env_logger;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

/// This program demonstrates consuming messages through a `Consumer`.
/// This is a convenient client that will fit most use cases.  Note
/// that messages must be marked and commited as consumed to ensure
/// only once delivery.
fn main() {
    env_logger::init().unwrap();

    let broker = "192.168.1.128:9092".to_owned();
    let topic = "control".to_owned();
    let group = "rustkafkatest".to_owned();

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        println!("Failed consuming messages: {}", e);
    }
}

fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut con = r#try!(
        Consumer::from_hosts(brokers)
            .with_topic(topic)
            .with_group(group)
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
    );

    loop {
        let mss = r#try!(con.poll());
        if mss.is_empty() {
            println!("No messages available right now.");
            return Ok(());
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                let s = String::from_utf8( m.value.to_vec()).expect("Found invalid UTF-8");
                println!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, s);
            }
            let _ = con.consume_messageset(ms);
        }
        r#try!(con.commit_consumed());
    }
}