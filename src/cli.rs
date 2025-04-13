use clap::{Parser, Subcommand, ValueEnum};

/// Quality of Service (QoS) levels for MQTT messages
#[derive(Debug, Clone, Copy, ValueEnum)]
#[repr(i32)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

/// Simple MQTT CLI tool with TLS and auth support
#[derive(Parser)]
#[command(
    name = "rmqtt",
    version,
    author,
    about = "Send and subscribe to MQTT messages"
)]
pub struct Cli {
    /// Host of the MQTT broker. This can be either a hostname or an IP address. This argument
    /// overides the .rmqqtconfig file's MQTT_HOST setting.
    #[arg(long, default_value_t = String::from("localhost"))]
    pub host: String,

    /// Port of the MQTT broker. This is usually 1883 for unencrypted connections and 8883 for TLS connections.
    /// This argument overides the .rmqqtconfig file's MQTT_PORT setting.
    #[arg(short, long, default_value_t = 1883)]
    pub port: u16,

    /// Topic to publish to or subscribe from.
    #[arg(long, short, default_value_t = String::from("test/topic"))]
    pub topic: String,

    /// QOS is the level of guarantee that the message will be delivered. The levels are:
    /// 0 - At most once (fire and forget)
    /// 1 - At least once (acknowledged)
    /// 2 - Exactly once (guaranteed delivery)
    #[arg(long, short, default_value_t = QoS::AtMostOnce, value_enum)]
    pub qos: QoS,

    #[command(subcommand)]
    pub command: Commands,
}

/// Commands for the MQTT CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Publish a message to the specified topics. The QOS used is the same for all messages published topics.  
    /// The message is sent as a utf-8 encoded string.
    Pub { message: String },

    /// Subscribe to a topic. This will continuously print messages received on the topics specified until
    /// the program is terminated or the connection to the server is lost. Received payloads are treated as utf-8 encoded strings.
    Sub {},
}
