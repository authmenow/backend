pub trait SecureGenerator {
    fn secure_words(&self, length: u8) -> String;
    fn confirmation_code(&self) -> String {
        self.secure_words(4)
    }
}
