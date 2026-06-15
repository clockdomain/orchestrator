#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    // Boot & Init
    StartStateMachine,
    InitDone,
    InitRotSecondaryBooted,

    // Verification
    VerifyDone,
    VerifyFailed,
    VerifyUnprovisioned,

    // Recovery
    RecoveryDone,
    RecoveryFailed,

    // Updates
    UpdateRequested,
    UpdateDone,
    UpdateFailed,
    UpdateIntent2Requested,

    // Watchdog & Runtime
    ResetDetected,
    WdtCheckpoint,
    WdtTimeout,
    AttestationFailed,

    // Provisioning
    ProvisionCmd,
    SealFirmware,
    BmcResetCommRequested,

    // Seamless (optional)
    SeamlessUpdateDone,
    SeamlessUpdateFailed,
    SeamlessVerifyDone,
    SeamlessVerifyFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
