#[cfg(test)]
mod tests {
    use orchestrator::{StateMachine, Event, EventData, State};

    #[test]
    fn test_event_data_intent_preserved_during_transition() {
        let mut sm = StateMachine::new();
        let mut data = EventData::default();
        data.bit8[0] = 1; // BmcUpdateIntent

        sm.set_state(State::FirmwareUpdate);
        sm.handle_event_with_data(Event::UpdateRequested, data);

        assert_eq!(sm.last_event_data().bit8[0], 1);
    }

    #[test]
    fn test_event_data_region_mask_accessible() {
        let mut sm = StateMachine::new();
        let mut data = EventData::default();
        data.bit8[1] = 0x0F; // All regions

        sm.set_state(State::FirmwareUpdate);
        sm.handle_event_with_data(Event::UpdateRequested, data);

        assert_eq!(sm.last_event_data().region_mask(), 0x0F);
    }
}
