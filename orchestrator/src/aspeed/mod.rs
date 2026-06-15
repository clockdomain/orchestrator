//! Aspeed PFR (Platform Firmware Resilience) Hierarchical State Machine
//!
//! This module implements a complete hierarchical state machine for managing
//! BMC, PCH, and HROT firmware verification, recovery, and update operations
//! on ASPEED BMC SoCs.
//!
//! ## State Hierarchy
//!
//! **Top-level states:**
//! - `Boot` — Initial idle state
//! - `Init` — System initialization
//! - `RotRecovery` — HROT firmware recovery
//! - `Tmin1` — Pre-boot verification/recovery/update
//! - `Tzero` — Release boot holds, enter runtime
//! - `SystemReboot` — Force platform reboot
//!
//! **Tmin1 children:**
//! - `FirmwareVerify` — Authenticate all firmware images
//! - `FirmwareRecovery` — Recover corrupted images
//! - `FirmwareUpdate` — Perform firmware updates
//! - `SystemLockdown` — Security lockdown on fatal failures
//!
//! **Tzero children:**
//! - `Unprovisioned` — Unprovisioned platform state
//! - `Runtime` — Normal operation (monitoring active)
//! - `SeamlessUpdate` — Non-blocking PCH update
//! - `SeamlessVerify` — Verify seamless update
//!
//! ## Usage
//!
//! ```ignore
//! use orchestrator::aspeed::{StateMachine, Event, EventData};
//!
//! let mut sm = StateMachine::new();
//! let mut data = EventData::new([1, 0, 0, 0], 0); // intent = 1
//!
//! sm.handle_event_with_data(Event::StartStateMachine, data);
//! ```
//!
//! ## Event-Driven Architecture
//!
//! Events drive state transitions and carry contextual data via `EventData`:
//! - Boot & Init: `StartStateMachine`, `InitDone`, `InitRotSecondaryBooted`
//! - Verification: `VerifyDone`, `VerifyFailed`, `VerifyUnprovisioned`
//! - Recovery: `RecoveryDone`, `RecoveryFailed`
//! - Updates: `UpdateRequested`, `UpdateDone`, `UpdateFailed`, `UpdateIntent2Requested`
//! - Watchdog & Runtime: `ResetDetected`, `WdtCheckpoint`, `WdtTimeout`, `AttestationFailed`
//! - Provisioning: `ProvisionCmd`, `SealFirmware`, `BmcResetCommRequested`
//! - Seamless: `SeamlessUpdateDone`, `SeamlessUpdateFailed`, `SeamlessVerifyDone`, `SeamlessVerifyFailed`

pub mod types;
pub mod states;
pub mod handlers;
pub mod context;

pub use types::{Event, EventData, ActiveObject};
pub use states::State;
pub use context::StateMachine;
