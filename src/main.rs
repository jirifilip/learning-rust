fn add(a: i32, b: i32) -> i32 {
    return a + b;
}


fn main() {
    println!("Hello, world!");
    println!("Add result: {}", add(2, 3));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 1), 3);
    }
}
