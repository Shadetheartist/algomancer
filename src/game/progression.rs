
pub enum Stage {
    Setup(SetupStageStep),
    Play(PlayStageStep),
}

pub enum SetupStageStep {
    FactionSelection,
    PlayerPreparation, // might be irrelevant in digital
}

pub enum PlayStageStep {
    Draw,
    Draft,
    Mana,
    Combat(CombatStep),
}


pub enum CombatStep {
    DeclareAttacks, // in-formation

    // priority window

    DeclareBlocks, // and counter-attack (not in formation)

    // priority window

    Damage,

    AfterCombat // priority window
}
