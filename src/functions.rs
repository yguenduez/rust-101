pub fn power_of_2_for(number: i32) -> i32 {
    number.pow(2)
}

mod test {
    use super::*;

    #[test]
    fn power_of_2_for_powers_a_number_by_2() {
        assert_eq!(power_of_2_for(6), 36);
    }
}
