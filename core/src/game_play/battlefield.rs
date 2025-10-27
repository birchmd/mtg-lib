use crate::{
    card::{
        Card,
        abilities::{Ability, EntersAbility},
    },
    game_play::{
        OwnedCard,
        counters::Counters,
        player::{PlayerId, PlayerState},
    },
};

#[derive(Debug, Clone)]
pub struct Battlefield {
    pub players: Vec<PlayerState>,
    pub objects: Vec<InPlayObject>,
    // Log of events that took place during the game
    pub log: Vec<Event>,
}

impl Battlefield {
    pub fn play_land(&mut self, card: OwnedCard) -> Option<EntersAbility> {
        let tapped = card.card.enters_tapped();
        // TODO: there are other kinds of ETBs for lands.
        // TODO: there can be more than 1 ETB on a land.
        let etb = match &card.card {
            Card::Single(face) => face.abilities.iter().find_map(|a| {
                if let Ability::Enters(etb) = a {
                    if matches!(
                        etb,
                        EntersAbility::Scry { .. } | EntersAbility::Surveil { .. }
                    ) {
                        Some(etb.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }),
            _ => None,
        };
        let object = InPlayObject {
            controller: card.owner,
            is_token: false,
            counters: Vec::new(),
            card,
            tapped,
        };
        self.log.push(Event::EnteredPlay(object.clone()));
        self.objects.push(object);
        // TODO: technically at this point the ETB ability would go on the stack (if any).
        etb
    }

    pub fn cast_spell(&mut self, card: OwnedCard) {
        // TODO: spell should go on the stack
        // TODO: need to specify which half of a split card is being cast
        // TODO: players can have choices within the ETB. For example discover allows to hand.

        // The only non-permanent spell we current cast is Cease.
        if let Card::Split(cease, _) = &card.card
            && cease.name == "Cease"
        {
            let Ability::Other(ability) = cease.abilities.first().expect("Cease has an ability")
            else {
                panic!("Cease is an instant");
            };
            ability(self);
            // Used spell goes to the graveyard
            // TODO: should look up player based on who cast the spell
            let player = self.players.first_mut().expect("There is a player");
            player.zones.graveyard.push(card);
            return;
        }

        // TODO: there are _so many_ other kinds of etbs
        let discover_amount = match &card.card {
            Card::Single(face) => face.abilities.iter().find_map(|a| {
                if let Ability::Enters(EntersAbility::Discover { amount }) = a {
                    Some(*amount)
                } else {
                    None
                }
            }),
            _ => None,
        };

        let tapped = card.card.enters_tapped();
        let object = InPlayObject {
            controller: card.owner, // TODO: depends on caster, not owner
            is_token: false,
            counters: Vec::new(),
            card,
            tapped,
        };
        self.log.push(Event::EnteredPlay(object.clone()));
        self.objects.push(object);

        if let Some(amount) = discover_amount {
            // TODO: should look up player based on who cast the spell
            let player = self.players.first_mut().expect("There is a player");
            if let Some(index) = player
                .zones
                .library
                .iter()
                .position(|c| !c.card.is_land() && c.card.mana_value() <= amount)
            {
                let card = player
                    .zones
                    .library
                    .remove(index)
                    .expect("Card is present in find");
                self.log.push(Event::DiscoverInto(Some(card.clone())));
                // TODO: can allow player to decide to go to hand instead
                self.cast_spell(card);
            } else {
                self.log.push(Event::DiscoverInto(None));
            }
        }
    }
}

// TODO: Magic has a notion of timestamps not yet present here.
// TODO: Need to model modifications (aura's, equipment, pump spells, anthems, etc).
#[derive(Debug, Clone)]
pub struct InPlayObject {
    pub controller: PlayerId,
    pub is_token: bool,
    pub counters: Vec<Counters>,
    pub card: OwnedCard,
    pub tapped: bool,
}

/// Events that can happen during a game (e.g. play a land, scry 1 top, etc.).
#[derive(Debug, Clone)]
pub enum Event {
    StartTurn(PlayerId),
    EnteredPlay(InPlayObject),
    Tap(InPlayObject),
    Untap(InPlayObject),
    DiscoverInto(Option<OwnedCard>),
    Draw(OwnedCard),
    LostLife(PlayerId, i32),
    ScryTop(OwnedCard),
    ScryBottom(OwnedCard),
    SurveilTop(OwnedCard),
    SurveilYard(OwnedCard),
    EndTurn(PlayerId),
}