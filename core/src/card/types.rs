#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Creature(CreatureProperties),
    Artifact(ArtifactProperties),
    Enchantment(EnchantmentProperties),
    ArtifactCreature {
        artifact: ArtifactProperties,
        creature: CreatureProperties,
    },
    EnchantmentCreature {
        enchantment: EnchantmentProperties,
        creature: CreatureProperties,
    },
    Land(LandProperties),
    // TODO: instant properties
    Instant,
    // TODO: sorcery properties
    Sorcery,
    // TODO: Planeswalker, Battle, Kindred
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandProperties {
    pub is_basic: bool,
    pub subtypes: Vec<LandSubtypes>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnchantmentProperties {
    pub subtypes: Vec<EnchantmentSubtypes>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactProperties {
    pub subtypes: Vec<ArtifactSubtypes>,
}

// TODO: other land subtypes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LandSubtypes {
    Plains,
    Island,
    Swamp,
    Mountain,
    Forest,
    Town,
    Cave,
    Desert,
}

// TODO: other enchantment subtypes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnchantmentSubtypes {
    Room,
}

// TODO: other artifact subtypes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactSubtypes {
    Food,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureProperties {
    pub subtypes: Vec<CreatureSubtypes>,
    pub power: Power,
    pub toughness: Toughness,
}

// TODO: other creature subtypes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureSubtypes {
    Artificer,
    Demon,
    Dinosaur,
    Dragon,
    Human,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Power {
    Value(u32),
    Dynamic { modifier: Option<Expression> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Toughness {
    Value(u32),
    Dynamic { modifier: Option<Expression> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    PlusOne,
}
