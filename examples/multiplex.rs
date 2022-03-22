use futures::AsyncWriteExt;
use srt_rs::callback::ListenerCallbackAction;
use srt_rs::statistics::Statistics;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut multiplex = srt_rs::async_builder()
        .set_live_transmission_type()
        .set_peer_latency(1000)
        .listen("127.0.0.1:5555", 1)
        .unwrap();

    // multiplex = multiplex
    //     .with_callback(|stream_id: Option<&_>| {
    //         println!("{:?}", stream_id);
    //         ListenerCallbackAction::Allow {
    //             passphrase: Some("OK".to_string()),
    //         }
    //     })
    //     .unwrap();

    // multiplex.socket.get_stream_id().unwrap();

    // println!("{}", multiplex.socket.get_stream_id().unwrap());

    for (mut stream, sock_id) in multiplex.accept().await {
        // println!("{}", stream.get_stream_id().unwrap().len());

        let mut statistics = Statistics::new(stream.socket.id);
        let stat_task = tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                if let Err(e) = statistics.set() {
                    println!("cannot set statistics: {}", e);
                };
            }
        });

        loop {
            if let Err(e) = stream.write_all(&vec![0; 1316]).await {
                println!("{}", e);
                stat_task.abort();
                break;
            };

            sleep(Duration::from_millis(1)).await;
        }
    }
}
