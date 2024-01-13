use std::collections::HashMap;

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ID(&'static str);

impl From<&'static str> for ID {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl Into<String> for ID {
    fn into(self) -> String {
        self.0.to_owned()
    }
}

pub trait Object {
    fn id(&self) -> ID;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {}
}
