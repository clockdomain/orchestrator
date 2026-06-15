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
