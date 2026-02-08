use battery::{Battery, units::time::second};
use color_eyre::Result;
use color_eyre::eyre::bail;
use std::{collections::HashMap, time::Duration};

use crate::serializable::{BatState, BatteriesState, BatteryState};

fn map_state(state: battery::State) -> BatState {
    use crate::serializable::BatState;

    use battery::State::*;
    match state {
        Charging => BatState::Charging,
        Discharging => BatState::Discharging,
        Full => BatState::Full,
        Empty => BatState::Empty,
        _ => BatState::Unknown,
    }
}

fn get_symbol(p: u32, state: &BatState) -> &'static str {
    if !matches!(state, BatState::Discharging) && p == 100 {
        return "󰂅";
    }

    match (state, p) {
        (BatState::Charging, 5..=14) => "󰢜",
        (BatState::Charging, 15..=24) => "󰂆",
        (BatState::Charging, 25..=34) => "󰂇",
        (BatState::Charging, 35..=44) => "󰂈",
        (BatState::Charging, 45..=54) => "󰂉",
        (BatState::Charging, 55..=64) => "󰂊",
        (BatState::Charging, 65..=74) => "󰂋",
        (BatState::Charging, 75..=84) => "󰂊",
        (BatState::Charging, 81..=99) => "󰂋",
        (BatState::Charging, 100) => "󰂅",

        (BatState::Unknown, 5..=14) => "󰢜",
        (BatState::Unknown, 15..=24) => "󰂆",
        (BatState::Unknown, 25..=34) => "󰂇",
        (BatState::Unknown, 35..=44) => "󰂈",
        (BatState::Unknown, 45..=54) => "󰂉",
        (BatState::Unknown, 55..=64) => "󰂊",
        (BatState::Unknown, 65..=74) => "󰂋",
        (BatState::Unknown, 75..=84) => "󰂊",
        (BatState::Unknown, 85..=99) => "󰂋",
        (BatState::Unknown, 100) => "󰂅",

        (_, 0..=10) => "󰁺",
        (_, 11..=20) => "󰁻",
        (_, 21..=30) => "󰁼",
        (_, 31..=40) => "󰁽",
        (_, 41..=50) => "󰁾",
        (_, 51..=60) => "󰁿",
        (_, 61..=70) => "󰂀",
        (_, 71..=80) => "󰂁",
        (_, 81..=99) => "󰂂",
        (_, 100) => "󰁹",

        _ => "󰂃",
    }
}

fn battery_to_state(b: &Battery) -> BatteryState {
    let percentage = ((b.energy().value / b.energy_full().value) * 100.0).round() as u32;

    let state = map_state(b.state());

    let time = match b.state() {
        battery::State::Charging => match b.time_to_full() {
            Some(time) => {
                humantime::format_duration(Duration::from_secs(time.get::<second>() as u64))
                    .to_string()
            }
            None => "N/A".to_string(),
        },
        battery::State::Discharging => match b.time_to_empty() {
            Some(time) => {
                humantime::format_duration(Duration::from_secs(time.get::<second>() as u64))
                    .to_string()
            }
            None => "N/A".to_string(),
        },
        _ => "N/A".to_string(),
    };

    BatteryState {
        symbol: get_symbol(percentage, &state),
        percentage,
        state,
        time,
    }
}

fn get(batteries: &[battery::Battery]) -> Result<BatteriesState> {
    let mut map = HashMap::new();

    for (idx, battery) in batteries.iter().enumerate() {
        let key = format!("BAT{idx}");
        map.insert(key, battery_to_state(battery));
    }

    if map.is_empty() {
        bail!("Unable to find any batteries in system!");
    }

    Ok(BatteriesState { batteries: map })
}

pub fn get_battery_infomation() -> Option<BatteriesState> {
    let manager = battery::Manager::new().ok()?;
    let batteries: Vec<battery::Battery> = manager.batteries().ok()?.flatten().collect::<_>();

    get(&batteries).ok()
}
