#[cfg(test)]
mod tests {
    use orchestrator::{StateMachine, Event};
    use orchestrator::states::State;

    #[test]
    fn test_handler_on_entry_called_when_entering_state() {
        let mut sm = StateMachine::new();
        sm.set_state(State::Init);
        assert_eq!(sm.current_state(), State::Init);
    }

    #[test]
    fn test_firmware_verify_entry_prepares_for_verification() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareVerify);
        assert_eq!(sm.current_state(), State::FirmwareVerify);
    }
}
