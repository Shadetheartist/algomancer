First we need a working minimal front end client, no game rules engine involvement beyond simple game steps and phases. This would be a similar product to Cockatrice or the existing Tabletop Simulator version of Algomancy, a playground containing all the basic components which make up the game, plus some specific features to help streamline game-play. 

# Stop-Gap Product: Algomancy Sandbox

Algomancy is offered to the public as a Tabletop Simulator workshop mod. While versatile, does a poor job of providing a streamlined experience for players of Algomancy. 
In addition, it's not a free bit of software, so there's likely a large population of potentially interested players who are excluded from playing. 

## User Expectations
To maintain clarity in the sandbox’s identity, we should be cautious about adding too many features. If it sits between a fully automated game and a pure sandbox, users may find it unclear what to expect. An incomplete feature set can also lead to reliance on systems that don’t fully cover all cases, causing confusion and mistakes. Any features we include should reinforce a consistent experience.
For example, in some TTS board game mods, automation handles most steps but fails in edge cases, leading players to miss important actions. When users rely on scripted systems, gaps in automation can create more confusion than they solve.

## Multiplayer
The sandbox must be multiplayer to serve any purpose. There is some decision to be made on the flavor of networking.  
Luckily for us, high latency connections should have low impact on the experience. 

- P2P
  This is potentially more complex to implement, we have very little control over the end-user's experience.
- Client / Server
  We would have to host the games on some hardware. 

## Included Features
here are the features that we have carefully decided to incorporate.

- Object Region Traversal
  Players should be able to easily choose objects to move to other regions for the purposes of attacking and counter-attacking. 
- formation structuring
  Players should be able easily create and modify valid formations of units.
- grafting / augmenting
  It should be possible to position cards underneath other cards when grafting or augmenting, such that the cards form a new super-structure that can be manipulated as one object.
- Card Expansion
  Players should be able to hover over a card to view its details in a larger view.
  For grafted/augmented cards, the combined effect chain should be visible all at once. 
- Keyword Expansion
  While looking at card text containing a keyword, such as 'graft', players should be able to expand the keyword to read a detailed explanation of the mechanic. 
- Zone movement QoL
  Players should be able to easily enact common object-zone transfers. Such as drawing, recycling, putting into the bin and put into exile, and any other 