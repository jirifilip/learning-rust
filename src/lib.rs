/// This function is doing some adding.
///
/// Next line that should maybe be here?
///
/// ```
/// let result = learning_rust::add(1, 2);
/// assert_eq!(result, 3);
/// ```
pub mod structs_and_traits;


pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 1), 3);
    }
}