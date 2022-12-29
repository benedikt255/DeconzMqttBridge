extern crate paho_mqtt as mqtt;

pub mod config;

use url::Url;
use tungstenite::connect;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::process;
use std::time::Duration;

use crate::config::{ConfigData,EventChangedData, EventChangedSensorsData};

const DFLT_CLIENT:&str = "Deconz_Mqtt_Bridge";
const QOS:i32 = 0;

fn main() {
    println!("Hello, websocket world!");

    //get config
    let file = File::open("./Config_DeconzMqttBridge.json").expect("file not found");
    let reader = BufReader::new(file);
    let config: ConfigData = serde_json::from_reader(reader).expect("error while reading or parsing");
  
    //connect to websocket
    let (mut socket, _response) = connect(
            Url::parse(&config.deconz_url).unwrap()
        ).expect("Can't connect");

    // Define the set of options for the create.
    // Use an ID for a persistent session.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(config.broker_url)
        .client_id(DFLT_CLIENT.to_string())
        .finalize();

    // Create a client.
    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    // Define the set of options for the connection.
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(config.broker_conn_timeout))
            .clean_session(true)
            .finalize();

    // Connect and wait for it to complete or fail.
    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    //infinite loop
    loop {
        let msg = socket.read_message().expect("Error reading message").into_text().unwrap();
        println!("Received: {}", msg);
        let data: Value = serde_json::from_str(&msg).unwrap();
        match data["e"].as_str().unwrap_or_default() {
            "added"=>println!("added events are not supported yet"),
            "changed"=>changed_event_data_handler(&data, &config.ws_event.changed, &cli),
            "deleted"=>println!("deleted events are not supported yet"),
            "scene-called"=>println!("scene-called events are not supported yet"),
            _=>println!("unexpected e value {}", data["e"])
        }
    }
}

// handle changed events
fn changed_event_data_handler(event_data: &Value, event_config: &EventChangedData, mqtt_client: &mqtt::Client) {
    match event_data["r"].as_str().unwrap_or_default() {
        "groups"=>println!("groups changed events are not supported yet"),
        "lights"=>println!("lights changed events are not supported yet"),
        "scenes"=>println!("scenes changed events are not supported yet"),
        "sensors"=>changed_event_sensors_data_handler(&event_data, &event_config.sensors, &mqtt_client),
        _=>println!("unexpected e value {}", event_data["e"])
    }
}

// handle sensor changed events
fn changed_event_sensors_data_handler(event_data: &Value, event_config: &Vec<EventChangedSensorsData>, mqtt_client: &mqtt::Client) {
    for elem in event_config {
        if elem.id == event_data["id"].as_str().unwrap_or_default() {
            for state_elem in &elem.state_items {
                //event may not contain state info
                if !event_data["state"].is_null() {
                    let value: f64 = state_elem.conversation_factor * event_data["state"][state_elem.field.as_str()].as_f64().unwrap_or_default();
                    // we do not want to retain all messages but some
                    let msg = 
                        if state_elem.retain {mqtt::Message::new_retained(state_elem.mqtt_topic.as_str(), value.to_string(), QOS)} 
                        else {mqtt::Message::new(state_elem.mqtt_topic.as_str(), value.to_string(), QOS)};
                    let tok = mqtt_client.publish(msg);
                    println!("published {} value {}", state_elem.mqtt_topic, value);

                    if let Err(e) = tok {
                        println!("Error sending message: {:?}", e);
                    }
                }
            };
        }
    }
}