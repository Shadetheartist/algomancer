# Algomancer - The Algomancy Game Rules Engine  

## Design Considerations

Server (long running process)

CLI (stateless)

State separate from Cards DB & Action History
- strictly speaking, the cards db will be completely immutable throughout the lifetime of a game, so it doesn't need 
constant re-evaluation & re-serialization in a game server context.
- And action history is supplementary data as well. When operating, it will be much more efficient to only send the state.


Action Classes?
- classify actions into groups so that it can be reported that a class of actions is possible, without enumerating every single one. 
  - ex: the 'Draft' action class could be valid for a player id, but not enumerating every single valid draft.

Effect Layering & timestamps (game-step-stamps)
- 0: Printed stats 
- 1: Base stats
- 2: Permanent buffs
- 3: temporary buffs
- 4: abilities/static effects
- 5: attributes
- 6: Unaware

State Isolation per Region
- for actions like drafting, after applying the action, there is no need to recompute the actions for other regions.  
  isolating action computation to regions may be a big performance boost. However, to cache actions in state would lead to an enormous state file. 
  And other than that, we have nowhere to put the data, so action caching would otherwise require a move to long-running processes. which is not the plan.

Freeplay mode (initial goal)
- enforces only the base rules of the game, does not handle card abilities at all
- to allow games to be piloted by players, allowing them to move cards in and out of various zones
- like cockatrice


Player view info filtering

Action history

n>2 Players
- multiple panes for UI (not that we care in this application)
- players are connected in an n-gon formation, with two neighbors
- FFA
- Teams

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
- Cards
  - Resources
  - Units
    - Tokens
  - Spells
    - Tokens
    - Haste, Virus 

Progression
- Turns
- Steps / Phases
- Initiative
- Priority
- Per player priority for simultaneous actions

Actions
- a set of effects which are all resolved as a unit
- auto-resolve if it's the only action a player has left

Effects
- effects go on the stack
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
- game has an async nature during some steps, in general, async issues are resolved as first-come-first-serve
