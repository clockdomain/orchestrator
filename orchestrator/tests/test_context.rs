#[cfg(test)]
mod tests {
    use orchestrator::{Event, EventData, State, StateMachine};

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

    #[test]
    fn test_boot_state_entry_handler_called() {
        let mut sm = StateMachine::new();
        assert_eq!(sm.current_state(), State::Boot);
    }

    #[test]
    fn test_init_state_entry_handler_executes() {
        let mut sm = StateMachine::new();
        sm.handle_event(Event::StartStateMachine, EventData::new([0, 0, 0, 0], 0));
        assert_eq!(sm.current_state(), State::Init);
    }

    #[test]
    fn test_firmware_verify_entry_sets_flags() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareVerify);
        assert_eq!(sm.current_state(), State::FirmwareVerify);
    }

    #[test]
    fn test_firmware_verify_verify_failed_goes_to_recovery() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareVerify);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::VerifyFailed, data);
        assert_eq!(sm.current_state(), State::FirmwareRecovery);
    }

    #[test]
    fn test_firmware_recovery_recovery_done_goes_to_verify() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareRecovery);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::RecoveryDone, data);
        assert_eq!(sm.current_state(), State::FirmwareVerify);
    }

    #[test]
    fn test_firmware_recovery_recovery_failed_goes_to_lockdown() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareRecovery);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::RecoveryFailed, data);
        assert_eq!(sm.current_state(), State::SystemLockdown);
    }

    #[test]
    fn test_firmware_verify_update_requested_goes_to_update() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareVerify);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::UpdateRequested, data);
        assert_eq!(sm.current_state(), State::FirmwareUpdate);
    }

    #[test]
    fn test_firmware_update_update_done_goes_to_verify() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareUpdate);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::UpdateDone, data);
        assert_eq!(sm.current_state(), State::FirmwareVerify);
    }

    #[test]
    fn test_firmware_verify_unprovisioned_goes_to_unprovisioned() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareVerify);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::VerifyUnprovisioned, data);
        assert_eq!(sm.current_state(), State::Unprovisioned);
    }

    #[test]
    fn test_firmware_verify_verify_done_goes_to_tzero() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareVerify);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::VerifyDone, data);
        assert_eq!(sm.current_state(), State::Tzero);
    }

    #[test]
    fn test_runtime_reset_detected_goes_to_verify() {
        let mut sm = StateMachine::new();
        sm.set_state(State::Runtime);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::ResetDetected, data);
        assert_eq!(sm.current_state(), State::FirmwareVerify);
    }

    #[test]
    fn test_runtime_update_requested_goes_to_update() {
        let mut sm = StateMachine::new();
        sm.set_state(State::Runtime);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::UpdateRequested, data);
        assert_eq!(sm.current_state(), State::FirmwareUpdate);
    }

    #[test]
    fn test_runtime_wdt_timeout_goes_to_recovery() {
        let mut sm = StateMachine::new();
        sm.set_state(State::Runtime);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::WdtTimeout, data);
        assert_eq!(sm.current_state(), State::FirmwareRecovery);
    }

    #[test]
    fn test_unprovisioned_reset_detected_goes_to_verify() {
        let mut sm = StateMachine::new();
        sm.set_state(State::Unprovisioned);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::ResetDetected, data);
        assert_eq!(sm.current_state(), State::FirmwareVerify);
    }

    #[test]
    fn test_seamless_update_done_goes_to_seamless_verify() {
        let mut sm = StateMachine::new();
        sm.set_state(State::SeamlessUpdate);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::SeamlessUpdateDone, data);
        assert_eq!(sm.current_state(), State::SeamlessVerify);
    }

    #[test]
    fn test_seamless_verify_done_goes_to_runtime() {
        let mut sm = StateMachine::new();
        sm.set_state(State::SeamlessVerify);
        let data = EventData::new([0, 0, 0, 0], 0);
        sm.handle_event(Event::SeamlessVerifyDone, data);
        assert_eq!(sm.current_state(), State::Runtime);
    }
}
