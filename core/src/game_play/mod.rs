use crate::{card::Card, game_play::player::PlayerId};

pub mod battlefield;
pub mod counters;
pub mod player;

#[derive(Debug, Clone)]
pub struct OwnedCard {
    pub card: Card,
    pub owner: PlayerId,
}
