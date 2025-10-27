//! Here is the deck list:
//! ```ignore
//! Deck
//! 4 Cease // Desist (MKM) 246
//! 3 Mountain (SLD) 1471
//! 4 Twinmaw Stormbrood (TDM) 232
//! 4 Virtue of Persistence (WOE) 115
//! 4 Roaring Furnace // Steaming Sauna (DSK) 230
//! 1 Swamp (SLD) 1470
//! 4 Trumpeting Carnosaur (LCI) 171
//! 2 Unholy Annex // Ritual Chamber (DSK) 118
//! 4 Glassworks // Shattered Yard (DSK) 137
//! 4 Duskmourn's Claim (OM1) 55
//! 4 Geological Appraiser (LCI) 150
//! 4 Conduit Pylons (OTJ) 254
//! 3 Crystal Grotto (WOE) 254
//! 4 Hidden Grotto (BLB) 254
//! 4 Temple of Malice (FDN) 701
//! 4 Raucous Theater (MKM) 266
//! 3 Blazemire Verge (DSK) 256
//! ```

use mtg_lib_core::{
    card::{
        Card, CardFace,
        abilities::{
            Ability, AbilityCost, EndStepAbility, EntersAbility, ManaAbility, ManaProduction,
        },
        color::Color,
        mana_cost::{ManaCost, Pip, Unit},
        types::{
            CreatureProperties, CreatureSubtypes, EnchantmentProperties, EnchantmentSubtypes,
            LandProperties, LandSubtypes, Power, Toughness, Type,
        },
    },
    game_play::battlefield::{Battlefield, Event},
};

