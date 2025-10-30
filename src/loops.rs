// As you probably know loops already from other languages,
// we just show you, how you can loop in different ways

/// Looping with a range
fn loop_over_range() -> i32 {
    let values = vec![4, 5, 3, 6, 3]; // 5 entries
    let mut sum = 0;
    for i in 1..=3 {
        sum += values[i]
    }

    sum
}

/// Here we loop in a procedural way over a collection
fn sum_over_vector(vector: &[i32]) -> i32 {
    let mut sum = 0;
    for value in vector {
        sum += value
    }

    sum
}

/// Here we use the fold combinator to reduce a collection to one value
fn sum_over_vector_functional(vector: &[i32]) -> i32 {
    vector.iter().fold(0, |acc, x| acc + x)
}

/// This is your task: concatinate the incoming words
fn concat_words_functional(words: &[&str]) -> String {
    words.iter().fold(String::new(), |mut string, x| {
        string.push_str(x);
        string
    })
}

#[cfg(test)]
mod test {
    use crate::loops::{concat_words_functional, loop_over_range, sum_over_vector, sum_over_vector_functional};

    #[test]
    fn loop_over_range_works(){
        assert_eq!(loop_over_range(), 14)
    }

    #[test]
    fn sum_over_vector_is_correct() {
        let vec = vec![4, 4, 16, 3, 9];
        assert_eq!(sum_over_vector(&vec), 36);
        assert_eq!(sum_over_vector_functional(&vec), 36);
    }

    #[test]
    fn concat_words_works() {
        let vec = vec!["This", "Is", "Awsome"];
        assert_eq!(concat_words_functional(&vec), String::from("ThisIsAwsome"));
    }
}
