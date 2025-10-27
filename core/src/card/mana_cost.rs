#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaCost {
    pub pips: Vec<Pip>,
}

impl ManaCost {
    pub fn mana_value(&self) -> u8 {
        self.pips.iter().map(|p| p.mana_value()).sum()
    }

    pub fn as_ref(&self) -> ManaCostRef<'_> {
        ManaCostRef { pips: &self.pips }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaCostRef<'a> {
    pub pips: &'a [Pip],
}

impl<'a> ManaCostRef<'a> {
    pub fn mana_value(&self) -> u8 {
        self.pips.iter().map(|p| p.mana_value()).sum()
    }
}

impl<'a> From<&'a [Pip]> for ManaCostRef<'a> {
    fn from(pips: &'a [Pip]) -> Self {
        Self { pips }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pip {
    Single(Unit),
    Hybrid(Unit, Unit),
}

impl Pip {
    pub fn mana_value(&self) -> u8 {
        match self {
            Self::Single(unit) => unit.mana_value(),
            Self::Hybrid(a, b) => core::cmp::max(a.mana_value(), b.mana_value()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unit {
    Generic { amount: u8 },
    X,
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

impl Unit {
    pub fn mana_value(&self) -> u8 {
        match self {
            Self::Generic { amount } => *amount,
            Self::X => 0,
            Self::White | Self::Blue | Self::Black | Self::Red | Self::Green | Self::Colorless => 1,
        }
    }
}
