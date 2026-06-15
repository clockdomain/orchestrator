use orchestrator::aspeed::types::Event;

#[test]
fn test_event_enum_has_start_state_machine() {
    let event = Event::StartStateMachine;
    assert_eq!(std::mem::discriminant(&event), std::mem::discriminant(&Event::StartStateMachine));
}

#[test]
fn test_event_enum_has_init_done() {
    let _event = Event::InitDone;
}

#[test]
fn test_event_enum_has_verify_done() {
    let _event = Event::VerifyDone;
}

#[test]
fn test_event_enum_has_verify_failed() {
    let _event = Event::VerifyFailed;
}
