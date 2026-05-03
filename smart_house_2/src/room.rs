use crate::devices::device::Device;
use std::collections::HashMap;

#[derive(Default)]
pub struct Room {
    devices: HashMap<String, Device>,
}

impl Room {
    pub fn new(devices: HashMap<String, Device>) -> Self {
        Self { devices }
    }

    pub fn get_device(&self, key: &str) -> Option<&Device> {
        self.devices.get(key)
    }

    pub fn get_device_mut(&mut self, key: &str) -> Option<&mut Device> {
        self.devices.get_mut(key)
    }

    pub fn print_state(&self) {
        println!("Room: ");
        for device in self.devices.values() {
            device.print_state();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::devices::{device::Device, socket::Socket, thermometer::Thermometer};

    #[test]
    pub fn default_room_device_access_test() {
        let r = Room::default();
        let maybe_device = r.get_device("any");
        assert!(maybe_device.is_none());
    }
    #[test]
    pub fn default_room_mut_device_access_test() {
        let mut r = Room::default();
        let maybe_device = r.get_device_mut("any");
        assert!(maybe_device.is_none());
    }
    #[test]
    pub fn device_access_test() {
        let mut devices: HashMap<String, Device> = HashMap::new();
        devices.insert("socket".to_string(), Device::Socket(Socket::new(220, true)));
        devices.insert(
            "thermometer".to_string(),
            Device::Thermometer(Thermometer::new(23)),
        );

        let room = Room::new(devices);
        let s_maybe_device = room.get_device("socket");
        assert!(s_maybe_device.is_some());

        if let Device::Socket(s) = s_maybe_device.unwrap() {
            assert_eq!(220, s.get_wattage());
            assert_eq!(true, s.is_on());
        } else {
            panic!("expected Socket device");
        }

        let t_maybe_device = room.get_device("thermometer");
        assert!(t_maybe_device.is_some());

        if let Device::Thermometer(t) = t_maybe_device.unwrap() {
            assert_eq!(23, t.get_temperature());
        } else {
            panic!("expected Thermometer device")
        }
    }
    #[test]
    pub fn device_mut_access_test() {
        let mut devices: HashMap<String, Device> = HashMap::new();
        devices.insert("socket".to_string(), Device::Socket(Socket::new(220, true)));

        let mut room = Room::new(devices);
        let s_maybe_device = room.get_device_mut("socket");
        assert!(s_maybe_device.is_some());

        if let Device::Socket(s) = s_maybe_device.unwrap() {
            s.turn_off();
            assert_eq!(false, s.is_on());
        }
    }
}
