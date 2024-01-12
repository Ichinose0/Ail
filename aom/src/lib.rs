use std::collections::HashMap;

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ID(&'static str);

impl From<&'static str> for ID {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

pub trait Object {
    fn id(&self) -> ID;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {

    }
}
