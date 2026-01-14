use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, PartialEq)]
pub struct Serializeble {
    pub battery: Option<BatteriesState>,
}

#[derive(Serialize, PartialEq)]
pub struct BatteriesState {
    pub batteries: HashMap<String, BatteryState>,
}

#[derive(Serialize, PartialEq)]
pub struct BatteryState {
    pub percentage: u32,
    pub state: BatState,
    pub time: String,
    pub symbol: &'static str,
}

#[derive(Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BatState {
    Charging,
    Discharging,
    Unknown,
    Empty,
    Full,
}
