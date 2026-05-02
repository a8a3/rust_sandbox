use super::socket::Socket;
use super::thermometer::Thermometer;

pub enum Device {
    Socket(Socket),
    Thermometer(Thermometer),
}

impl Device {
    pub fn print_state(&self) {
        match self {
            Device::Socket(s) => {
                println!("Socket, wattage: {}, is_on: {}", s.get_wattage(), s.is_on());
            }
            Device::Thermometer(t) => {
                println!("Thermometer, temperature: {}", t.get_temperature());
            }
        }
    }
}
