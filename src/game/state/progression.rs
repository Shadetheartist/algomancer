use serde::{Deserialize, Serialize};

use crate::game::state::{GameMode, State};
use crate::game::state::card::{Card, CardId};
use crate::game::state::deck::Deck;
use crate::game::state::hand::Hand;
use crate::game::state::player::{Player, PlayerId};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Phase {
    PrecombatPhase(PrecombatPhaseStep),
    CombatPhaseA(CombatPhaseAStep),
    CombatPhaseB(CombatPhaseBStep),
    MainPhase(MainPhaseStep),
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum PrecombatPhaseStep {
    Untap,
    Draw,
    Draft,
    ITMana,
    NITMana,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CombatPhaseAStep {
    ITPrepareFormation,
    ITAttack,
    AfterITAttackPriorityWindow,
    NITBlock,
    AfterNITBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CombatPhaseBStep {
    NITPrepareFormation,
    NITAttack,
    AfterNITAttackPriorityWindow,
    ITBlock,
    AfterITBlockPriorityWindow,
    Damage,
    AfterCombatPriorityWindow,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum MainPhaseStep {
    Regroup,
    ITMain,
    NITMain,
}


impl Phase {
    #[allow(dead_code)]
    pub fn is_combat(&self) -> bool {
        match self {
            Phase::CombatPhaseA(_) => true,
            Phase::CombatPhaseB(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_priority_window(&self) -> bool {
        match self {
            Phase::CombatPhaseA(step) => {
                match step {
                    CombatPhaseAStep::AfterITAttackPriorityWindow => true,
                    CombatPhaseAStep::AfterNITBlockPriorityWindow => true,
                    CombatPhaseAStep::AfterCombatPriorityWindow => true,
                    _ => false,
                }
            }
            Phase::CombatPhaseB(step) => {
                match step {
                    CombatPhaseBStep::AfterNITAttackPriorityWindow => true,
                    CombatPhaseBStep::AfterITBlockPriorityWindow => true,
                    CombatPhaseBStep::AfterCombatPriorityWindow => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    // this returns the next phase & step given the current phase & step
    pub fn get_next_step(&self, game_mode: &GameMode) -> Phase {
        match self {
            Phase::PrecombatPhase(step) => {
                match step {
                    PrecombatPhaseStep::Untap => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::Draw)
                    }
                    PrecombatPhaseStep::Draw => {
                        match &game_mode {
                            // skip the draft step in constructed
                            GameMode::Constructed { .. } => Phase::PrecombatPhase(PrecombatPhaseStep::ITMana),
                            _ => Phase::PrecombatPhase(PrecombatPhaseStep::Draft),
                        }
                    }
                    PrecombatPhaseStep::Draft => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::ITMana)
                    }
                    PrecombatPhaseStep::ITMana => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::NITMana)
                    }
                    PrecombatPhaseStep::NITMana => {
                        Phase::CombatPhaseA(CombatPhaseAStep::ITPrepareFormation)
                    }
                }
            }
            Phase::CombatPhaseA(step) => {
                match step {
                    CombatPhaseAStep::ITPrepareFormation => {
                        Phase::CombatPhaseA(CombatPhaseAStep::ITAttack)
                    }
                    CombatPhaseAStep::ITAttack => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterITAttackPriorityWindow)
                    }
                    CombatPhaseAStep::AfterITAttackPriorityWindow => {
                        Phase::CombatPhaseA(CombatPhaseAStep::NITBlock)
                    }
                    CombatPhaseAStep::NITBlock => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterNITBlockPriorityWindow)
                    }
                    CombatPhaseAStep::AfterNITBlockPriorityWindow => {
                        Phase::CombatPhaseA(CombatPhaseAStep::Damage)
                    }
                    CombatPhaseAStep::Damage => {
                        Phase::CombatPhaseA(CombatPhaseAStep::AfterCombatPriorityWindow)
                    }
                    CombatPhaseAStep::AfterCombatPriorityWindow => {
                        Phase::CombatPhaseB(CombatPhaseBStep::NITPrepareFormation)
                    }
                }
            }
            Phase::CombatPhaseB(step) => {
                match step {
                    CombatPhaseBStep::NITPrepareFormation => {
                        Phase::CombatPhaseB(CombatPhaseBStep::NITAttack)
                    }
                    CombatPhaseBStep::NITAttack => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterNITAttackPriorityWindow)
                    }
                    CombatPhaseBStep::AfterNITAttackPriorityWindow => {
                        Phase::CombatPhaseB(CombatPhaseBStep::ITBlock)
                    }
                    CombatPhaseBStep::ITBlock => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterITBlockPriorityWindow)
                    }
                    CombatPhaseBStep::AfterITBlockPriorityWindow => {
                        Phase::CombatPhaseB(CombatPhaseBStep::Damage)
                    }
                    CombatPhaseBStep::Damage => {
                        Phase::CombatPhaseB(CombatPhaseBStep::AfterCombatPriorityWindow)
                    }
                    CombatPhaseBStep::AfterCombatPriorityWindow => {
                        Phase::MainPhase(MainPhaseStep::Regroup)
                    }
                }
            }
            Phase::MainPhase(step) => {
                match step {
                    MainPhaseStep::Regroup => {
                        Phase::MainPhase(MainPhaseStep::ITMain)
                    }
                    MainPhaseStep::ITMain => {
                        Phase::MainPhase(MainPhaseStep::NITMain)
                    }
                    MainPhaseStep::NITMain => {
                        Phase::PrecombatPhase(PrecombatPhaseStep::Untap)
                    }
                }
            }
        }
    }
}

