// Convert temperatures between Fahrenheit and Celsius.

fn main() {
    println!("Hello, world!");

    another_function(5);
    assert_eq!(convert_f_to_c(70), 21);
    assert_eq!(convert_c_to_f(70), 158);

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    if rect1.width() {
        println!("And the width is {}.", rect1.width)
    }

    let x = define_x();
    println!("{}, world", x);
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

fn define_x() -> String {
    let x = "hello".to_string();
    x
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_to_c_works() {
        assert_eq!(convert_f_to_c(70), 21);
    }

    #[test]
    fn test_c_to_f_works() {
        assert_eq!(convert_c_to_f(70), 158);
    }
  
    #[test]
    fn test_define_x() {
        assert_eq!(define_x(), "hello");
    }

}
