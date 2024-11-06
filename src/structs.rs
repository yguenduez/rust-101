/// There are not classes in Rusts, only structs (record types)
/// You can implement logic for structs in the respective
/// "impl" Blocks.

struct Person {
    age: u8,
    pub name: String,
}

impl Person {
    pub fn get_age(&self) -> u8 {
        todo!()
    }

    pub fn birthday(&mut self) {
        todo!()
    }

    // An example ;)
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::Person;

    #[test]
    fn given_a_person_when_called_get_name_then_returns_name() {
        // given
        let max = Person {
            age: 34,
            name: "Max".to_string(),
        };

        // when
        let name = max.get_name();

        // then
        assert_eq!(name, "Max");
    }

    #[test]
    fn given_a_person_when_called_get_age_returns_age() {
        // given
        let max = Person {
            age: 34,
            name: "Max".to_string(),
        };

        // when
        let age = max.get_age();

        // then
        assert_eq!(age, 34);
    }

    #[test]
    fn given_a_person_when_called_birthday_then_age_increases_by_one() {
        // given
        // Note: We can only mutate a struct, if it is also declared as
        // mutable. Rust is quite strict! Everything is read-only and private as default.
        let mut jessica = Person {
            age: 7,
            name: "Jessica".to_string(),
        };

        // when
        jessica.birthday();

        // then
        assert_eq!(jessica.age, 8)
    }
}
