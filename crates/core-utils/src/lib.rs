/// A tiny utility, shared across apps.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Example of a function that's easy to unit-test.
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_greets() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }
    #[test]
    fn it_adds() {
        assert_eq!(add(2, 3), 5);
    }
}