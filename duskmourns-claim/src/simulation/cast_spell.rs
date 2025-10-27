use {
    super::CLAIM_NAME,
    mtg_lib_core::{
        card::{
            Card,
            abilities::{Ability, ManaProduction},
            mana_cost::{ManaCost, ManaCostRef, Pip, Unit},
        },
        game_play::{
            OwnedCard,
            battlefield::{Battlefield, Event, InPlayObject},
        },
    },
    std::cmp::Ordering,
};

const APPRAISER_NAME: &str = "Geological Appraiser";
const CARNOSAUR_NAME: &str = "Trumpeting Carnosaur";
const CEASE_NAME: &str = "Cease";
const SAUNA_NAME: &str = "Steaming Sauna";
const YARD_NAME: &str = "Shattered Yard";

const CLAIM_COST: &[Pip] = &[
    Pip::Single(Unit::Generic { amount: 2 }),
    Pip::Single(Unit::Black),
];

const APPRAISER_COST: &[Pip] = &[
    Pip::Single(Unit::Generic { amount: 2 }),
    Pip::Single(Unit::Red),
    Pip::Single(Unit::Red),
];

const CARNOSAUR_COST: &[Pip] = &[
    Pip::Single(Unit::Generic { amount: 4 }),
    Pip::Single(Unit::Red),
    Pip::Single(Unit::Red),
];

const CEASE_COST: &[Pip] = &[
    Pip::Single(Unit::Generic { amount: 1 }),
    Pip::Hybrid(Unit::Black, Unit::Green),
];

const SAUNA_COST: &[Pip] = &[
    Pip::Single(Unit::Generic { amount: 3 }),
    Pip::Single(Unit::Blue),
    Pip::Single(Unit::Blue),
];

const YARD_COST: &[Pip] = &[
    Pip::Single(Unit::Generic { amount: 4 }),
    Pip::Single(Unit::Red),
];

pub fn cast_spells(battlefield: &mut Battlefield) {
    // Spells priority:
    // 1. Duskmourn's Claim
    // 2. Geological Appraiser (finds claim)
    // 3. Trumpeting Carnosaur (finds claim)
    // 4. Cease (draws a card)
    // 5. Steaming Sauna (draws cards)
    // 6. Shattered Yard (incidental damage)

    let spells = [
        (Finder::Single(CLAIM_NAME), CLAIM_COST),
        (Finder::Single(APPRAISER_NAME), APPRAISER_COST),
        (Finder::Single(CARNOSAUR_NAME), CARNOSAUR_COST),
        (Finder::Split(CEASE_NAME), CEASE_COST),
        (Finder::Split(SAUNA_NAME), SAUNA_COST),
        (Finder::Split(YARD_NAME), YARD_COST),
    ];

    for (finder, cost) in spells {
        while let Some(card) = finder.find(battlefield) {
            if auto_tapper(battlefield, cost.into()) {
                battlefield.cast_spell(card);
            } else {
                break;
            }
        }
    }
}

