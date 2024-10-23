// No real programming language comes without branching

fn get_bigger(first: i32, second: i32) -> i32 {
    todo!()
}

fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        todo!()
    } else {
        todo!()
    }
}

fn another_picky_eater(food: &str) -> &str {
    match food {
        "strawberry" => todo!(),
        "orange" => todo!(),
        // This one "matches" any other expression
        _ => "No thank you!",
    }
}

#[cfg(test)]
mod tests {
    use crate::branching::{another_picky_eater, picky_eater};

    use super::get_bigger;

    #[test]
    fn given_two_numbers_when_calling_get_bigger_returns_the_bigger_one() {
        // given
        let number_a = 2;
        let number_b = 99;

        // when
        let result = get_bigger(number_a, number_b);

        // then
        assert_eq!(result, 99);
    }

    #[test]
    fn given_different_foods_when_calling_picky_eater_then_return_feelings() {
        assert_eq!(picky_eater("strawberry"), "Mhhh. Yummi");
        assert_eq!(picky_eater("apple"), "No thank you!");
        assert_eq!(picky_eater("banana"), "No thank you!");
        assert_eq!(picky_eater("orange"), "Mhhh. Please more!");
    }

    #[test]
    fn given_different_foods_when_calling_another_picky_eater_then_return_feelings() {
        assert_eq!(another_picky_eater("strawberry"), "Mhhh. Yummi");
        assert_eq!(another_picky_eater("apple"), "No thank you!");
        assert_eq!(another_picky_eater("banana"), "No thank you!");
        assert_eq!(another_picky_eater("orange"), "Mhhh. Please more!");
    }
}
