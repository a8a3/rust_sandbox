use crate::devices::device::Device;

#[derive(Default)]
pub struct Room {
    devices: Vec<Device>,
}

impl Room {
    pub fn new(devices: Vec<Device>) -> Self {
        Self { devices }
    }

    pub fn get_device(&self, index: usize) -> Option<&Device> {
        let device = &self.devices.get(index)?;
        *device
    }

    pub fn get_device_mut(&mut self, index: usize) -> &mut Device {
        &mut self.devices[index]
    }

    pub fn print_state(&self) {
        println!("Room: ");
        for item in &self.devices {
            item.print_state();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::devices::{device::Device, socket::Socket, thermometer::Thermometer};

    #[test]
    #[should_panic(expected = "index out of bounds")]
    pub fn default_room_device_access_test() {
        let r = Room::default();
        r.get_device(0);
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    pub fn default_room_mut_device_access_test() {
        let mut r = Room::default();
        r.get_device_mut(0);
    }
    #[test]
    pub fn device_access_test() {
        let devices = vec![
            Device::Socket(Socket::new(220, true)),
            Device::Thermometer(Thermometer::new(23)),
        ];

        let room = Room::new(devices);
        let s_device = room.get_device(0);
        if let Device::Socket(s) = s_device {
            assert_eq!(220, s.get_wattage());
            assert_eq!(true, s.is_on());
        } else {
            panic!("expected Socket device");
        }

        let t_device = room.get_device(1);
        if let Device::Thermometer(t) = t_device {
            assert_eq!(23, t.get_temperature());
        } else {
            panic!("expected Thermometer device")
        }
    }
    #[test]
    pub fn device_mut_access_test() {
        let devices = vec![Device::Socket(Socket::new(220, true))];

        let mut room = Room::new(devices);
        let s_device = room.get_device_mut(0);
        if let Device::Socket(s) = s_device {
            s.turn_off();
            assert_eq!(false, s.is_on());
        }
    }
}
