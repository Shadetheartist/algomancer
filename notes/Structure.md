
## Client 

The Algomancy client, the front-end UI for the game is the most important part of the system. If this client feels good to use, like hearthstone or MTGA, we are in great shape. How can we maximize the possibility of replicating their success?


## Game Rules Engine
### Replays



## Online Multiplayer

### Lobby System
A lobby system like counter strike or AoE2 had is a great way to get people into games. 

### Matchmaking
The goal of matchmaking it to optimize solo player enjoyment by getting them into a game with someone where there is a high likelihood of the game being fun for both players (not a stomp). 

To facilitate this there is usually a "Find Match" button, when the user clicks the button, they are matched with an opponent of approximately equal strength. 

However, determining player strength in this game is a very difficult problem to tackle because it has a high randomness factor. Most systems (Elo, Glicko, TrueSkill) to determine player skill don't apply. Plus, for constructed, the strength of decks determines a lot about who will win. And determining that strength analytically is some NP-hard problem.

If we had enough data, there is some potential to analyze decks statistically determine their strength. I'm pretty sure MTGA does this, the result is that players playing high-power meta decks are more often matched with someone doing the same. And players playing some theory-crafted funny garbage are also playing someone matching their energy. 

I think i we could have something similar in place, it would improve the quality of the games overall. However it could be controversial, as there's computers involved to sway the balance of 'fair matchmaking'. Something similar could be said for potential [[Design#Hand Smoothing]].

