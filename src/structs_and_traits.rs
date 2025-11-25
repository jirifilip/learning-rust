use std::fmt;


pub struct HelloWorldRecord {
    pub width: i32,
    pub height: i32
}


impl fmt::Display for HelloWorldRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "HelloWorldRecord --- width: {}, height: {}", self.width, self.height);
    }    
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format() {
        let hello_world_record = HelloWorldRecord{ width: 5, height: 6 };

        assert_eq!(format!("{}", hello_world_record), "HelloWorldRecord --- width: 5, height: 6");
    }
}