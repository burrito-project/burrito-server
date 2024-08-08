#[derive(Debug)]
pub enum BusServiceState {
    OnRoute,
    OutOfService,
    Resting,
    Accident,
    Off,
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