impl State {
    fn reset_player_draft_flags(&mut self) {
        self.players_mut().iter_mut().for_each(|t| t.has_drafted = false)
    }

    fn reset_player_priority(&mut self) {
        self.players_mut().iter_mut().for_each(|p| p.passed_priority = false);
    }

    pub fn move_card(card_id: CardId, from: &mut Vec<Card>, to: &mut Vec<Card>) -> Result<(), &'static str> {
        if let Some(index) = from.iter().position(|c| c.card_id == card_id) {
            let card = from.remove(index);
            to.push(card);
            Ok(())
        } else {
            Err("cannot move card, it does not exist in 'from' vec")
        }
    }



    pub fn player_draw_n_cards(&mut self, player_id: PlayerId, n: usize){

        let cards = {
            let mut deck = self.player_deck_mut(player_id);
            let mut cards = Vec::new();
            for _ in 0..n {
                let card = deck.draw().expect("a card");
                cards.push(card);
            }
            cards
        };

        let mut player = self.player_mut(player_id).expect("player");
        for card in cards {
            player.hand.cards.push(card);
        }
    }

    fn each_player_takes_draw_step_cards(&mut self) {
        let player_ids: Vec<PlayerId> = self.players().into_iter().map(|p| p.player_id).collect();
        for p_id in player_ids {
            self.player_draw_n_cards(p_id, 2);
        }
    }

    fn player_moves_pack_into_hand(&mut self) {
        let mut players = self.players_mut();
        for p in players {
            if let Some(pack) = &mut p.pack {
                p.hand.cards.append(&mut pack.cards);
            }
        }
    }

    pub fn transition_to_next_step(&mut self) {
        let next_step = self.step.get_next_step(&self.game_mode);
        println!("Transitioning from {:?} to {:?}", self.step, next_step);

        self.reset_player_priority();

        match next_step {
            Phase::PrecombatPhase(PrecombatPhaseStep::Untap) => {
                self.reset_player_draft_flags()
            }
            Phase::PrecombatPhase(PrecombatPhaseStep::Draw) => {
                self.each_player_takes_draw_step_cards()
            }
            Phase::PrecombatPhase(PrecombatPhaseStep::Draft) => {
                self.player_moves_pack_into_hand()
            }
            _ => {}
        }

        self.step = next_step;
    }
}

#[cfg(test)]
mod tests {
    use crate::game::state::GameMode;
    use crate::game::state::progression::{MainPhaseStep, Phase, PrecombatPhaseStep};

    #[test]
    fn test_phase_next() {
        let initial_phase = Phase::PrecombatPhase(PrecombatPhaseStep::Untap);
        let mode = &GameMode::new_player_mode();
        // there aren't nearly 100 steps in a round,
        // so if we get to the end of a round before the loop is over, the test is successful
        let mut phase = initial_phase.clone();
        for _ in 0..100 {
            println!("{:?}", phase);
            phase = phase.get_next_step(mode);

            // we got from the beginning to the end of the loop, success!
            if phase == Phase::MainPhase(MainPhaseStep::NITMain) {
                return;
            }
        }

        // go one more
        phase = phase.get_next_step(mode);

        // we should be back to the initial phase
        assert_eq!(phase, initial_phase);
    }
}