use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

use crate::game::state::card::CardId;
use crate::game::state::error::StateError;
use crate::game::state::error::EntityNotFoundError;
use crate::game::state::formation::{DefensiveFormation, Formation};
use crate::game::state::permanent::Permanent;
use crate::game::state::player::{Player, PlayerId, TeamId};
use crate::game::state::progression::{Phase, PrecombatPhaseStep, Team};
use crate::game::state::stack::Stack;
use crate::game::state::State;
use crate::game::state::unordered_cards::UnorderedCards;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct RegionId(pub u8);

impl Display for RegionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Region {
    pub id: RegionId,
    pub owner_player_id: PlayerId,
    pub players: Vec<Player>,
    pub unformed_permanents: Vec<Permanent>,
    pub attacking_formation: Option<Formation<Permanent>>,
    pub defending_formation: Option<DefensiveFormation<Permanent>>,
    pub step: Phase,
    pub stack: Stack,
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

    pub fn sole_team_player(&self, team_id: TeamId) -> &Player {
        let players_on_team: Vec<&Player> = self.players.iter().filter(|p| p.team_id == team_id).collect();
        if players_on_team.len() == 1 {
            players_on_team[0]
        } else {
            panic!("This region must have a single player on team {:?} occupying it to call this function.", team_id)
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

    pub fn active_team_id(&self, state: &State) -> Option<TeamId> {
        let active_team = self.step.active_team()?;
        match active_team {
            Team::IT => {
                Some(state.initiative_team())
            }
            Team::NIT => {
                Some(state.non_initiative_team())
            }
        }
    }
}

impl State {

    pub fn find_region(&self, region_id: RegionId) -> Result<&Region, EntityNotFoundError> {
        match self.regions.iter().find(|r| r.id == region_id) {
            None => {
                Err(EntityNotFoundError::Region(region_id))
            }
            Some(region) => {
                Ok(region)
            }
        }
    }

    pub fn find_region_mut(&mut self, region_id: RegionId) -> Result<&mut Region, EntityNotFoundError> {
        match self.regions.iter_mut().find(|r| r.id == region_id) {
            None => {
                Err(EntityNotFoundError::Region(region_id))
            }
            Some(region) => {
                Ok(region)
            }
        }
    }

    pub fn region_counterclockwise_neighbour(&self, region_id: RegionId) -> Option<&Region> {
        let self_idx_result = self.regions.iter().enumerate().find(|(_, val)| val.id == region_id);
        match self_idx_result {
            None => None,
            Some((self_idx, _)) => {
                let neighbour_idx = wrap_index(self.regions.len(), self_idx as i32 - 1).expect("a wrapped index");
                Some(&self.regions[neighbour_idx])
            }
        }
    }

    pub fn region_clockwise_neighbour(&self, region_id: RegionId) -> Option<&Region> {
        let self_idx_result = self.regions.iter().enumerate().find(|(_, val)| val.id == region_id);
        match self_idx_result {
            None => None,
            Some((self_idx, _)) => {
                let neighbour_idx = wrap_index(self.regions.len(), (self_idx + 1) as i32).expect("a wrapped index");
                Some(&self.regions[neighbour_idx])
            }
        }
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

        region.id
    }

    pub fn find_region_containing_player_mut(&mut self, player_id: PlayerId) -> &mut Region {
        let region = self.regions.iter_mut().find(|r| {
            r.players.iter().any(|p| p.id == player_id)
        }).expect("a region containing this player");

        region
    }

    pub fn find_region_containing_player(&self, player_id: PlayerId) -> Result<&Region, EntityNotFoundError> {
        let find_result = self.regions.iter().find(|r| {
            r.players.iter().any(|p| p.id == player_id)
        });

        match find_result {
            None => {
                Err(EntityNotFoundError::Player(player_id))
            }
            Some(region) => {
                Ok(region)
            }
        }
    }

    fn each_player_sends_pack_clockwise(mut self) -> State {

        // make a vec of packs, which will be populated where each
        // index holds it's respective region's neighbours pack
        let mut packs: Vec<UnorderedCards> = Vec::new();

        for region in self.regions.iter() {
            // by using the counter-clockwise neighbour here, the packs are remapped so
            // that when we apply the changes, the packs are aligned with the clockwise neighbour
            let neighbouring_region = self.region_counterclockwise_neighbour(region.id).expect("a neighbouring region");
            let neighbour_pack = neighbouring_region.sole_player().pack.as_ref().expect("a neighbouring pack");
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

        match next_step {
            Phase::PrecombatPhase(PrecombatPhaseStep::Draw) => {
                self.each_player_in_region_takes_draw_step_cards(region_id)
            }
            Phase::PrecombatPhase(PrecombatPhaseStep::Draft) => {
                self.players_in_region_combine_packs_with_hand(region_id)
            }
            Phase::PrecombatPhase(PrecombatPhaseStep::Mana(Team::IT)) => {
                self = self.each_player_sends_pack_clockwise()
            }
            _ => {}
        }

        {
            let region = self.find_region_mut(region_id).expect("a region");
            // eprintln!("Region {:?} is transitioning from {:?} to {:?}", region.id, region.step, next_step);
            region.step = next_step;
        }

        self
    }

    pub fn players_on_team(&self, team_id: TeamId) -> Result<Vec<&Player>, StateError> {
        let team_players = self.players().filter(|p| p.team_id == team_id).collect();
        Ok(team_players)
    }


}

pub fn wrap_index(len: usize, idx: i32) -> Option<usize> {
    if len == 0 {
        return Some(0)
    }

    if idx == 0 {
        return Some(0)
    }

    // on the off-chance we can't actually compute this
    if len > i32::MAX as usize {
        return None
    }

    let i_len = len as i32;

    if idx >= 0 {
        Some((idx % i_len) as usize)
    } else {
        let abs_idx = idx.abs() - 1;
        let e = abs_idx % i_len;
        let f = (i_len - e) - 1;
        Some(f as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::state::region::wrap_index;

    #[test]
    fn test_wrap_index(){
        assert_eq!(wrap_index(usize::MAX, 333), None);
        assert_eq!(wrap_index(6, 0).unwrap(), 0);
        assert_eq!(wrap_index(6, 3).unwrap(), 3);
        assert_eq!(wrap_index(5, 5).unwrap(), 0);
        assert_eq!(wrap_index(5, 6).unwrap(), 1);
        assert_eq!(wrap_index(5, -1).unwrap(), 4);
        assert_eq!(wrap_index(5, -5).unwrap(), 0);
        assert_eq!(wrap_index(10, -5).unwrap(), 5);
        assert_eq!(wrap_index(0, -1).unwrap(), 0);
        assert_eq!(wrap_index(333, 0).unwrap(), 0);
    }
}