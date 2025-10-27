use {
    super::CLAIM_NAME,
    mtg_lib_core::{
        card::{
            Card,
            abilities::{Ability, EntersAbility, ManaProduction},
            mana_cost::Unit,
            types::{LandSubtypes, Type},
        },
        game_play::{
            OwnedCard,
            battlefield::{Battlefield, Event},
            player::PlayerState,
        },
    },
};

type CardProperty = fn(&OwnedCard) -> bool;

pub fn play_a_land(battlefield: &mut Battlefield) {
    // On turn 1 there are no other lands in play
    let is_turn_1 = !battlefield.objects.iter().any(|o| o.card.card.is_land());

    let claim_in_play = battlefield
        .objects
        .iter()
        .any(|o| o.card.card.primary_name() == Some(CLAIM_NAME));

    let player = battlefield.players.first_mut().unwrap();
    let Some(card_to_play) = select_land_to_play(player, is_turn_1, claim_in_play) else {
        return;
    };

    if let Some(etb) = battlefield.play_land(card_to_play) {
        match etb {
            EntersAbility::Scry { amount } => {
                // TODO: support multiple cards for scry
                debug_assert_eq!(amount, 1);
                let library = &mut battlefield.players.first_mut().unwrap().zones.library;
                let Some(card) = library.pop_front() else {
                    return;
                };
                if decide_to_bottom(claim_in_play, &card.card) {
                    battlefield.log.push(Event::ScryBottom(card.clone()));
                    library.push_back(card);
                } else {
                    battlefield.log.push(Event::ScryTop(card.clone()));
                    library.push_front(card);
                }
            }
            EntersAbility::Surveil { amount } => {
                // TODO: support multiple cards for surveil
                debug_assert_eq!(amount, 1);
                let player = battlefield.players.first_mut().unwrap();
                let Some(card) = player.zones.library.pop_front() else {
                    return;
                };
                if decide_to_bottom(claim_in_play, &card.card) {
                    battlefield.log.push(Event::SurveilYard(card.clone()));
                    player.zones.graveyard.push(card);
                } else {
                    battlefield.log.push(Event::SurveilTop(card.clone()));
                    player.zones.library.push_front(card);
                }
            }
            _ => (),
        }
    }
}

fn decide_to_bottom(claim_in_play: bool, card: &Card) -> bool {
    // - Top lands if Claim not in play, bottom otherwise
    // - Top big spells if Claim is in play, bottom otherwise
    // - Always top Claim

    let is_land = card.is_land();
    let is_big_spell = !is_land && (card.mana_value() > 5);
    let is_claim = !is_land && !is_big_spell && (card.primary_name() == Some(CLAIM_NAME));

    let should_top = (is_land && !claim_in_play) || (is_big_spell && claim_in_play) || is_claim;
    !should_top
}

fn select_land_to_play(
    player: &mut PlayerState,
    is_turn_1: bool,
    claim_in_play: bool,
) -> Option<OwnedCard> {
    let mut lands: Vec<&OwnedCard> = player
        .zones
        .hand
        .iter()
        .filter(|c| c.card.is_land())
        .collect();

    let preference: (CardProperty, CardProperty, CardProperty) = if is_turn_1 {
        // Prefer tapped lands on turn 1
        (enters_tapped, produces_black, has_scry_etb)
    } else if claim_in_play {
        // prefer scry/surveil lands if Claim is in play
        (has_scry_etb, enters_untapped, produces_black)
    } else {
        // prefer untapped black sources otherwise
        (produces_black, enters_untapped, has_scry_etb)
    };

    lands.sort_unstable_by_key(|c| (!(preference.0)(c), !(preference.1)(c), !(preference.2)(c)));

    let name_to_play = lands.first().and_then(|c| c.card.primary_name())?;
    let index_to_play = player
        .zones
        .hand
        .iter()
        .position(|c| c.card.primary_name() == Some(name_to_play))
        .expect("Card is known to be in hand");
    Some(player.zones.hand.swap_remove(index_to_play))
}

fn enters_tapped(card: &OwnedCard) -> bool {
    card.card.enters_tapped()
}

fn enters_untapped(card: &OwnedCard) -> bool {
    !card.card.enters_tapped()
}

fn has_scry_etb(card: &OwnedCard) -> bool {
    match &card.card {
        Card::Single(face) => face.abilities.iter().any(|a| {
            matches!(
                a,
                Ability::Enters(EntersAbility::Scry { .. })
                    | Ability::Enters(EntersAbility::Surveil { .. })
            )
        }),
        Card::Adventure { primary, .. } => primary.abilities.iter().any(|a| {
            matches!(
                a,
                Ability::Enters(EntersAbility::Scry { .. })
                    | Ability::Enters(EntersAbility::Surveil { .. })
            )
        }),
        _ => false,
    }
}

fn produces_black(card: &OwnedCard) -> bool {
    let face = match &card.card {
        Card::Single(face) => face,
        Card::Adventure { primary, .. } => primary,
        _ => {
            return false;
        }
    };
    let Type::Land(properties) = &face.type_line else {
        return false;
    };
    if properties.subtypes.contains(&LandSubtypes::Swamp) {
        return true;
    }
    face.abilities.iter().any(|a| {
        let Ability::Mana(ability) = a else {
            return false;
        };
        if ability.cost.mana_cost.is_some() {
            return false;
        }
        matches!(&ability.produce, ManaProduction::Single { possible } if possible.contains(&Unit::Black))
    })
}

#[test]
fn test_select_land() {
    let deck = crate::deck::deck();

    // Untapped, produces black
    let swamp = deck
        .iter()
        .find(|c| c.primary_name() == Some("Swamp"))
        .cloned()
        .unwrap();
    // Produces black, scrys
    let theatre = deck
        .iter()
        .find(|c| c.primary_name() == Some("Raucous Theater"))
        .cloned()
        .unwrap();
    // Untapped, scrys
    let grotto = deck
        .iter()
        .find(|c| c.primary_name() == Some("Crystal Grotto"))
        .cloned()
        .unwrap();

    let mut battlefield = super::initialize();
    let player = battlefield.players.first_mut().unwrap();
    let id = player.id;
    let hand = &mut player.zones.hand;
    hand.push(OwnedCard {
        card: swamp.clone(),
        owner: id,
    });
    hand.push(OwnedCard {
        card: theatre.clone(),
        owner: id,
    });
    hand.push(OwnedCard {
        card: grotto.clone(),
        owner: id,
    });

    // On turn 1 we choose theater
    assert_eq!(
        select_land_to_play(player, true, false)
            .unwrap()
            .card
            .primary_name(),
        theatre.primary_name()
    );
    player.zones.hand.push(OwnedCard {
        card: theatre.clone(),
        owner: id,
    });

    // Before claim is in play we prefer an untapped black source
    assert_eq!(
        select_land_to_play(player, false, false)
            .unwrap()
            .card
            .primary_name(),
        swamp.primary_name()
    );
    player.zones.hand.push(OwnedCard {
        card: swamp.clone(),
        owner: id,
    });

    // After Claim is in play we prefer to scry
    assert_eq!(
        select_land_to_play(player, false, true)
            .unwrap()
            .card
            .primary_name(),
        grotto.primary_name()
    );
    player.zones.hand.push(OwnedCard {
        card: grotto.clone(),
        owner: id,
    });
}
