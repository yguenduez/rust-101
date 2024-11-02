use std::cmp::max;

pub fn power_of_2_for(number: i32) -> i32 {
    return number.pow(2)
}

pub fn get_max(first: i64, second: i64) -> i64 {
    return max(first, second);
}

pub fn hello(name: String) -> String {
    format!("Hello {name}")
}

pub fn another_hello(name: String) -> String {
    return format!("Hello {name}");
}

#[cfg(test)]
mod test {
    use crate::functions::{get_max, hello, power_of_2_for};

    #[test]
    fn given_2_and_4_when_calling_get_max_then_return_4() {
        assert_eq!(get_max(2, 4), 4)
    }

    #[test]
    fn power_of_2_for_powers_a_number_by_2() {
        assert_eq!(power_of_2_for(6), 36);
    }

    #[test]
    fn given_max_when_calling_hello_then_return_hello_max() {
        let max = "Max".to_string();

        assert_eq!(hello(max), "Hello Max".to_string())
    }
}
