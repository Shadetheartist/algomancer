# Algomancer - The Algomancy Game Rules Engine  

## Design Considerations

Player view info filtering

Action history

n>2 Players
- multiple panes for UI (not that we care in this application)

Zones (this is some sort of top level object)
- card zones
- player zones
- main deck (does this even have a zone?)

Cards
- compile time guarantees from macros?
- procedural macros to enable automatic effect ids?

Determinism
- random seeding initial state
- serializing / deserializing rand
- game state hashing
- f(s, a) = s'
- Replay-ability
    - Initial state + list of actions = State N
    - expand initial 'state + actions' into individual game states for efficient history browsing

game history browser (external application?)
- move between states recorded in the game's history

game states database (external application?)
- input hash gets a game state
- serialization of game states and actions

Optimizations
- Parallelism (rayon crate?)
- computed fields (lazily computed as needed per state)

Objects
- have identifiers
- targetable, target filtering

Effects
- triggered effects
- target objects
- preparation stage
    - came across this idea when implementing an effect which used a random value.
      probably the parameters in effect need to be fully resolved before they are applied to the game state.
      So like, a random, or X, damage effect would turn into a normal damage effect with a concrete value. 
    
- replacement effects
- input between effects 
    - for example, *strange bargain: target player reveals their hand. you choose a card from it and put it into your hand.*
      - this would be decomposed into an effect for 'reveal hand', then a
    
Webassembly Support
- need to make sure included crates also have webassembly support 

Documentation Generation
- possibly some way to generate a glossary of effects?
- partial rulebook generation?

Organization (Obsidian?)

CI/CD

Rust macros can probably do a lot of work

Input mechanism
- consider how it affects multiplayer 
- consider actions requiring input in-between effects
- we may need a sort of transaction system for in-between states, 
  if in-between states are even real (they're maybe also valid states)

State in general
- at any point, the immutable state should basically be valid. A good way to think about it is around the framing 
  device of 'if i disconnected from the game at this moment, and reconnected, would the game show me the same view?'