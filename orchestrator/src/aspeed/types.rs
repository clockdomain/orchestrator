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

/// EventData carries metadata for events: intent, region mask, reset policy, and handled region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventData {
    pub bit8: [u8; 4],
    pub bit32: u32,
}

impl EventData {
    /// Create a new EventData with the given bit8 array and bit32 value.
    pub fn new(bit8: [u8; 4], bit32: u32) -> Self {
        EventData { bit8, bit32 }
    }

    /// Extract intent from bit8[0].
    pub fn intent(&self) -> u8 {
        self.bit8[0]
    }

    /// Extract region mask from bit8[1].
    pub fn region_mask(&self) -> u8 {
        self.bit8[1]
    }

    /// Extract reset policy from bit8[2].
    pub fn reset_policy(&self) -> u8 {
        self.bit8[2]
    }

    /// Extract handled region from bit8[3].
    pub fn handled_region(&self) -> u8 {
        self.bit8[3]
    }
}

impl Default for EventData {
    fn default() -> Self {
        EventData {
            bit8: [0; 4],
            bit32: 0,
        }
    }
}

/// ActiveObject tracks firmware verification and recovery status via bit flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveObject {
    flags: u16,
}

impl ActiveObject {
    /// Bit flag: active firmware verified
    const ACTIVE_VERIFIED: u16 = 1 << 0;
    /// Bit flag: recovery firmware verified
    const RECOVERY_VERIFIED: u16 = 1 << 1;
    /// Bit flag: in lockdown state
    const IN_LOCKDOWN: u16 = 1 << 2;

    /// Set the active verified flag.
    pub fn set_active_verified(&mut self, verified: bool) {
        if verified {
            self.flags |= Self::ACTIVE_VERIFIED;
        } else {
            self.flags &= !Self::ACTIVE_VERIFIED;
        }
    }

    /// Check if active firmware is verified.
    pub fn active_verified(&self) -> bool {
        (self.flags & Self::ACTIVE_VERIFIED) != 0
    }

    /// Set the recovery verified flag.
    pub fn set_recovery_verified(&mut self, verified: bool) {
        if verified {
            self.flags |= Self::RECOVERY_VERIFIED;
        } else {
            self.flags &= !Self::RECOVERY_VERIFIED;
        }
    }

    /// Check if recovery firmware is verified.
    pub fn recovery_verified(&self) -> bool {
        (self.flags & Self::RECOVERY_VERIFIED) != 0
    }

    /// Set the in-lockdown flag.
    pub fn set_in_lockdown(&mut self, lockdown: bool) {
        if lockdown {
            self.flags |= Self::IN_LOCKDOWN;
        } else {
            self.flags &= !Self::IN_LOCKDOWN;
        }
    }

    /// Check if in lockdown state.
    pub fn in_lockdown(&self) -> bool {
        (self.flags & Self::IN_LOCKDOWN) != 0
    }
}

impl Default for ActiveObject {
    fn default() -> Self {
        ActiveObject { flags: 0 }
    }
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
