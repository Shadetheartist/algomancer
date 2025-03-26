
[This article on which reveals much about how MTGA accomplishes their GRE.](https://magic.wizards.com/en/news/mtg-arena/on-whiteboards-naps-and-living-breakthrough) MTGA is mostly a great product so taking notes from their design could help us avoid architectural pitfalls, and perhaps we can make some improvements of our own. 

I'm going to reduce the article's salient points for this note about the GRE.

MTGA splits responsibilities into GRE (C++) and [CLIPS](https://clipsrules.net/) logic. There is also a Game Rules Parser (GRP).

Their GRE is responsible for handling logic around basic game rules, but is not specialized in a way that would let it understand the minutiae of object-to-object interaction.

# CLIPS

 [CLIPS](https://clipsrules.net/) is a language designed to build 'expert systems'. A style of AI that uses a 'knowledge base' of rules which are queried by an 'inference engine' to give answers to complex questions. 

The article doesn't go into exact detail about the way CLIPS is used internally, but our own use of CLIPS could prove to be the most straightforward approach to resolving complex game rules interactions. It's simple, data driven, should be quite performant, and it's field tested by MTGA. However, devs need to learn CLIPS, which is obscure and ancient, and i'm wondering how easy it is to debug issues stemming from CLIPS logic errors?

The main goal of CLIPS is to take a list of things the GRE thinks it will do and modify the list to reflect the additional influence of a variety of effects. So if we don't use CLIPS for this, then a different architecture would be required to solve the same problem.

# Who's the Boss?

Notably also, this quote reflects that their internal systems have the GRE *asking the client* to make a choice. Which the client just needs to resolve, then the GRE can get back in action. It's an important distinction.

> \[the] GRE is constantly coming to points in a _Magic_ game in which it needs a player to make a decision. It sends a message asking the client to make a choice, and it's the responsibility of the DS team to display that choice to the user, which they do in a variety of different ways, sometimes highlighting creatures on the battlefield, sometimes popping up a dialog of some sort, sometimes displaying various buttons, etc.

