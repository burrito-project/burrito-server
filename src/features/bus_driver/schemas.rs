use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::features::bus_stops::schemas::BusStopInfo;

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WsClientMessage {
    pub record: BurritoPosRecord,
    pub last_stop: Option<BusStopInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BusServiceState {
    OnRoute,
    OutOfService,
    Resting,
    Accident,
    Off,
}

impl BusServiceState {
    pub fn is_locatable(&self) -> bool {
        matches!(self, BusServiceState::OnRoute | BusServiceState::Accident)
    }
}

impl<'de> serde::Deserialize<'de> for BusServiceState {
    fn deserialize<D>(deserializer: D) -> Result<BusServiceState, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let i = i32::deserialize(deserializer)?;
        BusServiceState::try_from(i).map_err(|_| {
            serde::de::Error::custom(format!("Invalid value for BusServiceState: {}", i))
        })
    }
}

impl serde::Serialize for BusServiceState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        i32::from(*self).serialize(serializer)
    }
}

impl BusServiceState {
    /// When the burrito stops reporting, we need to infer its state based on the last state it
    /// reported. Basically we consider it off if is was previously off, on accident or on route.
    pub fn inherit_from_inactive<S: Into<BusServiceState>>(other: S) -> BusServiceState {
        match other.into() {
            BusServiceState::Off => BusServiceState::Off,
            BusServiceState::OnRoute => BusServiceState::Off,
            BusServiceState::Accident => BusServiceState::Off,
            BusServiceState::OutOfService => BusServiceState::OutOfService,
            BusServiceState::Resting => BusServiceState::Resting,
        }
    }
}

impl TryFrom<i32> for BusServiceState {
    type Error = ();

    fn try_from(i: i32) -> Result<Self, ()> {
        match i {
            0 => Ok(BusServiceState::OnRoute),
            1 => Ok(BusServiceState::OutOfService),
            2 => Ok(BusServiceState::Resting),
            3 => Ok(BusServiceState::Accident),
            4 => Ok(BusServiceState::Off),
            _ => Err(()),
        }
    }
}

impl From<BusServiceState> for i32 {
    fn from(status: BusServiceState) -> Self {
        match status {
            BusServiceState::OnRoute => 0,
            BusServiceState::OutOfService => 1,
            BusServiceState::Resting => 2,
            BusServiceState::Accident => 3,
            BusServiceState::Off => 4,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BurritoPosRecord {
    pub lt: f64,
    pub lg: f64,
    pub sts: BusServiceState,
    /// Device battery. None means off or not applicable
    pub bat: Option<i32>,
    pub timestamp: SystemTime,
    pub velocity: f64,
}

impl BurritoPosRecord {
    pub fn formatted_time_ago(&self) -> String {
        let elapsed = self.timestamp.elapsed().unwrap();
        let secs = elapsed.as_secs();
        let mins = secs / 60;
        let hours = mins / 60;
        let days = hours / 24;

        if days > 0 {
            format!("hace {} días", days)
        } else if hours > 0 {
            format!("hace {} horas", hours)
        } else if mins > 0 {
            format!("hace {} minutos", mins)
        } else if secs == 0 {
            "justo ahora".to_string()
        } else {
            format!("hace {} segundos", secs)
        }
    }
}

/// The status payload received from the server, which contains the latitude, longitude
/// and status of the burrito
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BurritoRecordPayload {
    pub lt: f64,
    pub lg: f64,
    pub sts: BusServiceState, // i32
    pub bat: Option<i32>,     // Device battery. None means not applicable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from() {
        assert_eq!(BusServiceState::try_from(0), Ok(BusServiceState::OnRoute));
        assert_eq!(
            BusServiceState::try_from(1),
            Ok(BusServiceState::OutOfService)
        );
        assert_eq!(BusServiceState::try_from(2), Ok(BusServiceState::Resting));
        assert_eq!(BusServiceState::try_from(3), Ok(BusServiceState::Accident));
        assert_eq!(BusServiceState::try_from(4), Ok(BusServiceState::Off));
        assert_eq!(BusServiceState::try_from(5), Err(()));
    }

    #[test]
    fn test_from_json() {
        let raw_json = r#"[0,1,2,3,4]"#;

        let states: Vec<BusServiceState> = serde_json::from_str(&raw_json).unwrap();
        assert_eq!(
            states,
            vec![
                BusServiceState::OnRoute,
                BusServiceState::OutOfService,
                BusServiceState::Resting,
                BusServiceState::Accident,
                BusServiceState::Off
            ]
        );
    }

    #[test]
    fn test_to_json() {
        let states = vec![
            BusServiceState::OnRoute,
            BusServiceState::OutOfService,
            BusServiceState::Resting,
            BusServiceState::Accident,
            BusServiceState::Off,
        ];

        let raw_json = serde_json::to_string(&states).unwrap();
        assert_eq!(raw_json, r#"[0,1,2,3,4]"#);
    }
}
