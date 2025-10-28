use {
    mtg_lib_core::{
        card::{
            Card,
            abilities::{Ability, EndStepAbility},
        },
        game_play::{
            OwnedCard,
            battlefield::{Battlefield, Event},
            player::PlayerState,
        },
    },
    rand::seq::SliceRandom,
};

const APPRAISER_NAME: &str = "Geological Appraiser";
const CLAIM_NAME: &str = "Duskmourn's Claim";

mod cast_spell;
mod play_land;

pub enum GameFlow {
    Continue,
    Victory,
    Loss,
}

pub fn initialize() -> Battlefield {
    let player_deck = crate::deck::deck();

    let mountain = player_deck
        .iter()
        .find(|c| {
            if let Card::Single(face) = c
                && face.name == "Mountain"
            {
                true
            } else {
                false
            }
        })
        .expect("Deck has a mountain")
        .clone();
    let gold_fish_deck = vec![mountain; 60];

    Battlefield {
        players: vec![
            PlayerState::new(0, player_deck),
            PlayerState::new(1, gold_fish_deck),
        ],
        objects: Vec::new(),
        log: Vec::new(),
    }
}

pub fn opening_hand(battlefield: &mut Battlefield) {
    let player = battlefield.players.first_mut().unwrap();

    let mut n_keep = 7;
    loop {
        let mut hand = Vec::with_capacity(7);
        for _ in 0..7 {
            let card = player.zones.library.pop_front().unwrap();
            battlefield.log.push(Event::Draw(card.clone()));
            hand.push(card);
        }
        player.zones.hand = hand;
        if decide_to_keep(&player.zones.hand, n_keep) {
            break;
        }
        battlefield.log.push(Event::Mulligan(player.id));
        mulligan(player);
        n_keep -= 1;
    }

    // Bottom cards if we took mulligans
    while n_keep < 7 {
        let index = player
            .zones
            .hand
            .iter()
            .position(|o| {
                // Bottom a big spell
                o.card.mana_value() > 4
            })
            .or_else(|| {
                // Or an extra land
                player.zones.hand.iter().position(|o| o.card.is_land())
            })
            .unwrap_or_default();
        let card = player.zones.hand.remove(index);
        player.zones.library.push_back(card);
        n_keep += 1;
    }
}

fn mulligan(player: &mut PlayerState) {
    let mut library = Vec::with_capacity(60);
    for card in player.zones.hand.drain(..) {
        library.push(card);
    }
    for card in player.zones.library.drain(..) {
        library.push(card);
    }
    library.shuffle(&mut rand::rng());
    player.zones.library = library.into();
}

fn decide_to_keep(hand: &[OwnedCard], n_keep: u8) -> bool {
    // Keep every hand; the heuristics I have makes things worse.
    // TODO: better mulligan heuristics.
    if n_keep < 8 {
        return true;
    }

    let has_claim = hand
        .iter()
        .any(|o| o.card.primary_name() == Some(CLAIM_NAME));
    let has_appraiser = hand
        .iter()
        .any(|o| o.card.primary_name() == Some(CLAIM_NAME));
    let n_lands = hand.iter().filter(|o| o.card.is_land()).count();

    // Keep any hand with Claim and at least two lands
    if n_lands >= 2 && has_claim {
        return true;
    }

    // Keep any hand with Appraiser and at lest three lands
    if n_lands >= 3 && has_appraiser {
        return true;
    }

    // Mulligan otherwise
    false
}

// Returns the number of turns to victory
pub fn simulation_run() -> isize {
    let mut battlefield = initialize();
    opening_hand(&mut battlefield);

    let mut turn = 0;
    loop {
        turn += 1;
        match turn_cycle(&mut battlefield) {
            GameFlow::Continue => (),
            GameFlow::Victory => {
                return turn;
            }
            GameFlow::Loss => {
                return -turn;
            }
        }
    }
}

pub fn turn_cycle(battlefield: &mut Battlefield) -> GameFlow {
    let player = battlefield.players.first_mut().unwrap();
    let player_id = player.id;
    battlefield.log.push(Event::StartTurn(player_id));

    // Untap
    for object in battlefield.objects.iter_mut() {
        if object.tapped {
            battlefield.log.push(Event::Untap(object.clone()));
        }
        object.tapped = false;
    }

    // Draw for turn
    if player.draw_a_card().is_err() {
        return GameFlow::Loss;
    }
    battlefield.log.push(Event::Draw(
        player
            .zones
            .hand
            .last()
            .expect("A card was just drawn")
            .clone(),
    ));

    play_land::play_a_land(battlefield);
    cast_spell::cast_spells(battlefield);

    // End step
    // TODO: it is probably more efficient to keep the end step triggers
    // in the `Battlefield` object and just resolve them by iterating  over
    // that list instead of iterating through all in-play objects each time.
    let mut end_step_effects = Vec::new();
    for object in &battlefield.objects {
        match &object.card.card {
            Card::Single(face) => {
                push_end_step_abilities(&mut end_step_effects, &face.abilities);
            }
            Card::Split(left_face, right_face) => {
                // TODO: need to keep track of which faces are unlocked somehow.
                push_end_step_abilities(&mut end_step_effects, &left_face.abilities);
                push_end_step_abilities(&mut end_step_effects, &right_face.abilities);
            }
            // TODO: should check other card kinds as well
            _ => (),
        }
    }
    for effect in end_step_effects {
        effect(battlefield);
    }

    // Check victory
    if battlefield.players.get(1).unwrap().life_total <= 0 {
        GameFlow::Victory
    } else {
        battlefield.log.push(Event::EndTurn(player_id));
        GameFlow::Continue
    }
}

fn push_end_step_abilities(
    end_step_effects: &mut Vec<fn(&mut Battlefield)>,
    abilities: &[Ability],
) {
    for ability in abilities {
        if let Ability::EndStep(EndStepAbility::Other(ability)) = ability {
            end_step_effects.push(*ability);
        }
    }
}
