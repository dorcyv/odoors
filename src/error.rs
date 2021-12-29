#[derive(Debug, Clone)]
pub struct Error(pub String);

impl Error {
    pub fn message(&self) -> &String {
        &self.0
    }
}
