use std::io::Error;

const BANNED_WORDS: [&str; 1] = ["fuck"];



pub trait Verifiy {
    fn verify(&self) -> Result<String,Error>;
}

impl Verifiy for String {
    fn verify(&self) -> Result<String, Error> {
        for word in BANNED_WORDS {
            if self.contains(word) {
                let err_message = format!("Cannot contain banned word: {}", word);
                Err(err_message).unwrap()
                // instead of doing this we should probably
                // make our own error type
            }
        }
        Ok(self.as_str().to_string())
    }
}