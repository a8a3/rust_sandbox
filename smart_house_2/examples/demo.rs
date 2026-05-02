use smart_house::{
    devices::{device::Device, socket::Socket, thermometer::Thermometer},
    house::House,
    room::Room,
};

pub fn main() {
    let devices_room_0 = vec![
        Device::Socket(Socket::new(220, true)),
        Device::Socket(Socket::new(380, false)),
    ];

    let devices_room_1 = vec![
        Device::Socket(Socket::new(220, true)),
        Device::Thermometer(Thermometer::new(10)),
    ];

    let devices_room_2 = vec![Device::Thermometer(Thermometer::new(20))];

    let rooms = vec![
        Room::new(devices_room_0),
        Room::new(devices_room_1),
        Room::new(devices_room_2),
    ];

    let mut house = House::new(rooms);
    house.print_state();

    let room_0_ref = house.get_room_mut(0);
    let device_1_ref = room_0_ref.get_device_mut(1);

    if let Device::Socket(s) = device_1_ref {
        s.turn_on();
    } else {
        panic!("expected Socket device");
    }
    house.print_state();
}
