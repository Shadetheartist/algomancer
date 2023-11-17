use serde::{Deserialize, Serialize};

use crate::game::state::card::CardId;
use crate::game::state::card_collection::CardCollection;
use crate::game::state::error::StateError;
use crate::game::state::error::StateError::{NoPlayersOnTeam, RegionNotFound};
use crate::game::state::formation::{DefensiveFormation, Formation};
use crate::game::state::permanent::Permanent;
use crate::game::state::player::{Player, PlayerId, TeamId};
use crate::game::state::progression::{Phase, PrecombatPhaseStep};
use crate::game::state::State;
use crate::wrap_index::wrap_index;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct RegionId(pub u8);

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Region {
    pub region_id: RegionId,
    pub owner_player_id: PlayerId,
    pub players: Vec<Player>,
    pub unformed_permanents: Vec<Permanent>,
    pub attacking_formation: Option<Formation<Permanent>>,
    pub defending_formation: Option<DefensiveFormation<Permanent>>,
    pub step: Phase,
}

impl Region {

    pub fn formations(&self) -> Vec<&Formation<Permanent>> {
        let mut formations = Vec::new();

        if let Some(f) = self.attacking_formation.as_ref() {
            formations.push(f);
        }

        if let Some(f) = self.defending_formation.as_ref() {
            formations.push(&f.formation);
        }

        formations
    }

    /// This function gets the current player in the region,
    /// it will panic if there is not exactly one player in the region.
    /// This serves a dual purpose, as there are many stages in the game where it would be a huge
    /// error if there wasn't exactly one player in the region, such as the draft step.
    pub fn sole_player(&self) -> &Player {
        if self.players.len() == 1 {
            &self.players[0]
        } else {
            panic!("This region must have a single player occupying it to call this function.")
        }
    }

    /// see sole_player
    pub fn sole_player_mut(&mut self) -> &mut Player {
        if self.players.len() == 1 {
            &mut self.players[0]
        } else {
            panic!("This region must have a single player occupying it to call this function.")
        }
    }
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

    pub fn region_counterclockwise_neighbour(&self, region_id: RegionId) -> Option<&Region> {
        let self_idx_result = self.regions.iter().enumerate().find(|(_, val)| val.region_id == region_id);
        match self_idx_result {
            None => None,
            Some((self_idx, _)) => {
                let neighbour_idx = wrap_index(self.regions.len(), self_idx as i32 - 1).expect("a wrapped index");
                Some(&self.regions[neighbour_idx])
            }
        }
    }

