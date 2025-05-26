use super::*;

impl Request {
    pub fn is_free(&self) -> bool {
        self.key().is_free()
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl Key {
    pub fn is_free(&self) -> bool {
        self.r#type == "free"
    }
}
