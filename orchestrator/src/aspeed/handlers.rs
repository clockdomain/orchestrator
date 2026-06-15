use crate::aspeed::types::EventData;

pub trait StateHandler {
    fn on_entry(&mut self, data: &EventData);
    fn on_exit(&mut self, data: &EventData);
    fn on_run(&mut self, data: &EventData);
}

pub struct BootHandler;
impl StateHandler for BootHandler {
    fn on_entry(&mut self, _data: &EventData) {}
    fn on_exit(&mut self, _data: &EventData) {}
    fn on_run(&mut self, _data: &EventData) {}
}

pub struct InitHandler;
impl StateHandler for InitHandler {
    fn on_entry(&mut self, _data: &EventData) {
        // Initialize engines, logging, etc.
    }
    fn on_exit(&mut self, _data: &EventData) {}
    fn on_run(&mut self, _data: &EventData) {}
}

pub struct FirmwareVerifyHandler;
impl StateHandler for FirmwareVerifyHandler {
    fn on_entry(&mut self, _data: &EventData) {
        // Set up verification parameters
    }
    fn on_exit(&mut self, _data: &EventData) {}
    fn on_run(&mut self, _data: &EventData) {
        // Perform verification
    }
}

// HANDLER RESPONSIBILITY MAPPING
//
// This section documents the entry/run/exit responsibilities for each state.
// Handlers are called when entering/exiting states and during the run loop.
//
// BOOT: No-op state, entry/exit do nothing
//
// INIT: Entry initializes engines, exit clears state
//   - on_entry: Initialize crypto engines, manifest processor, logging, SPI monitors
//   - on_run: (not used in BOOT→INIT transition)
//   - on_exit: Clean up initialization state
//
// FIRMWARE_VERIFY: Entry prepares verification engine, run executes verify
//   - on_entry: Set up verification parameters, cache manifests
//   - on_run: Execute image verification (BMC, PCH, AFM, CPLD)
//   - on_exit: Save verification results
//
// FIRMWARE_RECOVERY: Entry sets state, run executes recovery
//   - on_entry: Initialize recovery parameters from UFM
//   - on_run: Execute recovery of failed regions (active↔recovery copy)
//   - on_exit: Clear recovery state
//
// FIRMWARE_UPDATE: Entry initializes update engine, run executes update
//   - on_entry: Decode update intent from event data, validate signatures
//   - on_run: Execute firmware updates for each region
//   - on_exit: Clear update buffers
//
// TZERO: Entry releases boot holds, exit disarms reset monitor
//   - on_entry: Switch SPI MUX ownership, release resets, arm watchdogs
//   - on_run: (not used, children handle run)
//   - on_exit: Disarm reset monitor
//
// RUNTIME: Entry arms watchdogs, run monitors, exit stops attester
//   - on_entry: Arm watchdog timers for checkpoint recovery
//   - on_run: Monitor checkpoints, process provisioning commands, handle updates
//   - on_exit: Stop SPDM attester, disarm watchdogs
//
// UNPROVISIONED: Entry disables protection, run handles provisioning
//   - on_entry: Bypass SPI/I2C filters (full read-write)
//   - on_run: Handle provisioning commands, accept images from Cerberus
//   - on_exit: Re-enable SPI filters
//
// SYSTEM_LOCKDOWN: Entry holds all resets, run is terminal (no-op)
//   - on_entry: Hold BMC and PCH resets, log fatal error
//   - on_run: Do nothing (wait for external intervention)
//   - on_exit: (never called)
//
// SYSTEM_REBOOT: Entry logs panic, triggers platform reboot
//   - on_entry: LOG_PANIC(), call pfr_cpld_update_reboot()
//   - on_run: (not reached)
//   - on_exit: (not reached, platform resets)
