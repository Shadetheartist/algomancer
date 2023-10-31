use serde::{Deserialize, Serialize};

use crate::game::state::permanent::Permanent;
use crate::game::state::player::{Player, PlayerId, StateError};
use crate::game::state::player::StateError::RegionNotFound;
use crate::game::state::progression::{Phase, PrecombatPhaseStep};
use crate::game::state::State;
use crate::wrap_index::wrap_index;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct RegionId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Region {
    pub region_id: RegionId,
    pub owner_player_id: PlayerId,
    pub players: Vec<Player>,
    pub permanents: Vec<Permanent>,
    pub step: Phase,
}

impl State {
    pub fn find_region(&self, region_id: RegionId) -> Result<&Region, StateError> {
        match self.regions.iter().find(|r| r.region_id == region_id) {
            None => {
                Err(RegionNotFound(region_id))
            }
            Some(region) => {
                Ok(region)
            }
        }
    }

    pub fn find_region_mut(&mut self, region_id: RegionId) -> Result<&mut Region, StateError> {
        match self.regions.iter_mut().find(|r| r.region_id == region_id) {
            None => {
                Err(RegionNotFound(region_id))
            }
            Some(region) => {
                Ok(region)
            }
        }
    }

    pub fn region_clockwise_neighbour(&self, region_id: RegionId) -> Option<&Region> {
        let self_idx_result = self.regions.iter().enumerate().find(|(_, val)| val.region_id == region_id);
        match self_idx_result {
            None => None,
            Some((self_idx, _)) => {
                let neighbour_idx = wrap_index(self.regions.len(), self_idx as i32).expect("a wrapped index");
                Some(&self.regions[neighbour_idx])
            }
        }
    }

    fn reset_player_priority_in_region(&mut self, region_id: RegionId) {
        let players = self.players_in_region_mut(region_id).expect("a set of players in a region");
        players.iter_mut().for_each(|p| p.passed_priority = false);
    }

    fn each_player_in_region_takes_draw_step_cards(&mut self, region_id: RegionId) {
        let players = self.players_in_region_mut(region_id).expect("a set of players in a region");
        let player_ids: Vec<PlayerId> = players.iter().map(|p| p.player_id).collect();
        for p_id in player_ids {
            self.player_draw_n_cards(p_id, 2);
        }
    }

    fn players_in_region_combine_packs_with_hand(&mut self, region_id: RegionId) {
        let players = self.players_in_region_mut(region_id).expect("a set of players in a region");
        for p in players {
            if let Some(pack) = &mut p.pack {
                p.hand.cards.append(&mut pack.cards);
            }
        }
    }

    pub fn all_players_in_region_passed_priority(&self, region_id: RegionId) -> Result<bool, StateError> {
        let players = self.players_in_region(region_id)?;
        if players.len() == 0 {
            // this may be valid? if a player leaves their region to move to combat in another region
            panic!("wtf there's no players in the region")
        }
        Ok(!players.iter().any(|p| p.passed_priority == false))
    }

    pub fn players_in_region(&self, region_id: RegionId) -> Result<&Vec<Player>, StateError> {
        let region = self.find_region(region_id)?;
        Ok(&region.players)
    }

    pub fn players_in_region_mut(&mut self, region_id: RegionId) -> Result<&mut Vec<Player>, StateError> {
        let region = self.find_region_mut(region_id)?;
        Ok(& mut region.players)
    }

    pub fn find_region_id_containing_player(&self, player_id: PlayerId) -> RegionId {
        let region = self.regions.iter().find(|r| {
            r.players.iter().find(|p| p.player_id == player_id) != None
        }).expect("a region containing this player");

        region.region_id
    }

    pub fn find_region_containing_player(&self, player_id: PlayerId) -> Option<(RegionId, PlayerId)> {
        let find_result = self.regions.iter().find(|r| {
            r.players.iter().find(|p| p.player_id == player_id) != None
        });

        match find_result {
            None => {
                None
            }
            Some(region) => {
                let player = region.players.iter().find(|p| p.player_id == player_id).expect("the player we found before");
                Some((region.region_id, player.player_id))
            }
        }
    }

    pub fn region_transition_to_next_step(mut self, region_id: RegionId) -> State {
        let next_step = {
            let region = self.regions.iter().find(|r| r.region_id == region_id).expect("a region");
            let next_step = region.step.get_next_step(&self.game_mode);

            println!("Region {:?} is transitioning from {:?} to {:?}", region.region_id, region.step, next_step);

            next_step
        };


        self.reset_player_priority_in_region(region_id);

        match next_step {
            Phase::PrecombatPhase(PrecombatPhaseStep::Draw) => {
                self.each_player_in_region_takes_draw_step_cards(region_id)
            }
            Phase::PrecombatPhase(PrecombatPhaseStep::Draft) => {
                self.players_in_region_combine_packs_with_hand(region_id)
            }
            _ => {}
        }

        {
            let region = self.regions.iter_mut().find(|r| r.region_id == region_id).expect("a region");
            region.step = next_step;
        }

        self
    }
}