pub fn deck() -> Vec<Card> {
    let cease = CardFace {
        name: "Cease".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 1 }),
                Pip::Hybrid(Unit::Black, Unit::Green),
            ],
        }),
        color: Color::golgari(),
        is_legendary: false,
        type_line: Type::Instant,
        abilities: vec![Ability::Other(cease_ability)],
    };
    let desist = CardFace {
        name: "Desist".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 4 }),
                Pip::Hybrid(Unit::Green, Unit::White),
                Pip::Hybrid(Unit::Green, Unit::White),
            ],
        }),
        color: Color::selesnya(),
        is_legendary: false,
        type_line: Type::Sorcery,
        abilities: Vec::new(), // TODO
    };
    let cease_desist = Card::Split(cease, desist);

    let twinmaw_stormbrood = CardFace {
        name: "Twinmaw Stormbrood".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 5 }),
                Pip::Single(Unit::White),
            ],
        }),
        color: Color::white(),
        is_legendary: false,
        type_line: Type::Creature(CreatureProperties {
            subtypes: vec![CreatureSubtypes::Dragon],
            power: Power::Value(5),
            toughness: Toughness::Value(4),
        }),
        abilities: vec![
            Ability::Flying,
            Ability::Enters(EntersAbility::GainLife { amount: 5 }),
        ],
    };
    let charring_bite = CardFace {
        name: "Charring Bite".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 1 }),
                Pip::Single(Unit::Red),
            ],
        }),
        type_line: Type::Sorcery,
        color: Color::red(),
        is_legendary: false,
        abilities: Vec::new(), // TODO
    };
    let stormbrood = Card::Omen {
        primary: twinmaw_stormbrood,
        omen: charring_bite,
    };

    let roaring_furnace = CardFace {
        name: "Roaring Furnace".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 1 }),
                Pip::Single(Unit::Red),
            ],
        }),
        type_line: Type::Enchantment(EnchantmentProperties {
            subtypes: vec![EnchantmentSubtypes::Room],
        }),
        color: Color::red(),
        is_legendary: false,
        abilities: Vec::new(), // TODO
    };
    let steaming_sauna = CardFace {
        name: "Steaming Sauna".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 3 }),
                Pip::Single(Unit::Blue),
                Pip::Single(Unit::Blue),
            ],
        }),
        type_line: Type::Enchantment(EnchantmentProperties {
            subtypes: vec![EnchantmentSubtypes::Room],
        }),
        color: Color::blue(),
        is_legendary: false,
        abilities: vec![
            // TODO: no max hand size
            Ability::EndStep(EndStepAbility::Other(steaming_sauna_ability)),
        ],
    };
    let furnace_sauna = Card::Split(roaring_furnace, steaming_sauna);

    let virtue_of_persistence = CardFace {
        name: "Virtue of Persistence".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 5 }),
                Pip::Single(Unit::Black),
                Pip::Single(Unit::Black),
            ],
        }),
        type_line: Type::Enchantment(EnchantmentProperties {
            subtypes: Vec::new(),
        }),
        color: Color::black(),
        is_legendary: false,
        abilities: Vec::new(), // TODO
    };
    let locthwain_scorn = CardFace {
        name: "Locthwain Scorn".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 1 }),
                Pip::Single(Unit::Black),
            ],
        }),
        type_line: Type::Sorcery,
        color: Color::black(),
        is_legendary: false,
        abilities: Vec::new(), // TODO
    };
    let virtue = Card::Adventure {
        primary: virtue_of_persistence,
        adventure: locthwain_scorn,
    };

    let carnosaur = Card::Single(CardFace {
        name: "Trumpeting Carnosaur".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 4 }),
                Pip::Single(Unit::Red),
                Pip::Single(Unit::Red),
            ],
        }),
        color: Color::red(),
        is_legendary: false,
        type_line: Type::Creature(CreatureProperties {
            subtypes: vec![CreatureSubtypes::Dinosaur],
            power: Power::Value(7),
            toughness: Toughness::Value(6),
        }),
        abilities: vec![
            Ability::Trample,
            Ability::Enters(EntersAbility::Discover { amount: 5 }),
            // TODO: discard burn ability
        ],
    });

    let glassworks = CardFace {
        name: "Glassworks".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 2 }),
                Pip::Single(Unit::Red),
            ],
        }),
        type_line: Type::Enchantment(EnchantmentProperties {
            subtypes: vec![EnchantmentSubtypes::Room],
        }),
        color: Color::red(),
        is_legendary: false,
        abilities: Vec::new(), // TODO
    };
    let shattered_yard = CardFace {
        name: "Shattered Yard".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 4 }),
                Pip::Single(Unit::Red),
            ],
        }),
        type_line: Type::Enchantment(EnchantmentProperties {
            subtypes: vec![EnchantmentSubtypes::Room],
        }),
        color: Color::red(),
        is_legendary: false,
        abilities: vec![Ability::EndStep(EndStepAbility::Other(
            shattered_yard_ability,
        ))],
    };
    let works_yard = Card::Split(glassworks, shattered_yard);

    let appraiser = Card::Single(CardFace {
        name: "Geological Appraiser".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 2 }),
                Pip::Single(Unit::Red),
                Pip::Single(Unit::Red),
            ],
        }),
        color: Color::red(),
        is_legendary: false,
        type_line: Type::Creature(CreatureProperties {
            subtypes: vec![CreatureSubtypes::Human, CreatureSubtypes::Artificer],
            power: Power::Value(3),
            toughness: Toughness::Value(2),
        }),
        abilities: vec![
            // TODO: "if you cast it" restriction.
            Ability::Enters(EntersAbility::Discover { amount: 3 }),
        ],
    });

    let unholy_annex = CardFace {
        name: "Unholy Annex".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 2 }),
                Pip::Single(Unit::Black),
            ],
        }),
        type_line: Type::Enchantment(EnchantmentProperties {
            subtypes: vec![EnchantmentSubtypes::Room],
        }),
        color: Color::black(),
        is_legendary: false,
        abilities: Vec::new(), // TODO
    };
    let ritual_chamber = CardFace {
        name: "Ritual Chamber".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 3 }),
                Pip::Single(Unit::Black),
                Pip::Single(Unit::Black),
            ],
        }),
        type_line: Type::Enchantment(EnchantmentProperties {
            subtypes: vec![EnchantmentSubtypes::Room],
        }),
        color: Color::black(),
        is_legendary: false,
        abilities: Vec::new(), // TODO
    };
    let annex_chamber = Card::Split(unholy_annex, ritual_chamber);

    let duskmourns_claim = Card::Single(CardFace {
        name: "Duskmourn's Claim".into(),
        mana_cost: Some(ManaCost {
            pips: vec![
                Pip::Single(Unit::Generic { amount: 2 }),
                Pip::Single(Unit::Black),
            ],
        }),
        color: Color::black(),
        is_legendary: false,
        type_line: Type::Enchantment(EnchantmentProperties {
            subtypes: Vec::new(),
        }),
        abilities: vec![Ability::EndStep(EndStepAbility::Other(
            duskmourns_claim_ability,
        ))],
    });

    let mountain = Card::Single(CardFace {
        name: "Mountain".into(),
        mana_cost: None,
        color: Color::colorless(),
        is_legendary: false,
        type_line: Type::Land(LandProperties {
            is_basic: true,
            subtypes: vec![LandSubtypes::Mountain],
        }),
        abilities: vec![Ability::Mana(ManaAbility {
            cost: AbilityCost {
                tap: true,
                mana_cost: None,
            },
            produce: ManaProduction::red(),
        })],
    });

    let swamp = Card::Single(CardFace {
        name: "Swamp".into(),
        mana_cost: None,
        color: Color::colorless(),
        is_legendary: false,
        type_line: Type::Land(LandProperties {
            is_basic: true,
            subtypes: vec![LandSubtypes::Swamp],
        }),
        abilities: vec![Ability::Mana(ManaAbility {
            cost: AbilityCost {
                tap: true,
                mana_cost: None,
            },
            produce: ManaProduction::black(),
        })],
    });

    let conduit_pylons = Card::Single(CardFace {
        name: "Conduit Pylons".into(),
        mana_cost: None,
        color: Color::colorless(),
        is_legendary: false,
        type_line: Type::Land(LandProperties {
            is_basic: false,
            subtypes: vec![LandSubtypes::Desert],
        }),
        abilities: vec![
            Ability::Enters(EntersAbility::Surveil { amount: 1 }),
            Ability::Mana(ManaAbility {
                cost: AbilityCost {
                    tap: true,
                    mana_cost: None,
                },
                produce: ManaProduction::colorless(),
            }),
            Ability::Mana(ManaAbility {
                cost: AbilityCost {
                    tap: true,
                    mana_cost: Some(ManaCost {
                        pips: vec![Pip::Single(Unit::Generic { amount: 1 })],
                    }),
                },
                produce: ManaProduction::any_color(),
            }),
        ],
    });

    let crystal_grotto = Card::Single(CardFace {
        name: "Crystal Grotto".into(),
        mana_cost: None,
        color: Color::colorless(),
        is_legendary: false,
        type_line: Type::Land(LandProperties {
            is_basic: false,
            subtypes: Vec::new(),
        }),
        abilities: vec![
            Ability::Enters(EntersAbility::Scry { amount: 1 }),
            Ability::Mana(ManaAbility {
                cost: AbilityCost {
                    tap: true,
                    mana_cost: None,
                },
                produce: ManaProduction::colorless(),
            }),
            Ability::Mana(ManaAbility {
                cost: AbilityCost {
                    tap: true,
                    mana_cost: Some(ManaCost {
                        pips: vec![Pip::Single(Unit::Generic { amount: 1 })],
                    }),
                },
                produce: ManaProduction::any_color(),
            }),
        ],
    });

    let hidden_grotto = Card::Single(CardFace {
        name: "Hidden Grotto".into(),
        mana_cost: None,
        color: Color::colorless(),
        is_legendary: false,
        type_line: Type::Land(LandProperties {
            is_basic: false,
            subtypes: Vec::new(),
        }),
        abilities: vec![
            Ability::Enters(EntersAbility::Surveil { amount: 1 }),
            Ability::Mana(ManaAbility {
                cost: AbilityCost {
                    tap: true,
                    mana_cost: None,
                },
                produce: ManaProduction::colorless(),
            }),
            Ability::Mana(ManaAbility {
                cost: AbilityCost {
                    tap: true,
                    mana_cost: Some(ManaCost {
                        pips: vec![Pip::Single(Unit::Generic { amount: 1 })],
                    }),
                },
                produce: ManaProduction::any_color(),
            }),
        ],
    });

    let temple_of_malice = Card::Single(CardFace {
        name: "Temple of Malice".into(),
        mana_cost: None,
        color: Color::colorless(),
        is_legendary: false,
        type_line: Type::Land(LandProperties {
            is_basic: false,
            subtypes: Vec::new(),
        }),
        abilities: vec![
            Ability::Enters(EntersAbility::Tapped),
            Ability::Enters(EntersAbility::Scry { amount: 1 }),
            Ability::Mana(ManaAbility {
                cost: AbilityCost {
                    tap: true,
                    mana_cost: None,
                },
                produce: ManaProduction::rakdos(),
            }),
        ],
    });

    let raucous_theater = Card::Single(CardFace {
        name: "Raucous Theater".into(),
        mana_cost: None,
        color: Color::colorless(),
        is_legendary: false,
        type_line: Type::Land(LandProperties {
            is_basic: false,
            subtypes: vec![LandSubtypes::Mountain, LandSubtypes::Swamp],
        }),
        abilities: vec![
            Ability::Enters(EntersAbility::Tapped),
            Ability::Enters(EntersAbility::Surveil { amount: 1 }),
            Ability::Mana(ManaAbility {
                cost: AbilityCost {
                    tap: true,
                    mana_cost: None,
                },
                produce: ManaProduction::rakdos(),
            }),
        ],
    });

    let blazemire_verge = Card::Single(CardFace {
        name: "Blazemire Verge".into(),
        mana_cost: None,
        color: Color::colorless(),
        is_legendary: false,
        type_line: Type::Land(LandProperties {
            is_basic: false,
            subtypes: Vec::new(),
        }),
        abilities: vec![
            Ability::Mana(ManaAbility {
                cost: AbilityCost {
                    tap: true,
                    mana_cost: None,
                },
                produce: ManaProduction::black(),
            }),
            Ability::Mana(ManaAbility {
                // TODO: this ability has the restriction
                // "Activate only if you control a Swamp or a Mountain."
                cost: AbilityCost {
                    tap: true,
                    mana_cost: None,
                },
                produce: ManaProduction::red(),
            }),
        ],
    });

    vec![
        cease_desist.clone(),
        cease_desist.clone(),
        cease_desist.clone(),
        cease_desist,
        stormbrood.clone(),
        stormbrood.clone(),
        stormbrood.clone(),
        stormbrood,
        furnace_sauna.clone(),
        furnace_sauna.clone(),
        furnace_sauna.clone(),
        furnace_sauna,
        virtue.clone(),
        virtue.clone(),
        virtue.clone(),
        virtue,
        carnosaur.clone(),
        carnosaur.clone(),
        carnosaur.clone(),
        carnosaur,
        works_yard.clone(),
        works_yard.clone(),
        works_yard.clone(),
        works_yard,
        appraiser.clone(),
        appraiser.clone(),
        appraiser.clone(),
        appraiser,
        annex_chamber.clone(),
        annex_chamber,
        duskmourns_claim.clone(),
        duskmourns_claim.clone(),
        duskmourns_claim.clone(),
        duskmourns_claim,
        mountain.clone(),
        mountain.clone(),
        mountain,
        swamp,
        conduit_pylons.clone(),
        conduit_pylons.clone(),
        conduit_pylons.clone(),
        conduit_pylons,
        crystal_grotto.clone(),
        crystal_grotto.clone(),
        crystal_grotto,
        hidden_grotto.clone(),
        hidden_grotto.clone(),
        hidden_grotto.clone(),
        hidden_grotto,
        temple_of_malice.clone(),
        temple_of_malice.clone(),
        temple_of_malice.clone(),
        temple_of_malice,
        raucous_theater.clone(),
        raucous_theater.clone(),
        raucous_theater.clone(),
        raucous_theater,
        blazemire_verge.clone(),
        blazemire_verge.clone(),
        blazemire_verge,
    ]
}

