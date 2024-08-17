#[derive(Debug, Clone, Copy)]
pub enum BusServiceState {
    OnRoute,
    OutOfService,
    Resting,
    Accident,
    Off,
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
