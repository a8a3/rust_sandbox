use crate::room::Room;

#[derive(Default)]
pub struct House {
    rooms: Vec<Room>,
}

impl House {
    pub fn new(rooms: Vec<Room>) -> Self {
        House { rooms }
    }

    pub fn get_room(&self, index: usize) -> &Room {
        &self.rooms[index]
    }

    pub fn get_room_mut(&mut self, index: usize) -> &mut Room {
        &mut self.rooms[index]
    }

    pub fn print_state(&self) {
        println!("House:");
        for item in &self.rooms {
            item.print_state();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        devices::device::Device, devices::socket::Socket, devices::thermometer::Thermometer,
        room::Room,
    };

    #[test]
    pub fn room_access_test() {
        let room_a_devices = vec![
            Device::Socket(Socket::new(220, true)),
            Device::Thermometer(Thermometer::new(20)),
        ];

        let room_b_devices = vec![
            Device::Socket(Socket::new(380, true)),
            Device::Socket(Socket::new(220, true)),
        ];

        let rooms = vec![Room::new(room_a_devices), Room::new(room_b_devices)];

        let house = House::new(rooms);
        let room_a = house.get_room(0);
        let socket_in_room_a = room_a.get_device(0);

        if let Device::Socket(s) = socket_in_room_a {
            assert_eq!(220, s.get_wattage());
            assert_eq!(true, s.is_on());
        } else {
            panic!("expected Socket device");
        }

        let room_b = house.get_room(1);
        let socket_in_room_b = room_b.get_device(0);

        if let Device::Socket(s) = socket_in_room_b {
            assert_eq!(380, s.get_wattage());
            assert_eq!(true, s.is_on());
        } else {
            panic!("expected Socket device");
        }
    }

    #[test]
    pub fn room_mut_access_test() {
        let devices = vec![Device::Socket(Socket::new(220, true))];

        let rooms = vec![Room::new(devices)];

        let mut house = House::new(rooms);
        let room = house.get_room_mut(0);
        let socket = room.get_device_mut(0);

        if let Device::Socket(s) = socket {
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
