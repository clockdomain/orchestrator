// State definitions for the Aspeed PFR state machine

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    // Top-level states
    Boot,
    Init,
    RotRecovery,
    Tmin1,
    Tzero,
    SystemReboot,

    // Tmin1 children
    FirmwareVerify,
    FirmwareRecovery,
    FirmwareUpdate,
    SystemLockdown,

    // Tzero children
    Unprovisioned,
    Runtime,
    SeamlessUpdate,
    SeamlessVerify,
}

impl Default for State {
    fn default() -> Self {
        State::Boot
    }
}
