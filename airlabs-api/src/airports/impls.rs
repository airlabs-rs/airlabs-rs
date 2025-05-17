use super::*;

impl Airport {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_by_language(&self, language: impl AsRef<str>) -> Option<&str> {
        self.names.get(language.as_ref()).map(|name| name.as_str())
    }
}
