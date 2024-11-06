// Example for a free function
pub fn times_3(number: i32) -> i32 {
    3 * number
}

pub fn square_of(number: i32) -> i32 {
    todo!()
}

pub fn get_max(first: i64, second: i64) -> i64 {
    todo!()
}

// Those two return statements are equivalent
// the first one "hello" is reagarded idiomatic
pub fn hello(name: String) -> String {
    format!("Hello {name}")
}

pub fn welcome_to_rust(people: String) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::functions::{get_max, hello, square_of, times_3, welcome_to_rust};

    #[test]
    fn given_number_when_called_times_3_then_returns_number_timed_3() {
        let number = 42;
        assert_eq!(times_3(number), 3 * 42);
    }

    #[test]
    fn given_2_and_4_when_calling_get_max_then_return_4() {
        assert_eq!(get_max(2, 4), 4)
    }

    #[test]
    fn power_of_2_for_powers_a_number_by_2() {
        assert_eq!(square_of(6), 36);
    }

    #[test]
    fn given_max_when_calling_hello_then_return_hello_max() {
        let max = "Max".to_string();

        assert_eq!(hello(max), "Hello Max".to_string())
    }

    #[test]
    fn give_endress_folks_a_hearty_welcome() {
        let eh = "Endress".to_string();

        assert_eq!(
            welcome_to_rust(eh),
            "Welcome People of Endress and enjoy some Rust".to_string()
        )
    }
}
