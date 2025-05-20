use super::*;

impl Request {
    pub fn is_free(&self) -> bool {
        self.key.is_free()
    }
}

impl Key {
    pub fn is_free(&self) -> bool {
        self.r#type == "free"
    }
}
