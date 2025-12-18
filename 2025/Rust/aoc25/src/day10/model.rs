use std::fmt::Debug;

pub type IntButton = u64;
pub type PartTwoTarget = Vec<u64>;

pub type InputLine = (IntButton, Vec<Button>, PartTwoTarget);

#[derive(Eq, Hash, PartialEq)]
pub struct Button {
    raw: Vec<u64>,
    int: IntButton,
}

impl Button {
    pub fn new(raw: Vec<u64>) -> Self {
        Self {
            int: raw.iter().map(|i| 2_u64.pow(*i as u32)).sum(),
            raw,
        }
    }

    pub fn raw(&self) -> &[u64] {
        &self.raw
    }

    pub fn to_int(&self) -> IntButton {
        self.int
    }
}

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Btn<{:?}>", self.raw)
    }
}
