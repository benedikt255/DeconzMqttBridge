use serde::{Deserialize, Serialize};


 #[derive(Serialize, Deserialize, Debug)]
 pub struct ConfigData {
    pub broker_url: String,
    pub broker_port: u16,
    pub broker_conn_timeout: u64,
    pub deconz_url: String,
    pub wait_time: u64,
    pub ws_event: WsEventData,
 }

 #[derive(Serialize, Deserialize, Debug)]
pub struct WsEventData {
    pub added: Vec<EventAddedData>,
    pub changed: EventChangedData,
    pub deleted: Vec<EventDeletedData>,
    pub scene_called: Vec<EventSceneCalledData>,
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct EventChangedData {
    pub lights: Vec<EventChangedLightsData>,
    pub sensors: Vec<EventChangedSensorsData>,
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct EventChangedSensorsData {
    pub config_items: Vec<EventChangedConfigItemData>,
    pub id: String,
    pub name: String,
    pub state_items: Vec<EventChangedStateItemData>,
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct EventChangedLightsData {
    pub config_items: Vec<EventChangedConfigItemData>,
    pub id: String,
    pub name: String,
    pub state_items: Vec<EventChangedStateItemData>,
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct EventChangedConfigItemData {
    pub field: String,
    pub mqtt_topic: String
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct EventChangedStateItemData {
    pub conversation_factor: f64,
    pub field: String,
    pub mqtt_topic: String,
    pub retain: bool,
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct EventAddedData {
    pub event: String,
    pub id: String,
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct EventDeletedData {
    pub event: String,
    pub id: String,
 }


 #[derive(Serialize, Deserialize, Debug)]
 pub struct EventSceneCalledData {
    pub gid: String,
    pub scid: String,
 }