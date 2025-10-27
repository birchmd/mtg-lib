use std::collections::VecDeque;

use rand::seq::SliceRandom;

use crate::{card::Card, game_play::OwnedCard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(u32);

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub id: PlayerId,
    pub life_total: i32,
    pub zones: Zones,
}

impl PlayerState {
    // Initialize a player with 20 life and given deck as the library.
    // The library is shuffled.
    pub fn new(id: u32, deck: Vec<Card>) -> Self {
        let id = PlayerId(id);
        let mut library: Vec<OwnedCard> = deck
            .into_iter()
            .map(|card| OwnedCard { card, owner: id })
            .collect();
        library.shuffle(&mut rand::rng());
        Self {
            id,
            life_total: 20,
            zones: Zones {
                hand: Vec::new(),
                library: library.into(),
                graveyard: Vec::new(),
                exile: Vec::new(),
            },
        }
    }

    // Returns None if the draw failed
    pub fn draw_a_card(&mut self) -> Result<(), Decked> {
        let card = self.zones.library.pop_front().ok_or(Decked)?;
        self.zones.hand.push(card);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Zones {
    pub hand: Vec<OwnedCard>,
    pub library: VecDeque<OwnedCard>,
    pub graveyard: Vec<OwnedCard>,
    pub exile: Vec<OwnedCard>,
}

#[derive(Debug, Clone, Copy)]
pub struct Decked;
