//! State definitions for the Aspeed PFR state machine.
//!
//! The state machine is hierarchical with top-level parent states (`Boot`, `Init`, etc.)
//! and child states (`FirmwareVerify`, `Runtime`, etc.). Transitions are event-driven
//! and may depend on verification results, recovery status, or external commands.

/// Hierarchical state of the Aspeed PFR state machine.
///
/// States represent phases of the firmware lifecycle:
///
/// **Top-Level States:**
/// - `Boot` — Initial idle state; system awaiting startup signal
/// - `Init` — System initialization and HROT setup
/// - `RotRecovery` — HROT firmware recovery flow
/// - `Tmin1` — Pre-boot verification, recovery, and update (parent)
/// - `Tzero` — Release boot holds and enter runtime (parent)
/// - `SystemReboot` — Force platform reboot
///
/// **Tmin1 Child States (pre-boot phase):**
/// - `FirmwareVerify` — Authenticate BMC, PCH, and HROT firmware images
/// - `FirmwareRecovery` — Restore corrupted active or recovery images
/// - `FirmwareUpdate` — Apply pending firmware updates
/// - `SystemLockdown` — Fatal security failure; halt boot
///
/// **Tzero Child States (runtime phase):**
/// - `Unprovisioned` — Platform lacks secure keys; provisioning needed
/// - `Runtime` — Normal operation with watchdog monitoring
/// - `SeamlessUpdate` — Non-blocking PCH firmware update during runtime
/// - `SeamlessVerify` — Verify seamless update integrity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    // Top-level states
    /// Initial idle state; system awaiting startup signal
    Boot,
    /// System initialization and HROT setup in progress
    Init,
    /// HROT firmware recovery flow active
    RotRecovery,
    /// Pre-boot verification, recovery, and update parent state
    Tmin1,
    /// Release boot holds and enter runtime parent state
    Tzero,
    /// Force platform reboot and return to `Boot`
    SystemReboot,

    // Tmin1 children
    /// Authenticate all firmware images (BMC, PCH, HROT)
    FirmwareVerify,
    /// Restore corrupted active or recovery images from backup
    FirmwareRecovery,
    /// Apply pending firmware updates to active or recovery regions
    FirmwareUpdate,
    /// Fatal security failure; halt boot and enter lockdown
    SystemLockdown,

    // Tzero children
    /// Platform is unprovisioned; secure keys need to be installed
    Unprovisioned,
    /// Normal operation with active watchdog and attestation monitoring
    Runtime,
    /// Non-blocking PCH firmware update during runtime (seamless)
    SeamlessUpdate,
    /// Verify integrity and correctness of seamless PCH update
    SeamlessVerify,
}

impl Default for State {
    fn default() -> Self {
        State::Boot
    }
}