// TODO: can also exile up to two cards from a single graveyard.
// TODO: should be target player; need to figure out targeting.
fn cease_ability(battlefield: &mut Battlefield) {
    let player = battlefield.players.first_mut().unwrap();

    player.life_total += 2;
    if player.draw_a_card().is_ok() {
        battlefield.log.push(Event::Draw(
            player.zones.hand.last().expect("Just drew card").clone(),
        ));
    }
}

// TODO: should be target players
fn duskmourns_claim_ability(battlefield: &mut Battlefield) {
    let mut players = battlefield.players.iter_mut();
    let player1 = players.next().unwrap();
    let player2 = players.next().unwrap();

    let card1 = player1.zones.library.pop_front();
    let card2 = player2.zones.library.pop_front();

    if let Some(card) = card1 {
        let life_lost = card.card.mana_value() as i32;
        battlefield.log.push(Event::LostLife(player2.id, life_lost));
        player2.life_total -= life_lost;
        player1.zones.hand.push(card);
    }

    if let Some(card) = card2 {
        let life_lost = card.card.mana_value() as i32;
        player1.life_total -= life_lost;
        battlefield.log.push(Event::LostLife(player1.id, life_lost));
        player2.zones.hand.push(card);
    }
}

// TODO: proper notion of opponent
// TODO: notion of damage as opposed to life loss
fn shattered_yard_ability(battlefield: &mut Battlefield) {
    let opponents = battlefield.players.iter_mut().skip(1);

    for player in opponents {
        player.life_total -= 1;
        battlefield.log.push(Event::LostLife(player.id, 1));
    }
}

// TODO: notion of "you"
fn steaming_sauna_ability(battlefield: &mut Battlefield) {
    let player = battlefield.players.first_mut().unwrap();
    if player.draw_a_card().is_ok() {
        battlefield.log.push(Event::Draw(
            player.zones.hand.last().expect("Just drew card").clone(),
        ));
    }
}

#[test]
fn test_deck() {
    assert_eq!(deck().len(), 60);
}
