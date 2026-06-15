use orchestrator::types::{Event, EventData, ActiveObject};

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

#[test]
fn test_event_data_can_store_intent() {
    let data = EventData::new([0x01, 0x02, 0x03, 0x04], 0xDEADBEEF);
    assert_eq!(data.intent(), 0x01);
    assert_eq!(data.bit32, 0xDEADBEEF);
}

#[test]
fn test_event_data_can_store_region_mask() {
    let data = EventData::new([0x10, 0x20, 0x30, 0x40], 0x12345678);
    assert_eq!(data.region_mask(), 0x20);
}

#[test]
fn test_active_object_verify_flags() {
    let mut obj = ActiveObject::default();

    assert!(!obj.active_verified());
    assert!(!obj.recovery_verified());
    assert!(!obj.in_lockdown());

    obj.set_active_verified(true);
    assert!(obj.active_verified());

    obj.set_recovery_verified(true);
    assert!(obj.recovery_verified());

    obj.set_in_lockdown(true);
    assert!(obj.in_lockdown());
}
