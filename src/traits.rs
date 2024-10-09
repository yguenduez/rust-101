/// Traits are interfaces on stereoids.
/// With those you can have static and dynamic polymorphism

trait MakesSound {
    fn make_sound(&self) -> String;
}

/// Implementing a trait is also done in an impl Block,
/// that should look quite familiar

/// A struct without members
struct Cat;

/// This is how you implement a trait for your type.
impl MakesSound for Cat {
    fn make_sound(&self) -> String {
        "Miau".to_string()
    }
}

struct Dog;
impl MakesSound for Dog {
    fn make_sound(&self) -> String {
        todo!()
    }
}

struct ItalianPerson;

/// Then - how can you use those polymorphic bevahiours?
/// static polymorphism:
fn things_that_make_sound<T: MakesSound>(list_of_things: Vec<T>) -> Vec<String> {
    // We iterator over a list_of_things and map every element to a string, as make_sound() returns
    // a string. We then collect every string back into a vector
    list_of_things
        .iter()
        .map(|thing| thing.make_sound())
        .collect()
}

/// dynamic polymorphism:
fn things_that_make_sound_dynamic(list_of_things: Vec<Box<dyn MakesSound>>) -> Vec<String> {
    list_of_things
        .iter()
        .map(|thing| thing.make_sound())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::traits::{things_that_make_sound_dynamic, Cat, Dog, ItalianPerson, MakesSound};

    use super::things_that_make_sound;

    #[test]
    fn a_cat_makes_a_miau_sound() {
        let cat = Cat;
        assert_eq!(cat.make_sound(), "Miau".to_string());
    }

    #[test]
    fn a_dog_makes_woof_sound() {
        let dog = Dog;
        assert_eq!(dog.make_sound(), "Woof".to_string());
    }

    // This will not compile, as you did not implement MakeSound for an ItalianPerson ;). You can
    // try youself.
    //    #[test]
    //    fn an_italian_shouts_mammamia() {
    //        let person = ItalianPerson;
    //        assert_eq!(person.make_sound(), "MammaMia".to_string());
    //    }

    // Correct the list animals
    #[test]
    fn a_zoo_of_animals_make_funny_sounds() {
        // given
        let animals: Vec<Box<dyn MakesSound>> = vec![Box::new(Cat), Box::new(Dog)];

        // when
        let sounds = things_that_make_sound_dynamic(animals);

        // then
        assert_eq!(
            sounds,
            vec![
                "Miau".to_string(),
                "Woof".to_string(),
                "Woof".to_string(),
                "Miau".to_string()
            ]
        );
    }
}
