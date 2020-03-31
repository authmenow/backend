lazy_static::lazy_static! {
    static ref WORDS: Vec<&'static str> = {
        let str = include_str!("../../resources/words.txt");
        str.lines().collect()
    };
}

pub fn create_words_password(length: i8, separator: &str) -> String {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let mut words = vec![];

    for _ in 0..length {
        let pos = rng.gen_range(0, WORDS.len() - 1);
        words.push(WORDS[pos].to_owned());
    }

    words.join(separator)
}

pub fn password_hash(original_password: &str, salt: &str) -> String {
    use sha2::{Digest, Sha256};

    let string = format!("{}:{}", original_password, salt);

    format!("{:x}", Sha256::digest(string.as_bytes()))
}

mod tests {
    #[test]
    fn password_hash_is_correct_sha256() {
        assert_eq!(
            "bc705a6e854fd4d7911a032a1678a0e06150d4bb5205bb6926b3342e71264f9d",
            super::password_hash("123456789", "SALT")
        );
    }
}