    pub fn region_clockwise_neighbour(&self, region_id: RegionId) -> Option<&Region> {
        let self_idx_result = self.regions.iter().enumerate().find(|(_, val)| val.region_id == region_id);
        match self_idx_result {
            None => None,
            Some((self_idx, _)) => {
                let neighbour_idx = wrap_index(self.regions.len(), (self_idx + 1) as i32).expect("a wrapped index");
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
        let player_ids: Vec<PlayerId> = players.iter().map(|p| p.id).collect();
        for p_id in player_ids {
            self.player_draw_n_cards(p_id, 2);
        }
    }

    fn players_in_region_combine_packs_with_hand(&mut self, region_id: RegionId) {
        let players = self.players_in_region_mut(region_id).expect("a set of players in a region");
        for p in players {
            if let Some(pack) = &mut p.pack {
                let card_ids: Vec<CardId> = pack.iter().map(|c| c.card_id).collect();
                for c_id in card_ids {
                    pack.transfer_to(&mut p.hand, c_id).expect("a card was transferred from pack to hand");
                }
            }
        }
    }

    pub fn all_players_in_region_except_passed_priority(&self, region_id: RegionId, except: PlayerId) -> Result<bool, StateError> {
        let players = self.players_in_region_except(region_id, except)?;
        Ok(!players.iter().any(|p| !p.passed_priority))
    }

    pub fn all_players_in_region_on_team_passed_priority(&self, region_id: RegionId, team_id: TeamId) -> Result<bool, StateError> {
        let players = self.players_in_region(region_id)?;
        let players: Vec<&Player> = players.into_iter().filter(|p| p.team_id == team_id).collect();
        Ok(!players.iter().any(|p| !p.passed_priority))
    }


    pub fn players_in_region(&self, region_id: RegionId) -> Result<Vec<&Player>, StateError> {
        let region = self.find_region(region_id)?;
        Ok(region.players.iter().collect())
    }

    pub fn players_in_region_except(&self, region_id: RegionId, player_id: PlayerId) -> Result<Vec<&Player>, StateError> {
        let region = self.find_region(region_id)?;
        Ok(region.players.iter().filter(|p| p.id != player_id).collect())
    }

    pub fn players_in_region_mut(&mut self, region_id: RegionId) -> Result<&mut Vec<Player>, StateError> {
        let region = self.find_region_mut(region_id)?;
        Ok(&mut region.players)
    }

    pub fn find_region_id_containing_player(&self, player_id: PlayerId) -> RegionId {
        let region = self.regions.iter().find(|r| {
            r.players.iter().any(|p| p.id == player_id)
        }).expect("a region containing this player");

        region.region_id
    }

    pub fn find_region_containing_player_mut(&mut self, player_id: PlayerId) -> &mut Region {
        let region = self.regions.iter_mut().find(|r| {
            r.players.iter().any(|p| p.id == player_id)
        }).expect("a region containing this player");

        region
    }

    pub fn find_region_containing_player(&self, player_id: PlayerId) -> Result<&Region, StateError> {
        let find_result = self.regions.iter().find(|r| {
            r.players.iter().any(|p| p.id == player_id)
        });

        match find_result {
            None => {
                Err(StateError::NoRegionContainsPlayer(player_id))
            }
            Some(region) => {
                Ok(region)
            }
        }
    }

    fn each_player_sends_pack_clockwise(mut self) -> State {

        // make a vec of packs, which will be populated where each
        // index holds it's respective region's neighbours pack
        let mut packs: Vec<CardCollection> = Vec::new();

        for region in self.regions.iter() {
            // by using the counter-clockwise neighbour here, the packs are remapped so
            // that when we apply the changes, the packs are aligned with the clockwise neighbour
            let neighbouring_region = self.region_counterclockwise_neighbour(region.region_id).expect("a neighbouring region");
            let neighbour_pack = neighbouring_region.sole_player().pack.as_ref().expect("a pack");
            packs.push(neighbour_pack.clone());
        }

        for (idx, pack) in packs.into_iter().enumerate() {
            self.regions[idx].sole_player_mut().pack = Some(pack);
        }


        self
    }


    pub fn region_transition_to_next_step(mut self, region_id: RegionId) -> State {
        let next_step = {
            let region = self.find_region(region_id).expect("a region");
            region.step.get_next_phase(&self.game_mode)
        };

        self.reset_player_priority_in_region(region_id);

        match next_step {
            Phase::PrecombatPhase(PrecombatPhaseStep::Draw) => {
                self.each_player_in_region_takes_draw_step_cards(region_id)
            }
            Phase::PrecombatPhase(PrecombatPhaseStep::Draft) => {
                self.players_in_region_combine_packs_with_hand(region_id)
            }
            Phase::PrecombatPhase(PrecombatPhaseStep::ITMana) => {
                self = self.each_player_sends_pack_clockwise()
            }
            _ => {}
        }

        {
            let region = self.find_region_mut(region_id).expect("a region");
            eprintln!("Region {:?} is transitioning from {:?} to {:?}", region.region_id, region.step, next_step);
            region.step = next_step;
        }

        self
    }

    pub fn players_on_team(&self, team_id: TeamId) -> Result<Vec<&Player>, StateError> {
        let team_players = self.players().filter(|p| p.team_id == team_id).collect();
        Ok(team_players)
    }

    pub fn all_players_on_team_passed_priority(&self, team_id: TeamId) -> Result<bool, StateError> {
        let players = self.players_on_team(team_id)?;
        if players.is_empty() {
            return Err(NoPlayersOnTeam(team_id))
        }
        Ok(!players.iter().any(|p| !p.passed_priority))
    }

    pub fn all_players_on_team_passed_priority_except(&self, team_id: TeamId, except_player_id: PlayerId) -> Result<bool, StateError> {
        let players = self.players_on_team(team_id)?;
        if players.is_empty() {
            return Err(NoPlayersOnTeam(team_id))
        }
        Ok(!players.iter().filter(|p| p.id != except_player_id).any(|p| !p.passed_priority))
    }


}
