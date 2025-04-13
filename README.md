# rmqtt

A compact rust CLI tool for publishing and subscribing to MQTT messages. The binary supports both the v5 and v3.1 versions
of the MQTT protocol.

## Setup

### Installation

Make sure you have cargo/rustc installed. If not, follow the instructions[here] to install the latest version.
 To install with cargo, run `cargo install rmqqt --git=github.com/kevinlutzer/rmqqt`.

### Configure

Create a `.rmqqtconfig` file in your home directory with the following contents:

``` bash
MQTT_HOST=<ip or dns name>
MQTT_PORT=<port>
```

Additional the CLI tool can be run without the config file, but the arguments `--host`, and `--port` must be passed.

## Examples

Here are some examples of using the tool to get you started

``` bash
rmqqt --port 1883 --topic=hello/world pub "hello world!" # Publish message to the "hello/world" topic
rmqqt --topic=hello/world sub # Listen to messages on the "hello/world" topic.
```

### Building

To build the application, simply run:

``` bash
cargo build
```