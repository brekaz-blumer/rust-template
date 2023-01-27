use rdkafka::{Message};
use futures::{TryStreamExt};
use rdkafka::message::{BorrowedMessage, OwnedMessage};


pub struct Reducer {
}

impl Reducer {
    async fn record_borrowed_message_receipt(msg: &BorrowedMessage<'_>) {
        // Simulate some work that must be done in the same order as messages are
        // received; i.e., before truly parallel processing can begin.
        println!("Message received: {:?}", msg.offset());
    }

    async fn record_owned_message_receipt(&self, _msg: &OwnedMessage) {
        // Like `record_borrowed_message_receipt`, but takes an `OwnedMessage`
        // instead, as in a real-world use case  an `OwnedMessage` might be more
        // convenient than a `BorrowedMessage`.
        match _msg.topic() {
            "kafka" => {
                let payload = _msg.payload().expect("Kafka message should contain payload");
            }
            _ => {}
        }
    }

    pub async fn start_consumer(&self) -> () {
        let consumer = crate::infrastructure::kafka::create_consumer("kafka".to_string());

        let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
            println!("Message received: {}", borrowed_message.offset());
            async move {
                // Process each message
                Reducer::record_borrowed_message_receipt(&borrowed_message).await;
                // Borrowed messages can't outlive the consumer they are received from, so they need to
                // be owned in order to be sent to a separate thread.
                let owned_message = borrowed_message.detach();
                self.record_owned_message_receipt(&owned_message).await;
                Ok(())
            }
        });

        println!("Starting event loop");
        stream_processor.await.expect("stream processing failed");
        println!("Stream processing terminated");
    }
}