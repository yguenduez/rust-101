pub fn power_of_2_for(number: i32) -> i32 {
    number.pow(2)
}

pub fn hello(name: String) -> String {
    format!("Hello {name}")
}

#[cfg(test)]
mod test {
    use crate::functions::{hello, power_of_2_for};

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
