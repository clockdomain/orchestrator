/// Platform firmware events triggering state transitions.
///
/// Events represent significant occurrences in the firmware lifecycle:
/// initialization, verification results, recovery outcomes, updates, and
/// runtime monitoring events. Each event may carry contextual data via
/// `EventData` for policy decisions and region selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    // Boot & Init
    /// Initialize state machine from `Boot` to `Init`
    StartStateMachine,
    /// Initialization complete; proceed to `Tmin1`
    InitDone,
    /// Secondary HROT firmware has booted; proceed to ROT recovery flow
    InitRotSecondaryBooted,

    // Verification
    /// All firmware images verified successfully; proceed to next phase
    VerifyDone,
    /// Firmware verification failed; may trigger recovery or lockdown
    VerifyFailed,
    /// Firmware images are unprovisioned; platform needs provisioning
    VerifyUnprovisioned,

    // Recovery
    /// Firmware recovery completed successfully
    RecoveryDone,
    /// Firmware recovery failed; may escalate to lockdown
    RecoveryFailed,

    // Updates
    /// Firmware update requested via intent or external command
    UpdateRequested,
    /// Firmware update completed successfully
    UpdateDone,
    /// Firmware update failed; recovery or rollback may be needed
    UpdateFailed,
    /// Secondary update requested for dual-region updates
    UpdateIntent2Requested,

    // Watchdog & Runtime
    /// Unexpected reset detected; diagnose and recover
    ResetDetected,
    /// Watchdog checkpoint reached; normal operation ongoing
    WdtCheckpoint,
    /// Watchdog timeout; platform may be hung or deadlocked
    WdtTimeout,
    /// Attestation check failed; security compromise detected
    AttestationFailed,

    // Provisioning
    /// Provisioning command received; enter provisioning flow
    ProvisionCmd,
    /// Seal firmware after provisioning; finalize security state
    SealFirmware,
    /// BMC reset command requested; initiate controlled reboot
    BmcResetCommRequested,

    // Seamless (optional)
    /// Seamless PCH update completed without blocking boot
    SeamlessUpdateDone,
    /// Seamless update failed; may require full update cycle
    SeamlessUpdateFailed,
    /// Seamless update verification passed
    SeamlessVerifyDone,
    /// Seamless update verification failed; rollback triggered
    SeamlessVerifyFailed,
}

/// Event data passed with state transitions.
///
/// Carries contextual metadata for events:
/// - `bit8[0]` — Intent flag (0=passive, 1=active update)
/// - `bit8[1]` — Region mask (selects BMC, PCH, or HROT firmware)
/// - `bit8[2]` — Reset policy (0=no reset, 1=reset on completion)
/// - `bit8[3]` — Handled region (records which region was processed)
/// - `bit32` — Extended 32-bit value for policy-specific data
///
/// # Example
///
/// ```
/// use orchestrator::EventData;
///
/// // Create event data with intent = 1 (active), region = 0x01 (BMC)
/// let data = EventData::new([1, 0x01, 0, 0], 0);
/// assert_eq!(data.intent(), 1);
/// assert_eq!(data.region_mask(), 0x01);
/// ```
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

/// Tracks firmware verification and recovery status via bit flags.
///
/// Records the verification and recovery state of firmware images during
/// the boot flow. Used to determine if recovery is needed or if the system
/// can proceed to runtime.
///
/// Flags:
/// - Bit 0: Active firmware verified
/// - Bit 1: Recovery firmware verified
/// - Bit 2: System in lockdown (fatal error)
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
