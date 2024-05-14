grammar Card;


prog: effect* EOF;


effect
	: mod? trigger ',' mod? action '.'
	| action '.'
	;


trigger
    : trigger_word=('whenever' | 'when') event #eventTrigger
	| 'after combat' #afterCombatTrigger
	| 'at the end of the turn' #endOfTurnTrigger
	;


event
    : unit_event
    | board_event
    | you_event
    ;


unit_event
    : 'i' 'am'? unit_event_inner
	| 'a player plays a nonunit spell targeting me'
	| 'you play me'
	| 'my column deals combat damage'
	;

unit_event_inner
    : 'dealt damage'
    | 'modded or applied as a mod'
    | 'survive damage'
    | 'die'
    | 'spawn'
    | 'despawn'
    | 'attack'
    | 'block'
    | 'attack in a formation of' amount 'or more units'
    | 'become targeted'
    ;


board_event:
	subject='a' inner=board_event_inner
	| subject='another' inner=board_event_inner
	;

board_event_inner
    : type='unit token' event_='is created'
    | type='nontoken unit' event_='dies'
    | type='nontoken enemy' event_='dies'
    | type='nontoken ally' event_='spawns'
    | type='unit' event_='dies'
    | type='ally' event_='spawns'
    | type='ally' event_='spawns during battle'
    | type='card' event_='enters a player\'s hand during battle'
	;


player_event
    : subject='a player' inner=player_event_inner
	| subject='another player' inner=player_event_inner
	;

player_event_inner
    : 'loses life'
    | 'loses life during battle'
    | 'loses is dealt combat damage'
    | 'plays a spell'
    | 'plays their first spell in this battle'
    ;


you_event
    : subject='you' inner=you_event_inner
    ;

you_event_inner
    : 'play a spell'
    | 'play a spell during battle'
    | 'put a counter on an enemy'
    | 'create a token'
    | 'sacrifice a unit'
    | 'play a nontoken spell'
    | 'are dealt combat damage'
    | 'deal combat damage to an opponent'
    | 'put one or more counters on an ally'
    | 'play a unit'
    | 'apply an augment during battle'
    | 'do'
    | 'play a token spell'
    ;


action:
	action_put_counter
	| action_buff
	| action_stat_change
	| action_deal_damage
	| action ', then' action;


action_buff:
    'target' buff_target=('unit'|'ally') 'gains' stat derived_quantity='for each of your units'? ('and' evergreen_keyword)? 'until regroup'
    ;


action_deal_damage:
	'i deal' (
		(DIGIT | 'x') 'damage'
		| 'damage' unit_derived_quantity
	) 'to' ('each' ('player' | 'opponent' | 'unit')) 'for each blocked column'?;


counter_target
    : self_target='me'
    | 'target' target=('unit'|'ally') region_derived_quantity?
    | 'each' target_each=(
        'unit'
        | 'enemy'
        | 'of your units'
        | 'of the chosen units'
    )
    ;


action_put_counter: 'put' amount_item counter 'on' counter_target;


action_stat_change:
	('target' ('unit' | 'ally') 'gains' | 'your units gain') (
		stat 'and' evergreen_keyword
		| stat
		| evergreen_keyword
	) region_derived_quantity? lifetime;


region_derived_quantity:
	'for each of your' ('units' | affinity);


lifetime: 'until regroup';


counter: stat 'counter';


stat: power=signed_int '/' defence=signed_int;


evergreen_keyword: 'flying' | 'piercing';

keyword
    : 'recall'
    | 'ambush'
    | 'fight'
    | 'glimpse' amount
    | 'rockfall' amount
    ;


mod: graft | augment;


graft: '[graft' limit=amount? ']';


augment: '[augment]'limit='[once]'?;


affinity: '[' ('r' | 'b' | 'e' | 'g' | 'm') ']';


signed_int: SIGN DIGIT;


amount: DIGIT | NUMBER_WORD;


amount_item: 'a' | amount;


unit_derived_quantity:
	'equal to my' ('defense')
	| 'equal to your' affinity;


/* TOKENS */

NUMBER_WORD:
	'one'
	| 'two'
	| 'three'
	| 'four'
	| 'five'
	| 'six'
	| 'seven'
	| 'eight'
	| 'nine'
	| 'ten';
DIGIT: [0-9];
SIGN: '+' | '-';

/* SKIPPED TOKENS */

COMMENT: '{i}(' .+? ')' -> skip;
META: '{' .+? '}' -> skip;
PSUDO_NEWLINE: '-' ' '? '{/n}' -> skip;
SPACE: ' ' -> skip;
NEWLINE: [\r\n]+ -> skip;
