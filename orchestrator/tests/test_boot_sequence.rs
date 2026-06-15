#[cfg(test)]
mod tests {
    use orchestrator::aspeed::{StateMachine, Event, EventData, State};

    #[test]
    fn test_full_boot_sequence_unprovisioned() {
        let mut sm = StateMachine::new();
        assert_eq!(sm.current_state(), State::Boot);
        sm.handle_event(Event::StartStateMachine, EventData::default());
        assert_eq!(sm.current_state(), State::Init);
        sm.handle_event(Event::InitDone, EventData::default());
        assert_eq!(sm.current_state(), State::FirmwareVerify);
        sm.handle_event(Event::VerifyUnprovisioned, EventData::default());
        assert_eq!(sm.current_state(), State::Unprovisioned);
    }

    #[test]
    fn test_full_boot_sequence_to_runtime() {
        let mut sm = StateMachine::new();
        sm.handle_event(Event::StartStateMachine, EventData::default());
        sm.handle_event(Event::InitDone, EventData::default());
        sm.handle_event(Event::VerifyDone, EventData::default());
        assert_eq!(sm.current_state(), State::Tzero);
        sm.set_state(State::Runtime);
        assert_eq!(sm.current_state(), State::Runtime);
    }

    #[test]
    fn test_recovery_from_verify_failure() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareVerify);
        sm.handle_event(Event::VerifyFailed, EventData::default());
        assert_eq!(sm.current_state(), State::FirmwareRecovery);
        sm.handle_event(Event::RecoveryDone, EventData::default());
        assert_eq!(sm.current_state(), State::FirmwareVerify);
        sm.handle_event(Event::VerifyDone, EventData::default());
        assert_eq!(sm.current_state(), State::Tzero);
    }

    #[test]
    fn test_lockdown_on_recovery_failure() {
        let mut sm = StateMachine::new();
        sm.set_state(State::FirmwareVerify);
        sm.handle_event(Event::VerifyFailed, EventData::default());
        assert_eq!(sm.current_state(), State::FirmwareRecovery);
        sm.handle_event(Event::RecoveryFailed, EventData::default());
        assert_eq!(sm.current_state(), State::SystemLockdown);
    }

    #[test]
    fn test_runtime_update_sequence() {
        let mut sm = StateMachine::new();
        sm.set_state(State::Runtime);
        let mut data = EventData::default();
        data.bit8[0] = 1; // BmcUpdateIntent
        data.bit8[1] = 0x01; // BmcActiveUpdate
        sm.handle_event_with_data(Event::UpdateRequested, data);
        assert_eq!(sm.current_state(), State::FirmwareUpdate);
        sm.handle_event(Event::UpdateDone, EventData::default());
        assert_eq!(sm.current_state(), State::FirmwareVerify);
        sm.handle_event(Event::VerifyDone, EventData::default());
        assert_eq!(sm.current_state(), State::Tzero);
    }

    #[test]
    fn test_seamless_update_sequence() {
        let mut sm = StateMachine::new();
        sm.set_state(State::Runtime);
        sm.handle_event(Event::UpdateIntent2Requested, EventData::default());
        assert_eq!(sm.current_state(), State::SeamlessUpdate);
        sm.handle_event(Event::SeamlessUpdateDone, EventData::default());
        assert_eq!(sm.current_state(), State::SeamlessVerify);
        sm.handle_event(Event::SeamlessVerifyDone, EventData::default());
        assert_eq!(sm.current_state(), State::Runtime);
    }
}
