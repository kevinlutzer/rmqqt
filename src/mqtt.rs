use futures_util::StreamExt;
use paho_mqtt::{AsyncClient, CreateOptionsBuilder, Message};
use std::env;

/// Configuration used for the MQTT client. These fields are mapped
/// from the cli::Cli struct
pub struct ClientConfig {
    pub host: String,
    pub port: u16,
}

/// Creates a new instance of the ClientConfig struct.
/// This struct is used to configure build a new MQTT client.
impl ClientConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
}

/// Use this function to connect the client to the MQTT broker. The client must be instantiated
/// before calling this function.
pub async fn connect_client(client: &AsyncClient) -> anyhow::Result<()> {
    // Set up the connection options and then connect
    let connect_options = paho_mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(60))
        .clean_session(true)
        .finalize();

    client
        .connect(connect_options)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to MQTT server: {}", e))
        .map(|_| ())
}

/// Creates a new instance of the paho_mqtt client.
pub fn build_client(config: ClientConfig) -> anyhow::Result<AsyncClient> {
    // Set up the MQTT client options.
    // TODO - add TLS support, add username/password support
    let create_options = CreateOptionsBuilder::new()
        .server_uri(format!("{}:{}", config.host, config.port))
        .client_id(env!("CARGO_PKG_NAME"))
        .finalize();

    paho_mqtt::AsyncClient::new(create_options)
        .map_err(|e| anyhow::anyhow!("Failed to create MQTT client: {}", e))
}

/// Subscribe to the specified topic. The QOS used is the same for all topic
pub async fn subscribe(client: &mut AsyncClient, topic: String, qos: i32) -> anyhow::Result<()> {
    // Get a reference to a stream with an unbounded buffer
    let mut stream = client.get_stream(None);

    // Actually subscribe and get the token for the subscription
    client
        .subscribe(&topic, qos)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create subscription with error: {}", e))?;

    // Keep pulling messages from the stream
    // and print them to the console
    while let Some(msg_opt) = stream.next().await {
        if let Some(msg) = msg_opt {
            println!("{}", msg.payload_str());
        }
    }

    Ok(())
}

/// Publishes a message to the specified topics. The QOS used is the same for all messages published topics
pub async fn publish(
    client: &AsyncClient,
    topic: String,
    payload: &str,
    qos: i32,
) -> anyhow::Result<()> {
    // Publish the message to the topic with the specified QOS
    let msg = Message::new(topic.clone(), payload, qos);
    client
        .publish(msg)
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to publish message to topic {} with error: {}",
                topic,
                e
            )
        })
        .map(|_| ())
}