enum Finder {
    Single(&'static str),
    Split(&'static str),
}

impl Finder {
    fn find(&self, battlefield: &mut Battlefield) -> Option<OwnedCard> {
        match self {
            Self::Single(name) => find_by_name(battlefield, name),
            Self::Split(name) => find_split_card(battlefield, name),
        }
    }
}

fn find_by_name(battlefield: &mut Battlefield, name: &str) -> Option<OwnedCard> {
    let hand = &mut battlefield.players.first_mut()?.zones.hand;
    let index = hand
        .iter()
        .position(|c| c.card.primary_name() == Some(name))?;
    let card = hand.remove(index);
    Some(card)
}

fn find_split_card(battlefield: &mut Battlefield, name: &str) -> Option<OwnedCard> {
    let hand = &mut battlefield.players.first_mut()?.zones.hand;
    let index = hand.iter().position(|c| {
        if let Card::Split(left, right) = &c.card {
            (left.name == name) || (right.name == name)
        } else {
            false
        }
    })?;
    let card = hand.remove(index);
    Some(card)
}

// Returns `true` if the spell is cast.
// In that case the lands to cast it are tapped.
// Otherwise there is no change to the battlefield.
fn auto_tapper(battlefield: &mut Battlefield, mana_cost: ManaCostRef) -> bool {
    let mut mana_base: Vec<MaybeTap> = battlefield
        .objects
        .iter_mut()
        .filter(|o| !o.tapped && o.card.card.is_land())
        .map(MaybeTap::new)
        .collect();

    // If there not enough lands we definitely cannot cast it.
    if mana_base.len() < mana_cost.mana_value() as usize {
        return false;
    }

    // Tap for colored mana first. Prefer free sources.
    let colored_pips = mana_cost
        .pips
        .iter()
        .filter(|p| !matches!(p, Pip::Single(Unit::Generic { .. })));
    let mut extra_cost: u8 = 0;
    for p in colored_pips {
        let source = mana_base
            .iter_mut()
            .filter_map(|o| {
                if o.should_tap {
                    None
                } else if let MaybeProduce::ProducesFor { cost } = produces_color(o.inner, p) {
                    Some((o, cost))
                } else {
                    None
                }
            })
            .min_by_key(|(_, cost)| cost.as_ref().map(|c| c.mana_value()).unwrap_or(0));
        let Some((source, cost)) = source else {
            return false;
        };
        source.should_tap = true;
        if let Some(cost) = cost {
            // In this deck all filtering lands cost 1 generic mana
            debug_assert_eq!(&cost.pips, &[Pip::Single(Unit::Generic { amount: 1 })]);
            extra_cost += 1;
        }
    }

    let total_generic_cost: u8 = mana_cost
        .pips
        .iter()
        .filter_map(|p| {
            if let Pip::Single(Unit::Generic { amount }) = p {
                Some(*amount)
            } else {
                None
            }
        })
        .sum::<u8>()
        + extra_cost;
    let paid_generic_mana: u8 = mana_base
        .iter_mut()
        .filter(|o| !o.should_tap)
        .take(total_generic_cost.into())
        .fold(0, |acc, o| {
            o.should_tap = true;
            acc + 1
        });

    // Failed to tap enough mana to pay for spell.
    if paid_generic_mana != total_generic_cost {
        return false;
    }

    // Tap lands that were used to cast the spell
    for land in mana_base {
        if land.should_tap {
            land.inner.tapped = true;
            battlefield.log.push(Event::Tap(land.inner.clone()));
        }
    }

    true
}

fn produces_color(object: &InPlayObject, pip: &Pip) -> MaybeProduce {
    // TODO: how to handle MDFC lands.
    let Card::Single(face) = &object.card.card else {
        return MaybeProduce::CannotProduce;
    };
    let color = match pip {
        Pip::Single(unit) => unit,
        // TODO: this is an oversimplification.
        // The only hybrid symbol we care about is on Cease and we
        // only care about the black half so this is fine for now.
        Pip::Hybrid(_, _) => &Unit::Black,
    };
    face.abilities
        .iter()
        .filter_map(|a| {
            if let Ability::Mana(ability) = a {
                match &ability.produce {
                    ManaProduction::Single { possible } if possible.contains(color) => {
                        Some(ability.cost.mana_cost.clone())
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .min_by(|x, y| match (x, y) {
            (None, None) => Ordering::Equal,
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (Some(x), Some(y)) => x.mana_value().cmp(&y.mana_value()),
        })
        .map_or(MaybeProduce::CannotProduce, |cost| {
            MaybeProduce::ProducesFor { cost }
        })
}

enum MaybeProduce {
    CannotProduce,
    ProducesFor { cost: Option<ManaCost> },
}

struct MaybeTap<'a> {
    should_tap: bool,
    inner: &'a mut InPlayObject,
}

impl<'a> MaybeTap<'a> {
    fn new(inner: &'a mut InPlayObject) -> Self {
        Self {
            should_tap: false,
            inner,
        }
    }
}

#[test]
fn test_auto_tapper() {
    let mut battlefield = crate::simulation::initialize();
    let lands: Vec<OwnedCard> = battlefield
        .players
        .first()
        .unwrap()
        .zones
        .library
        .iter()
        .filter(|c| c.card.is_land())
        .cloned()
        .collect();

    let mana_cost = ManaCost {
        pips: vec![
            Pip::Single(Unit::Generic { amount: 2 }),
            Pip::Single(Unit::Black),
        ],
    };
    let mana_value = mana_cost.mana_value();

    // If there are no black sources in play then we cannot cast a card with cost {2}{B}
    let mountain = lands
        .iter()
        .find(|c| c.card.primary_name() == Some("Mountain"))
        .unwrap()
        .clone();
    for _ in 0..mana_value {
        battlefield.play_land(mountain.clone());
    }
    assert!(
        !auto_tapper(&mut battlefield, mana_cost.as_ref()),
        "Cannot cast"
    );
    assert!(
        battlefield.objects.iter().all(|o| !o.tapped),
        "Nothing is tapped if nothing is cast"
    );
    battlefield.objects.clear();

    // If there are black sources, but not enough total mana then nothing is cast
    let swamp = lands
        .iter()
        .find(|c| c.card.primary_name() == Some("Swamp"))
        .unwrap()
        .clone();
    for _ in 1..mana_value {
        battlefield.play_land(swamp.clone());
    }
    assert!(
        !auto_tapper(&mut battlefield, mana_cost.as_ref()),
        "Cannot cast"
    );
    assert!(
        battlefield.objects.iter().all(|o| !o.tapped),
        "Nothing is tapped if nothing is cast"
    );
    battlefield.objects.clear();

    // If there are black sources, but they cost extra mana then still cannot cast
    let grotto = lands
        .iter()
        .find(|c| c.card.primary_name() == Some("Crystal Grotto"))
        .unwrap()
        .clone();
    for _ in 0..mana_value {
        battlefield.play_land(grotto.clone());
    }
    assert!(
        !auto_tapper(&mut battlefield, mana_cost.as_ref()),
        "Cannot cast"
    );
    assert!(
        battlefield.objects.iter().all(|o| !o.tapped),
        "Nothing is tapped if nothing is cast"
    );
    battlefield.objects.clear();

    // With enough swamps we can cast the spell
    for _ in 0..mana_value {
        battlefield.play_land(swamp.clone());
    }
    assert!(
        auto_tapper(&mut battlefield, mana_cost.as_ref()),
        "Can cast"
    );
    assert!(
        battlefield.objects.iter().all(|o| o.tapped),
        "All lands are tapped to cast"
    );
    battlefield.objects.clear();

    // With one swamp and two grottos we can cast the spell
    battlefield.play_land(swamp.clone());
    for _ in 1..mana_value {
        battlefield.play_land(grotto.clone());
    }
    assert!(
        auto_tapper(&mut battlefield, mana_cost.as_ref()),
        "Can cast"
    );
    assert!(
        battlefield.objects.iter().all(|o| o.tapped),
        "All lands are tapped to cast"
    );
    battlefield.objects.clear();

    // With only grottos we can cast the spell, but we need an additional land
    for _ in 0..(mana_value + 1) {
        battlefield.play_land(grotto.clone());
    }
    assert!(
        auto_tapper(&mut battlefield, mana_cost.as_ref()),
        "Can cast"
    );
    assert!(
        battlefield.objects.iter().all(|o| o.tapped),
        "All lands are tapped to cast"
    );
    battlefield.objects.clear();

    // If we have more than enough lands then some remain untapped
    battlefield.play_land(swamp);
    for _ in 0..mana_value {
        battlefield.play_land(mountain.clone());
    }
    assert!(
        auto_tapper(&mut battlefield, mana_cost.as_ref()),
        "Can cast"
    );
    assert!(
        battlefield
            .objects
            .iter()
            .take(mana_value.into())
            .all(|o| o.tapped),
        "All lands except one tapped to cast"
    );
    assert!(
        !battlefield.objects.last().unwrap().tapped,
        "Last land remains untapped"
    );
}
