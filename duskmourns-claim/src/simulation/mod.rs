use mtg_lib_core::{
    card::{
        abilities::{Ability, EndStepAbility}, Card
    },
    game_play::{battlefield::{Battlefield, Event}, player::PlayerState},
};

const CLAIM_NAME: &str = "Duskmourn's Claim";

mod cast_spell;
mod play_land;

pub enum GameFlow {
    Continue,
    Victory,
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
    // TODO: mulligan rules

    let player = battlefield.players.first_mut().unwrap();
    let mut hand = Vec::with_capacity(7);
    for _ in 0..7 {
        let card = player.zones.library.pop_front().unwrap();
        battlefield.log.push(Event::Draw(card.clone()));
        hand.push(card);
    }
    player.zones.hand = hand;
}

// Returns the number of turns to victory
pub fn simulation_run() -> usize {
    let mut battlefield = initialize();
    opening_hand(&mut battlefield);

    let mut turn = 0;
    loop {
        turn += 1;
        if let GameFlow::Victory = turn_cycle(&mut battlefield) {
            return turn;
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
        // TODO: this is actually a loss by decking
        return GameFlow::Victory;
    }
    battlefield.log.push(Event::Draw(player.zones.hand.last().expect("A card was just drawn").clone()));

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
