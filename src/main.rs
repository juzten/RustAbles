// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
    println!("Hello, world!");

    another_function(5);
    assert_eq!(convert_f_to_c(70), 21);
    assert_eq!(convert_c_to_f(70), 158);
}

fn another_function(x: i32) {
    println!("another function: {}", x);
}

fn convert_f_to_c(temperature: i32) -> i32 {
    ((temperature - 32) as f64 * 0.5556) as i32
}

fn convert_c_to_f(temperature: i32) -> i32 {
    (temperature as f64 * 1.8) as i32 + 32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_to_c_works() {
        assert_eq!(convert_f_to_c(70), 21);
    }

    #[test]
    fn teset_c_to_f_works() {
        assert_eq!(convert_c_to_f(70), 158)
    }
}
