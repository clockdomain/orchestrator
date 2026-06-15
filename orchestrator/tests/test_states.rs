use orchestrator::aspeed::State;

#[test]
fn test_can_create_boot_state() {
    let _state = State::Boot;
}

#[test]
fn test_can_create_init_state() {
    let _state = State::Init;
}

#[test]
fn test_can_create_tmin1_state() {
    let _state = State::Tmin1;
}

#[test]
fn test_can_create_firmware_verify_state() {
    let _state = State::FirmwareVerify;
}

#[test]
fn test_can_create_all_top_level_states() {
    let _boot = State::Boot;
    let _init = State::Init;
    let _rot_recovery = State::RotRecovery;
    let _tmin1 = State::Tmin1;
    let _tzero = State::Tzero;
    let _system_reboot = State::SystemReboot;
}

#[test]
fn test_can_create_all_tmin1_child_states() {
    let _firmware_verify = State::FirmwareVerify;
    let _firmware_recovery = State::FirmwareRecovery;
    let _firmware_update = State::FirmwareUpdate;
    let _system_lockdown = State::SystemLockdown;
}

#[test]
fn test_can_create_all_tzero_child_states() {
    let _unprovisioned = State::Unprovisioned;
    let _runtime = State::Runtime;
    let _seamless_update = State::SeamlessUpdate;
    let _seamless_verify = State::SeamlessVerify;
}

#[test]
fn test_state_default_is_boot() {
    let state = State::default();
    assert_eq!(state, State::Boot);
}

#[test]
fn test_state_equality() {
    assert_eq!(State::Boot, State::Boot);
    assert_ne!(State::Boot, State::Init);
}

#[test]
fn test_state_can_be_cloned() {
    let state1 = State::Boot;
    let state2 = state1;
    assert_eq!(state1, state2);
}

#[test]
fn test_state_can_be_copied() {
    let state1 = State::Init;
    let state2 = state1;
    assert_eq!(state1, state2);
}

#[test]
fn test_state_debug_formatting() {
    let state = State::Boot;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Boot"));
}
