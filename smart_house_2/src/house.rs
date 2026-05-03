use crate::room::Room;
use std::collections::HashMap;

#[derive(Default)]
pub struct House {
    rooms: HashMap<String, Room>,
}

impl House {
    pub fn new(rooms: HashMap<String, Room>) -> Self {
        House { rooms }
    }

    pub fn get_room(&self, key: &str) -> Option<&Room> {
        self.rooms.get(key)
    }

    pub fn get_room_mut(&mut self, key: &str) -> Option<&mut Room> {
        self.rooms.get_mut(key)
    }

    pub fn print_state(&self) {
        println!("House:");
        for item in self.rooms.values() {
            item.print_state();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        devices::device::Device, devices::socket::Socket, devices::thermometer::Thermometer,
        room::Room,
    };

    #[test]
    pub fn room_access_test() {
        let mut room_a_devices: HashMap<String, Device> = HashMap::new();
        room_a_devices.insert("socket".to_string(), Device::Socket(Socket::new(220, true)));
        room_a_devices.insert(
            "thermometer".to_string(),
            Device::Thermometer(Thermometer::new(20)),
        );

        let mut room_b_devices: HashMap<String, Device> = HashMap::new();
        room_b_devices.insert(
            "socket_380".to_string(),
            Device::Socket(Socket::new(380, true)),
        );
        room_b_devices.insert(
            "socket_220".to_string(),
            Device::Socket(Socket::new(220, true)),
        );

        let mut rooms: HashMap<String, Room> = HashMap::new();
        rooms.insert("room_a".to_string(), Room::new(room_a_devices));
        rooms.insert("room_b".to_string(), Room::new(room_b_devices));

        let house = House::new(rooms);
        let maybe_room_a = house.get_room("room_a");
        assert!(maybe_room_a.is_some());

        let maybe_socket_in_room_a = maybe_room_a.unwrap().get_device("socket");
        assert!(maybe_socket_in_room_a.is_some());

        if let Device::Socket(s) = maybe_socket_in_room_a.unwrap() {
            assert_eq!(220, s.get_wattage());
            assert_eq!(true, s.is_on());
        } else {
            panic!("expected Socket device");
        }

        let maybe_room_b = house.get_room("room_b");
        let maybe_socket_in_room_b = maybe_room_b.unwrap().get_device("socket_380");
        assert!(maybe_socket_in_room_b.is_some());

        if let Device::Socket(s) = maybe_socket_in_room_b.unwrap() {
            assert_eq!(380, s.get_wattage());
            assert_eq!(true, s.is_on());
        } else {
            panic!("expected Socket device");
        }
    }

    #[test]
    pub fn room_mut_access_test() {
        let mut devices: HashMap<String, Device> = HashMap::new();
        devices.insert("socket".to_string(), Device::Socket(Socket::new(220, true)));

        let mut rooms: HashMap<String, Room> = HashMap::new();
        rooms.insert("room".to_string(), Room::new(devices));

        let mut house = House::new(rooms);
        let maybe_room = house.get_room_mut("room");
        assert!(maybe_room.is_some());

        let maybe_socket = maybe_room.unwrap().get_device_mut("socket");
        assert!(maybe_socket.is_some());

        if let Device::Socket(s) = maybe_socket.unwrap() {
            assert_eq!(220, s.get_wattage());
            assert_eq!(true, s.is_on());

            s.turn_off();

            assert_eq!(0, s.get_wattage());
            assert_eq!(false, s.is_on());
        } else {
            panic!("expected Socket device");
        }
    }
}
