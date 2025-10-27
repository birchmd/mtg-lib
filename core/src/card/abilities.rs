use crate::{
    card::mana_cost::{ManaCost, Unit},
    game_play::battlefield::Battlefield,
};

#[derive(Debug, Clone)]
pub enum Ability {
    Flying,
    Trample,
    Menace,
    Lifelink,
    FirstStrike,
    DoubleStrike,
    // TODO: unify triggered abilities
    Enters(EntersAbility),
    EndStep(EndStepAbility),
    Mana(ManaAbility),
    // TODO: notion of choosing targets
    Other(fn(&mut Battlefield)),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntersAbility {
    Tapped,
    Scry { amount: u8 },
    Surveil { amount: u8 },
    Discover { amount: u8 },
    GainLife { amount: u8 },
}

#[derive(Debug, Clone)]
pub enum EndStepAbility {
    // TODO: targeting (if applicable)
    Other(fn(&mut Battlefield)),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityCost {
    pub tap: bool,
    pub mana_cost: Option<ManaCost>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaAbility {
    pub cost: AbilityCost,
    pub produce: ManaProduction,
}

// TODO: produce multiple kinds of mana
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManaProduction {
    Single { possible: Vec<Unit> },
}

impl ManaProduction {
    pub fn colorless() -> Self {
        Self::Single {
            possible: vec![Unit::Colorless],
        }
    }

    pub fn white() -> Self {
        Self::Single {
            possible: vec![Unit::White],
        }
    }

    pub fn blue() -> Self {
        Self::Single {
            possible: vec![Unit::Blue],
        }
    }

    pub fn black() -> Self {
        Self::Single {
            possible: vec![Unit::Black],
        }
    }

    pub fn red() -> Self {
        Self::Single {
            possible: vec![Unit::Red],
        }
    }

    pub fn green() -> Self {
        Self::Single {
            possible: vec![Unit::Green],
        }
    }

    pub fn rakdos() -> Self {
        Self::Single {
            possible: vec![Unit::Black, Unit::Red],
        }
    }

    pub fn any_color() -> Self {
        Self::Single {
            possible: vec![Unit::White, Unit::Blue, Unit::Black, Unit::Red, Unit::Green],
        }
    }
}
