use self::abilities::{Ability, EntersAbility};

pub mod abilities;
pub mod color;
pub mod mana_cost;
pub mod types;

#[derive(Debug, Clone)]
pub enum Card {
    Single(CardFace),
    Split(CardFace, CardFace),
    Adventure {
        primary: CardFace,
        adventure: CardFace,
    },
    Omen {
        primary: CardFace,
        omen: CardFace,
    }, // TODO: MDFCs, Transforming cards
}

impl Card {
    pub fn primary_name(&self) -> Option<&str> {
        match self {
            Self::Single(face) => Some(&face.name),
            Self::Adventure { primary, .. } => Some(&primary.name),
            Self::Omen { primary, .. } => Some(&primary.name),
            Self::Split(_, _) => None,
        }
    }

    pub fn mana_value(&self) -> u8 {
        match self {
            Self::Single(face) => face.mana_value(),
            Self::Split(a, b) => a.mana_value() + b.mana_value(),
            Self::Adventure { primary, .. } => primary.mana_value(),
            Self::Omen { primary, .. } => primary.mana_value(),
        }
    }

    pub fn is_land(&self) -> bool {
        match &self {
            Self::Single(face) => face.is_land(),
            Self::Adventure { primary, .. } => primary.is_land(),
            Self::Split(_, _) | Self::Omen { .. } => false,
        }
    }

    pub fn enters_tapped(&self) -> bool {
        match self {
            Card::Single(face) => face
                .abilities
                .iter()
                .any(|a| matches!(a, Ability::Enters(EntersAbility::Tapped))),
            Card::Adventure { primary, .. } | Card::Omen { primary, .. } => primary
                .abilities
                .iter()
                .any(|a| matches!(a, Ability::Enters(EntersAbility::Tapped))),
            Card::Split(_, _) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardFace {
    pub name: String,
    pub mana_cost: Option<mana_cost::ManaCost>,
    pub color: color::Color,
    pub is_legendary: bool,
    pub type_line: types::Type,
    pub abilities: Vec<abilities::Ability>,
}

impl CardFace {
    pub fn mana_value(&self) -> u8 {
        self.mana_cost
            .as_ref()
            .map(mana_cost::ManaCost::mana_value)
            .unwrap_or(0)
    }

    pub fn is_land(&self) -> bool {
        matches!(self.type_line, types::Type::Land(_))
    }
}
