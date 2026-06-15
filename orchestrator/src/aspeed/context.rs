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
        // Simple transition logic for now (will be expanded in Phase 3)
        self.state = match (self.state, event) {
            (State::Boot, Event::StartStateMachine) => State::Init,
            (State::Init, Event::InitDone) => State::FirmwareVerify,
            (State::Tmin1, Event::VerifyDone) => State::Tzero,
            _ => self.state,
        };
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}
