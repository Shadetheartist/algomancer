use gre::GameRulesEngine;

pub fn test_scenario() -> GameRulesEngine {
    let options = gre::Options { seed: 0 };
    
    GameRulesEngine::try_from(&options).unwrap()
}

#[test]
fn test_a(){
    let gre = test_scenario();
    let actions = gre.valid_actions();
    println!("{:#?}", actions);
}