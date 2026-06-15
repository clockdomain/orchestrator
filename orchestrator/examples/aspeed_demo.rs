//! Example: Complete Aspeed PFR state machine workflow
//!
//! Demonstrates boot sequence, verification, recovery, and update flows.

use orchestrator::aspeed::{StateMachine, Event, EventData, State};

fn main() {
    println!("=== Aspeed PFR State Machine Demo ===\n");

    // Scenario 1: Normal boot to runtime
    println!("Scenario 1: Normal Boot to Runtime");
    let mut sm = StateMachine::new();
    println!("  Initial state: {:?}", sm.current_state());

    sm.handle_event(Event::StartStateMachine, EventData::default());
    println!("  After StartStateMachine: {:?}", sm.current_state());

    sm.handle_event(Event::InitDone, EventData::default());
    println!("  After InitDone: {:?}", sm.current_state());

    sm.handle_event(Event::VerifyDone, EventData::default());
    println!("  After VerifyDone: {:?}", sm.current_state());

    println!();

    // Scenario 2: Verify failure and recovery
    println!("Scenario 2: Verify Failure and Recovery");
    let mut sm = StateMachine::new();
    sm.set_state(State::FirmwareVerify);
    println!("  State: {:?}", sm.current_state());

    sm.handle_event(Event::VerifyFailed, EventData::default());
    println!("  After VerifyFailed: {:?}", sm.current_state());

    sm.handle_event(Event::RecoveryDone, EventData::default());
    println!("  After RecoveryDone: {:?}", sm.current_state());

    println!();

    // Scenario 3: Firmware update
    println!("Scenario 3: Firmware Update from Runtime");
    let mut sm = StateMachine::new();
    sm.set_state(State::Runtime);
    println!("  State: {:?}", sm.current_state());

    let data = EventData::new([1, 0x03, 0, 0], 0); // intent = 1, region = 0x03 (BMC active + recovery)

    sm.handle_event_with_data(Event::UpdateRequested, data);
    println!("  After UpdateRequested: {:?}", sm.current_state());
    println!("  Event intent: {}", sm.last_event_data().intent());
    println!("  Event region mask: {:#04x}", sm.last_event_data().region_mask());

    sm.handle_event(Event::UpdateDone, EventData::default());
    println!("  After UpdateDone: {:?}", sm.current_state());

    println!();
    println!("Demo complete!");
}
