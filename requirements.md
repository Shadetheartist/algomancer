Player view info filtering

Action history

n>2 Players

Zones (this is some sort of top level object)
    - card zones
    - player zones

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
    - Parallelism (rayon?)
    - computed fields (lazily computed as needed per state)

Objects
    - have identifiers
    - targetable, target filtering

Effects
    - target objects
    - preparation stage
        came across this idea when implementing an effect which used a random value.
        probably the parameters in effect need to be fully resolved before they are applied to the game state.
        So like, a random, or X, damage effect would turn into a normal damage effect with a concrete value.

Webassembly

Documentation Generation
    - possibly some way to generate a glossary of effects?
    - partial rulebook generation?

Organization (Obsidian?)

CI/CD