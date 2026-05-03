use std::collections::HashMap;

use smart_house::{
    devices::{device::Device, socket::Socket, thermometer::Thermometer},
    house::House,
    room::Room,
};

pub fn main() {
    let mut devices_room_0: HashMap<String, Device> = HashMap::new();
    devices_room_0.insert(
        "socket_220".to_string(),
        Device::Socket(Socket::new(220, true)),
    );
    devices_room_0.insert(
        "socket_380".to_string(),
        Device::Socket(Socket::new(380, false)),
    );

    let mut devices_room_1: HashMap<String, Device> = HashMap::new();
    devices_room_1.insert("socket".to_string(), Device::Socket(Socket::new(220, true)));
    devices_room_1.insert(
        "thermometer".to_string(),
        Device::Thermometer(Thermometer::new(10)),
    );

    let mut devices_room_2: HashMap<String, Device> = HashMap::new();
    devices_room_2.insert(
        "thermometer".to_string(),
        Device::Thermometer(Thermometer::new(20)),
    );

    let mut rooms: HashMap<String, Room> = HashMap::new();
    rooms.insert("room_0".to_string(), Room::new(devices_room_0));
    rooms.insert("room_1".to_string(), Room::new(devices_room_1));
    rooms.insert("room_2".to_string(), Room::new(devices_room_2));

    let mut house = House::new(rooms);
    house.print_state();

    let maybe_room_0_ref = house.get_room_mut("room_0");
    assert!(maybe_room_0_ref.is_some());

    let maybe_device_1_ref = maybe_room_0_ref.unwrap().get_device_mut("socket_220");

    if let Device::Socket(s) = maybe_device_1_ref.unwrap() {
        s.turn_on();
    } else {
        panic!("expected Socket device");
    }
    house.print_state();
}
