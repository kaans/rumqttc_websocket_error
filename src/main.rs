use rumqttc::v5::{AsyncClient, EventLoop, MqttOptions};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    println!("rumqttc websocket test");

    let task_handle = connect().await;

    task_handle.await.expect("Task already finished");
}

pub async fn connect() -> JoinHandle<()> {
    let options = MqttOptions::new("testid", "localhost", 1883);

    let (_client, event_loop) = AsyncClient::new(options, 10);

    let task_handle: JoinHandle<()> = start_connection_task(event_loop).await;

    task_handle
}

async fn start_connection_task(mut event_loop: EventLoop) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        loop {
            match event_loop.poll().await {
                Ok(event) => {
                    println!("Received {:?}", &event);
                }
                Err(e) => {
                    match e {
                        _ => {
                            println!("Error while processing mqtt loop: {:?}", e);
                            return;
                        }
                    }
                }
            }
        }
    })
}
