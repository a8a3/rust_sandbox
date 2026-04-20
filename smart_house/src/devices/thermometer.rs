#[derive(Default)]
pub struct Thermometer {
    temp: i16,
}

impl Thermometer {
    pub fn new(temp: i16) -> Self {
        Self { temp }
    }

    pub fn get_temperature(&self) -> i16 {
        self.temp
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_thermometer() {
        let t = Thermometer::new(42);
        assert_eq!(42, t.get_temperature());
    }
    #[test]
    fn test_temperature_access() {
        let t = Thermometer::new(42);
        assert_eq!(42, t.get_temperature());
    }
} // mod tests
