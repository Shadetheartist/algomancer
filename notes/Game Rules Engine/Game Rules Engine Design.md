
# Core Design Principles

## Simplicity
*Keep it simple and they will come, and it will be nice.*

As for any software, avoiding unnecessary complexity does wonders for code quality and maintainability. For open-source projects, an approachable code-base is even more desirable.
Low complexity reduces the barrier of entry for interested developers. It makes the process of becoming familiar with the code less onerous, and working with the code is overall much more enjoyable.

## Quality
*We really don't want bugs.*

For a project as inherently complex as a MTG-like game rules engine, bugs are inevitable. Even a small number of bugs can sink a project like this, because bugs erode the authority of the GRE, and players lose trust in the software. If players can't trust that the game is behaving properly, it's likely that they would prefer a playground like *TTS* or *Cockatrice*, where game rules are not enforced, and they can do what they want to do without interference from a broken game engine.
The importance of quality requires that the software has been thoroughly tested before release. In addition, a bug reporter would serve as a great tool for getting unknown bugs fixed ASAP. 

## Testable
*Avoid stress by testing the shit out of everything.*

Testing in a project like this is key to developer confidence. Since this is the foundation in which the rest of the project is built on, it's absolutely key that regressions are caught consistently through a battery of unit and integration tests.
We can test against the most ridiculous states we can imagine to ensure that the GRE follows our expectations.  

## Data Driven
*Cards are data.*

Algomancy is comprised of two things. A rule-book and a set of cards. The rule-book is like the GRE, and the cards are like a dataset for the GRE to work with. To avoid desynchronisation issues the card text itself is the database for the game. The card text is parsed into logic and applied to the game state by the GRE.

## Fast / Lightweight
*Limits are lame.*

It's not unlikely for a game of Algomancy to reach pretty disgusting levels of complexity, with a mass of tokens, competing layers of static effects, a stack like a can of Pringles, if you've played you know what I mean.
I'd like to avoid adding artificial limits to the number of units possible (within reason). To me it's really sad to see that [MTGA has a limit of only 250 permanents at once.](https://www.reddit.com/r/magicTCG/comments/11efcr9/is_there_a_max_token_count_for_arena/) 
The key to avoiding low limits is to have a lean, performant GRE. 

## Scalable

The GRE should be able to understand cards it's never seen before, so long as the effects exist. It should also be able to be shard-ed and scaled horizontally. The game might not get that popular but it shouldn't be a concession to design it in a way where it can be scaled up if needed.

## Flexible

Adding functionality the the GRE as new mechanics are introduced is essential. It's important that the system is designed in a way where further alterations and improvements can be accomplished. 

# Existing Engines to Reference

Early consideration of the pros/cons of various styles of implementation will help us build something we can be confident will work. [[Other Rules Engines]]



# Choice of Language
## Rust

[[#Quality]]
Building this in `Rust` will help maintain stability during development, especially when written in a way to maximally utilize Rust's powerful language features that ensure safety during compile time. This doesn't stop all bugs, but an entire class of them are practically eliminated. 

[[#Testable]]
Rust's package manager, `Cargo`, comes with a really nice & easy way to write both unit and integration tests.

[[#Fast / Lightweight]]
Rust is a low level language, so any performance issues can be addressed without being blocked by the base-level performance of a language. It's on us.

[[#Flexible]]
Rust's strict but expressive syntax helps keep code maintainable. And It's macros could do a lot of work in keeping the code-base simple to understand. 

## Antlr4 (Card Parser)

[[#Data Driven]]
Using the card text as the source of truth for the abilities they have and the effects they produce eliminates potential data synchronization issues between what a card says and what it does.

[[#Scalable]]
Unique effects are implemented in the core game logic, these become building blocks which cards utilize to enact their effects. As effects are implemented, cards become playable automatically, since the parser now knows what to do with them.

[[#Flexible]]
New cards can be created by players using the implemented effects. These could potentially be parsed during runtime for experimentation, testing, and fun. 

## CLIPS

This is in part how MTGA works. They use C++ & CLIPS in tandem to achieve their aim. [[How does MTGA do it#CLIPS]]
