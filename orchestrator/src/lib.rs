pub mod aspeed;

pub use aspeed::{Event, EventData, ActiveObject, StateMachine};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_orchestrator_aspeed_module_exports_event() {
        use crate::aspeed::Event;
        let _e = Event::StartStateMachine;
    }
}
