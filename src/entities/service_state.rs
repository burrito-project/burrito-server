#[derive(Debug)]
pub enum BusServiceState {
    OnRoute,
    OutOfService,
    Resting,
    Accident,
    Off,
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

impl From<i32> for BusServiceState {
    fn from(i: i32) -> Self {
        match i {
            0 => BusServiceState::OnRoute,
            1 => BusServiceState::OutOfService,
            2 => BusServiceState::Resting,
            3 => BusServiceState::Accident,
            4 => BusServiceState::Off,
            _ => BusServiceState::Off,
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
