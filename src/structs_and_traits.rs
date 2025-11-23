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