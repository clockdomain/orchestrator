#[cfg(test)]
mod tests {
    use orchestrator::aspeed::{Event, EventData, State, StateMachine};

    #[test]
    fn test_state_machine_initializes_in_boot() {
        let sm = StateMachine::new();
        assert_eq!(sm.current_state(), State::Boot);
    }

    #[test]
    fn test_state_machine_can_accept_event() {
        let mut sm = StateMachine::new();
        let data = EventData::default();
        sm.handle_event(Event::StartStateMachine, data);
        // Should transition to Init
        assert_eq!(sm.current_state(), State::Init);
    }
}
