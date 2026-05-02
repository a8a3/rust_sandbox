#[derive(Default)]
pub struct Socket {
    wattage: i16,
    is_on: bool,
}

impl Socket {
    pub fn new(wattage: i16, is_on: bool) -> Self {
        Self { wattage, is_on }
    }

    pub fn get_wattage(&self) -> i16 {
        if self.is_on() { self.wattage } else { 0 }
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }
} // impl Socket

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_default_values() {
        let s = Socket::default();
        assert_eq!(0, s.get_wattage());
        assert!(!s.is_on());
    }
    #[test]
    fn test_new_socket() {
        let s = Socket::new(220, false);
        assert_eq!(220, s.wattage);
        assert_eq!(false, s.is_on);
    }
    #[test]
    fn test_on_off() {
        let mut s = Socket::new(380, false);
        assert!(!s.is_on());
        s.turn_on();
        assert!(s.is_on());
        s.turn_off();
        assert!(!s.is_on());
    }
} // mod tests
