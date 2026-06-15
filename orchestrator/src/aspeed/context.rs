use crate::aspeed::types::{Event, EventData};
use crate::aspeed::states::State;

#[derive(Debug)]
pub struct StateMachine {
    state: State,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            state: State::Boot,
        }
    }

    pub fn current_state(&self) -> State {
        self.state
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn handle_event(&mut self, event: Event, _data: EventData) {
        // Full transition logic for Boot, Init, TMIN1, and RotRecovery states
        self.state = match (self.state, event) {
            // Boot & Init
            (State::Boot, Event::StartStateMachine) => State::Init,
            (State::Init, Event::InitDone) => State::FirmwareVerify,
            (State::Init, Event::InitRotSecondaryBooted) => State::RotRecovery,

            // TMIN1 hierarchy (FirmwareVerify)
            (State::FirmwareVerify, Event::VerifyDone) => State::Tzero,
            (State::FirmwareVerify, Event::VerifyFailed) => State::FirmwareRecovery,
            (State::FirmwareVerify, Event::UpdateRequested) => State::FirmwareUpdate,
            (State::FirmwareVerify, Event::VerifyUnprovisioned) => State::Unprovisioned,

            // TMIN1 hierarchy (FirmwareRecovery)
            (State::FirmwareRecovery, Event::RecoveryDone) => State::FirmwareVerify,
            (State::FirmwareRecovery, Event::RecoveryFailed) => State::SystemLockdown,

            // TMIN1 hierarchy (FirmwareUpdate)
            (State::FirmwareUpdate, Event::UpdateDone) => State::FirmwareVerify,
            (State::FirmwareUpdate, Event::UpdateFailed) => State::FirmwareVerify,

            // RotRecovery
            (State::RotRecovery, Event::RecoveryDone) => State::SystemReboot,
            (State::RotRecovery, Event::RecoveryFailed) => State::SystemLockdown,

            _ => self.state,
        };
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}
