pub mod config;

use rumqttc::{MqttOptions, Client, QoS};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::thread;
use tokio::{task, time};
use tungstenite::connect;
use url::Url;

use crate::config::{ConfigData,EventChangedData, EventChangedSensorsData};

const DFLT_CLIENT:&str = "Deconz_Mqtt_Bridge";
const QOS:QoS = QoS::AtMostOnce;

fn main() {
    //get conf
    println!("DeconzMqttBridge started\n\t");
    let file = File::open("config/DeconzMqttBridge.json").expect("file not found");
    let reader = BufReader::new(file);
    let config: ConfigData = serde_json::from_reader(reader).expect("error while reading or parsing");

    println!("config read, wait configured time\n\t");
  
    thread::sleep(Duration::from_secs(config.wait_time));

    println!("wait finished, connect\n\t");

    //connect to websocket
    let (mut socket, _response) = connect(
            Url::parse(&config.deconz_url).unwrap()
        ).expect("Can't connect");


    let mut mqttoptions = MqttOptions::new(DFLT_CLIENT.to_string(), config.broker_url, config.broker_port);
        mqttoptions.set_keep_alive(Duration::from_secs(config.broker_conn_timeout));

    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    task::spawn(async move { 
        for (i, notification) in connection.iter().enumerate() {
            println!("Notification = {:?}", notification);
        }
    });

    println!("connected, enter loop\n\t");

    //infinite loop
    loop {
        let msg = socket.read_message().expect("Error reading message").into_text().unwrap();
        println!("Received: {}", msg);
        let data: Value = serde_json::from_str(&msg).unwrap();
        match data["e"].as_str().unwrap_or_default() {
            "added"=>println!("added events are not supported yet"),
            "changed"=>changed_event_data_handler(&data, &config.ws_event.changed, &mut client),
            "deleted"=>println!("deleted events are not supported yet"),
            "scene-called"=>println!("scene-called events are not supported yet"),
            _=>println!("unexpected e value {}", data["e"])
        }
    }
}

// handle changed events
fn changed_event_data_handler(event_data: &Value, event_config: &EventChangedData, mqtt_client: &mut Client) {
    match event_data["r"].as_str().unwrap_or_default() {
        "groups"=>println!("groups changed events are not supported yet"),
        "lights"=>println!("lights changed events are not supported yet"),
        "scenes"=>println!("scenes changed events are not supported yet"),
        "sensors"=>changed_event_sensors_data_handler(&event_data, &event_config.sensors, mqtt_client),
        _=>println!("unexpected e value {}", event_data["e"])
    }
}

// handle sensor changed events
fn changed_event_sensors_data_handler(event_data: &Value, event_config: &Vec<EventChangedSensorsData>, mqtt_client: &mut Client) {
    for elem in event_config {
        if elem.id == event_data["id"].as_str().unwrap_or_default() {
            for state_elem in &elem.state_items {
                //event may not contain state info
                if !event_data["state"].is_null() {
                    let value: f64 = state_elem.conversation_factor * event_data["state"][state_elem.field.as_str()].as_f64().unwrap_or_default();
                    // we do not want to retain all messages but some
                    let tok = mqtt_client.publish(state_elem.mqtt_topic.as_str(), QOS, state_elem.retain, value.to_string());
                    println!("published {} value {}", state_elem.mqtt_topic, value);

                    if let Err(e) = tok {
                        println!("Error sending message: {:?}", e);
                    }
                }
            };
        }
    }
}