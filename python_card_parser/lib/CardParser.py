# Generated from Card.g4 by ANTLR 4.13.0
# encoding: utf-8
from antlr4 import *
from io import StringIO
import sys
if sys.version_info[1] > 5:
	from typing import TextIO
else:
	from typing.io import TextIO

def serializedATN():
    return [
        4,1,110,297,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,2,6,
        7,6,2,7,7,7,2,8,7,8,2,9,7,9,2,10,7,10,2,11,7,11,2,12,7,12,2,13,7,
        13,2,14,7,14,2,15,7,15,2,16,7,16,2,17,7,17,2,18,7,18,2,19,7,19,2,
        20,7,20,2,21,7,21,2,22,7,22,2,23,7,23,2,24,7,24,2,25,7,25,2,26,7,
        26,2,27,7,27,2,28,7,28,2,29,7,29,2,30,7,30,2,31,7,31,1,0,5,0,66,
        8,0,10,0,12,0,69,9,0,1,0,1,0,1,1,3,1,74,8,1,1,1,1,1,1,1,3,1,79,8,
        1,1,1,1,1,1,1,1,1,1,1,1,1,3,1,87,8,1,1,2,1,2,1,2,1,2,3,2,93,8,2,
        1,3,1,3,1,3,3,3,98,8,3,1,4,1,4,3,4,102,8,4,1,4,1,4,1,4,1,4,3,4,108,
        8,4,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,3,5,123,
        8,5,1,6,1,6,1,6,1,6,3,6,129,8,6,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,
        1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,3,7,147,8,7,1,8,1,8,1,8,1,8,3,8,
        153,8,8,1,9,1,9,1,10,1,10,1,10,1,11,1,11,1,12,1,12,1,12,1,12,1,12,
        3,12,167,8,12,1,12,1,12,1,12,5,12,172,8,12,10,12,12,12,175,9,12,
        1,13,1,13,1,13,1,13,1,13,3,13,182,8,13,1,13,1,13,3,13,186,8,13,1,
        13,1,13,1,14,1,14,1,14,1,14,1,14,3,14,195,8,14,1,14,1,14,1,14,1,
        14,1,14,3,14,202,8,14,1,15,1,15,1,15,1,15,3,15,208,8,15,1,15,1,15,
        3,15,212,8,15,1,16,1,16,1,16,1,16,1,16,1,16,1,17,1,17,1,17,1,17,
        3,17,224,8,17,1,17,1,17,1,17,1,17,1,17,1,17,3,17,232,8,17,1,17,3,
        17,235,8,17,1,17,1,17,1,18,1,18,1,18,3,18,242,8,18,1,19,1,19,1,20,
        1,20,1,20,1,21,1,21,1,21,1,21,1,22,1,22,1,23,1,23,1,23,1,23,1,23,
        1,23,1,23,3,23,262,8,23,1,24,1,24,3,24,266,8,24,1,25,1,25,3,25,270,
        8,25,1,25,1,25,1,26,1,26,3,26,276,8,26,1,27,1,27,1,27,1,27,1,28,
        1,28,1,28,1,29,1,29,1,30,1,30,3,30,289,8,30,1,31,1,31,1,31,1,31,
        3,31,295,8,31,1,31,0,1,24,32,0,2,4,6,8,10,12,14,16,18,20,22,24,26,
        28,30,32,34,36,38,40,42,44,46,48,50,52,54,56,58,60,62,0,10,1,0,3,
        4,1,0,39,43,1,0,45,57,1,0,32,33,2,0,65,65,104,104,2,0,32,32,69,70,
        2,0,32,32,73,75,1,0,83,84,1,0,95,99,1,0,103,104,319,0,67,1,0,0,0,
        2,86,1,0,0,0,4,92,1,0,0,0,6,97,1,0,0,0,8,107,1,0,0,0,10,122,1,0,
        0,0,12,128,1,0,0,0,14,146,1,0,0,0,16,152,1,0,0,0,18,154,1,0,0,0,
        20,156,1,0,0,0,22,159,1,0,0,0,24,166,1,0,0,0,26,176,1,0,0,0,28,189,
        1,0,0,0,30,211,1,0,0,0,32,213,1,0,0,0,34,223,1,0,0,0,36,238,1,0,
        0,0,38,243,1,0,0,0,40,245,1,0,0,0,42,248,1,0,0,0,44,252,1,0,0,0,
        46,261,1,0,0,0,48,265,1,0,0,0,50,267,1,0,0,0,52,273,1,0,0,0,54,277,
        1,0,0,0,56,281,1,0,0,0,58,284,1,0,0,0,60,288,1,0,0,0,62,294,1,0,
        0,0,64,66,3,2,1,0,65,64,1,0,0,0,66,69,1,0,0,0,67,65,1,0,0,0,67,68,
        1,0,0,0,68,70,1,0,0,0,69,67,1,0,0,0,70,71,5,0,0,1,71,1,1,0,0,0,72,
        74,3,48,24,0,73,72,1,0,0,0,73,74,1,0,0,0,74,75,1,0,0,0,75,76,3,4,
        2,0,76,78,5,1,0,0,77,79,3,48,24,0,78,77,1,0,0,0,78,79,1,0,0,0,79,
        80,1,0,0,0,80,81,3,24,12,0,81,82,5,2,0,0,82,87,1,0,0,0,83,84,3,24,
        12,0,84,85,5,2,0,0,85,87,1,0,0,0,86,73,1,0,0,0,86,83,1,0,0,0,87,
        3,1,0,0,0,88,89,7,0,0,0,89,93,3,6,3,0,90,93,5,5,0,0,91,93,5,6,0,
        0,92,88,1,0,0,0,92,90,1,0,0,0,92,91,1,0,0,0,93,5,1,0,0,0,94,98,3,
        8,4,0,95,98,3,12,6,0,96,98,3,20,10,0,97,94,1,0,0,0,97,95,1,0,0,0,
        97,96,1,0,0,0,98,7,1,0,0,0,99,101,5,7,0,0,100,102,5,8,0,0,101,100,
        1,0,0,0,101,102,1,0,0,0,102,103,1,0,0,0,103,108,3,10,5,0,104,108,
        5,9,0,0,105,108,5,10,0,0,106,108,5,11,0,0,107,99,1,0,0,0,107,104,
        1,0,0,0,107,105,1,0,0,0,107,106,1,0,0,0,108,9,1,0,0,0,109,123,5,
        12,0,0,110,123,5,13,0,0,111,123,5,14,0,0,112,123,5,15,0,0,113,123,
        5,16,0,0,114,123,5,17,0,0,115,123,5,18,0,0,116,123,5,19,0,0,117,
        118,5,20,0,0,118,119,3,58,29,0,119,120,5,21,0,0,120,123,1,0,0,0,
        121,123,5,22,0,0,122,109,1,0,0,0,122,110,1,0,0,0,122,111,1,0,0,0,
        122,112,1,0,0,0,122,113,1,0,0,0,122,114,1,0,0,0,122,115,1,0,0,0,
        122,116,1,0,0,0,122,117,1,0,0,0,122,121,1,0,0,0,123,11,1,0,0,0,124,
        125,5,23,0,0,125,129,3,14,7,0,126,127,5,24,0,0,127,129,3,14,7,0,
        128,124,1,0,0,0,128,126,1,0,0,0,129,13,1,0,0,0,130,131,5,25,0,0,
        131,147,5,26,0,0,132,133,5,27,0,0,133,147,5,28,0,0,134,135,5,29,
        0,0,135,147,5,28,0,0,136,137,5,30,0,0,137,147,5,31,0,0,138,139,5,
        32,0,0,139,147,5,28,0,0,140,141,5,33,0,0,141,147,5,31,0,0,142,143,
        5,33,0,0,143,147,5,34,0,0,144,145,5,35,0,0,145,147,5,36,0,0,146,
        130,1,0,0,0,146,132,1,0,0,0,146,134,1,0,0,0,146,136,1,0,0,0,146,
        138,1,0,0,0,146,140,1,0,0,0,146,142,1,0,0,0,146,144,1,0,0,0,147,
        15,1,0,0,0,148,149,5,37,0,0,149,153,3,18,9,0,150,151,5,38,0,0,151,
        153,3,18,9,0,152,148,1,0,0,0,152,150,1,0,0,0,153,17,1,0,0,0,154,
        155,7,1,0,0,155,19,1,0,0,0,156,157,5,44,0,0,157,158,3,22,11,0,158,
        21,1,0,0,0,159,160,7,2,0,0,160,23,1,0,0,0,161,162,6,12,-1,0,162,
        167,3,32,16,0,163,167,3,26,13,0,164,167,3,34,17,0,165,167,3,28,14,
        0,166,161,1,0,0,0,166,163,1,0,0,0,166,164,1,0,0,0,166,165,1,0,0,
        0,167,173,1,0,0,0,168,169,10,1,0,0,169,170,5,58,0,0,170,172,3,24,
        12,2,171,168,1,0,0,0,172,175,1,0,0,0,173,171,1,0,0,0,173,174,1,0,
        0,0,174,25,1,0,0,0,175,173,1,0,0,0,176,177,5,59,0,0,177,178,7,3,
        0,0,178,179,5,60,0,0,179,181,3,42,21,0,180,182,5,61,0,0,181,180,
        1,0,0,0,181,182,1,0,0,0,182,185,1,0,0,0,183,184,5,62,0,0,184,186,
        3,44,22,0,185,183,1,0,0,0,185,186,1,0,0,0,186,187,1,0,0,0,187,188,
        5,63,0,0,188,27,1,0,0,0,189,194,5,64,0,0,190,191,7,4,0,0,191,195,
        5,66,0,0,192,193,5,66,0,0,193,195,3,62,31,0,194,190,1,0,0,0,194,
        192,1,0,0,0,195,196,1,0,0,0,196,197,5,67,0,0,197,198,5,68,0,0,198,
        199,7,5,0,0,199,201,1,0,0,0,200,202,5,71,0,0,201,200,1,0,0,0,201,
        202,1,0,0,0,202,29,1,0,0,0,203,212,5,72,0,0,204,205,5,59,0,0,205,
        207,7,3,0,0,206,208,3,36,18,0,207,206,1,0,0,0,207,208,1,0,0,0,208,
        212,1,0,0,0,209,210,5,68,0,0,210,212,7,6,0,0,211,203,1,0,0,0,211,
        204,1,0,0,0,211,209,1,0,0,0,212,31,1,0,0,0,213,214,5,76,0,0,214,
        215,3,60,30,0,215,216,3,40,20,0,216,217,5,77,0,0,217,218,3,30,15,
        0,218,33,1,0,0,0,219,220,5,59,0,0,220,221,7,3,0,0,221,224,5,60,0,
        0,222,224,5,78,0,0,223,219,1,0,0,0,223,222,1,0,0,0,224,231,1,0,0,
        0,225,226,3,42,21,0,226,227,5,62,0,0,227,228,3,44,22,0,228,232,1,
        0,0,0,229,232,3,42,21,0,230,232,3,44,22,0,231,225,1,0,0,0,231,229,
        1,0,0,0,231,230,1,0,0,0,232,234,1,0,0,0,233,235,3,36,18,0,234,233,
        1,0,0,0,234,235,1,0,0,0,235,236,1,0,0,0,236,237,3,38,19,0,237,35,
        1,0,0,0,238,241,5,79,0,0,239,242,5,80,0,0,240,242,3,54,27,0,241,
        239,1,0,0,0,241,240,1,0,0,0,242,37,1,0,0,0,243,244,5,63,0,0,244,
        39,1,0,0,0,245,246,3,42,21,0,246,247,5,81,0,0,247,41,1,0,0,0,248,
        249,3,56,28,0,249,250,5,82,0,0,250,251,3,56,28,0,251,43,1,0,0,0,
        252,253,7,7,0,0,253,45,1,0,0,0,254,262,5,85,0,0,255,262,5,86,0,0,
        256,262,5,87,0,0,257,258,5,88,0,0,258,262,3,58,29,0,259,260,5,89,
        0,0,260,262,3,58,29,0,261,254,1,0,0,0,261,255,1,0,0,0,261,256,1,
        0,0,0,261,257,1,0,0,0,261,259,1,0,0,0,262,47,1,0,0,0,263,266,3,50,
        25,0,264,266,3,52,26,0,265,263,1,0,0,0,265,264,1,0,0,0,266,49,1,
        0,0,0,267,269,5,90,0,0,268,270,3,58,29,0,269,268,1,0,0,0,269,270,
        1,0,0,0,270,271,1,0,0,0,271,272,5,91,0,0,272,51,1,0,0,0,273,275,
        5,92,0,0,274,276,5,93,0,0,275,274,1,0,0,0,275,276,1,0,0,0,276,53,
        1,0,0,0,277,278,5,94,0,0,278,279,7,8,0,0,279,280,5,91,0,0,280,55,
        1,0,0,0,281,282,5,105,0,0,282,283,5,104,0,0,283,57,1,0,0,0,284,285,
        7,9,0,0,285,59,1,0,0,0,286,289,5,23,0,0,287,289,3,58,29,0,288,286,
        1,0,0,0,288,287,1,0,0,0,289,61,1,0,0,0,290,291,5,100,0,0,291,295,
        5,101,0,0,292,293,5,102,0,0,293,295,3,54,27,0,294,290,1,0,0,0,294,
        292,1,0,0,0,295,63,1,0,0,0,30,67,73,78,86,92,97,101,107,122,128,
        146,152,166,173,181,185,194,201,207,211,223,231,234,241,261,265,
        269,275,288,294
    ]

class CardParser ( Parser ):

    grammarFileName = "Card.g4"

    atn = ATNDeserializer().deserialize(serializedATN())

    decisionsToDFA = [ DFA(ds, i) for i, ds in enumerate(atn.decisionToState) ]

    sharedContextCache = PredictionContextCache()

    literalNames = [ "<INVALID>", "','", "'.'", "'whenever'", "'when'", 
                     "'after combat'", "'at the end of the turn'", "'i'", 
                     "'am'", "'a player plays a nonunit spell targeting me'", 
                     "'you play me'", "'my column deals combat damage'", 
                     "'dealt damage'", "'modded or applied as a mod'", "'survive damage'", 
                     "'die'", "'spawn'", "'despawn'", "'attack'", "'block'", 
                     "'attack in a formation of'", "'or more units'", "'become targeted'", 
                     "'a'", "'another'", "'unit token'", "'is created'", 
                     "'nontoken unit'", "'dies'", "'nontoken enemy'", "'nontoken ally'", 
                     "'spawns'", "'unit'", "'ally'", "'spawns during battle'", 
                     "'card'", "'enters a player's hand during battle'", 
                     "'a player'", "'another player'", "'loses life'", "'loses life during battle'", 
                     "'loses is dealt combat damage'", "'plays a spell'", 
                     "'plays their first spell in this battle'", "'you'", 
                     "'play a spell'", "'play a spell during battle'", "'put a counter on an enemy'", 
                     "'create a token'", "'sacrifice a unit'", "'play a nontoken spell'", 
                     "'are dealt combat damage'", "'deal combat damage to an opponent'", 
                     "'put one or more counters on an ally'", "'play a unit'", 
                     "'apply an augment during battle'", "'do'", "'play a token spell'", 
                     "', then'", "'target'", "'gains'", "'for each of your units'", 
                     "'and'", "'until regroup'", "'i deal'", "'x'", "'damage'", 
                     "'to'", "'each'", "'player'", "'opponent'", "'for each blocked column'", 
                     "'me'", "'enemy'", "'of your units'", "'of the chosen units'", 
                     "'put'", "'on'", "'your units gain'", "'for each of your'", 
                     "'units'", "'counter'", "'/'", "'flying'", "'piercing'", 
                     "'recall'", "'ambush'", "'fight'", "'glimpse'", "'rockfall'", 
                     "'[graft'", "']'", "'[augment]'", "'[once]'", "'['", 
                     "'r'", "'b'", "'e'", "'g'", "'m'", "'equal to my'", 
                     "'defense'", "'equal to your'", "<INVALID>", "<INVALID>", 
                     "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                     "' '" ]

    symbolicNames = [ "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                      "<INVALID>", "<INVALID>", "<INVALID>", "NUMBER_WORD", 
                      "DIGIT", "SIGN", "COMMENT", "META", "PSUDO_NEWLINE", 
                      "SPACE", "NEWLINE" ]

    RULE_prog = 0
    RULE_effect = 1
    RULE_trigger = 2
    RULE_event = 3
    RULE_unit_event = 4
    RULE_unit_event_inner = 5
    RULE_board_event = 6
    RULE_board_event_inner = 7
    RULE_player_event = 8
    RULE_player_event_inner = 9
    RULE_you_event = 10
    RULE_you_event_inner = 11
    RULE_action = 12
    RULE_action_buff = 13
    RULE_action_deal_damage = 14
    RULE_counter_target = 15
    RULE_action_put_counter = 16
    RULE_action_stat_change = 17
    RULE_region_derived_quantity = 18
    RULE_lifetime = 19
    RULE_counter = 20
    RULE_stat = 21
    RULE_evergreen_keyword = 22
    RULE_keyword = 23
    RULE_mod = 24
    RULE_graft = 25
    RULE_augment = 26
    RULE_affinity = 27
    RULE_signed_int = 28
    RULE_amount = 29
    RULE_amount_item = 30
    RULE_unit_derived_quantity = 31

    ruleNames =  [ "prog", "effect", "trigger", "event", "unit_event", "unit_event_inner", 
                   "board_event", "board_event_inner", "player_event", "player_event_inner", 
                   "you_event", "you_event_inner", "action", "action_buff", 
                   "action_deal_damage", "counter_target", "action_put_counter", 
                   "action_stat_change", "region_derived_quantity", "lifetime", 
                   "counter", "stat", "evergreen_keyword", "keyword", "mod", 
                   "graft", "augment", "affinity", "signed_int", "amount", 
                   "amount_item", "unit_derived_quantity" ]

    EOF = Token.EOF
    T__0=1
    T__1=2
    T__2=3
    T__3=4
    T__4=5
    T__5=6
    T__6=7
    T__7=8
    T__8=9
    T__9=10
    T__10=11
    T__11=12
    T__12=13
    T__13=14
    T__14=15
    T__15=16
    T__16=17
    T__17=18
    T__18=19
    T__19=20
    T__20=21
    T__21=22
    T__22=23
    T__23=24
    T__24=25
    T__25=26
    T__26=27
    T__27=28
    T__28=29
    T__29=30
    T__30=31
    T__31=32
    T__32=33
    T__33=34
    T__34=35
    T__35=36
    T__36=37
    T__37=38
    T__38=39
    T__39=40
    T__40=41
    T__41=42
    T__42=43
    T__43=44
    T__44=45
    T__45=46
    T__46=47
    T__47=48
    T__48=49
    T__49=50
    T__50=51
    T__51=52
    T__52=53
    T__53=54
    T__54=55
    T__55=56
    T__56=57
    T__57=58
    T__58=59
    T__59=60
    T__60=61
    T__61=62
    T__62=63
    T__63=64
    T__64=65
    T__65=66
    T__66=67
    T__67=68
    T__68=69
    T__69=70
    T__70=71
    T__71=72
    T__72=73
    T__73=74
    T__74=75
    T__75=76
    T__76=77
    T__77=78
    T__78=79
    T__79=80
    T__80=81
    T__81=82
    T__82=83
    T__83=84
    T__84=85
    T__85=86
    T__86=87
    T__87=88
    T__88=89
    T__89=90
    T__90=91
    T__91=92
    T__92=93
    T__93=94
    T__94=95
    T__95=96
    T__96=97
    T__97=98
    T__98=99
    T__99=100
    T__100=101
    T__101=102
    NUMBER_WORD=103
    DIGIT=104
    SIGN=105
    COMMENT=106
    META=107
    PSUDO_NEWLINE=108
    SPACE=109
    NEWLINE=110

    def __init__(self, input:TokenStream, output:TextIO = sys.stdout):
        super().__init__(input, output)
        self.checkVersion("4.13.0")
        self._interp = ParserATNSimulator(self, self.atn, self.decisionsToDFA, self.sharedContextCache)
        self._predicates = None




    class ProgContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def EOF(self):
            return self.getToken(CardParser.EOF, 0)

        def effect(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(CardParser.EffectContext)
            else:
                return self.getTypedRuleContext(CardParser.EffectContext,i)


        def getRuleIndex(self):
            return CardParser.RULE_prog

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterProg" ):
                listener.enterProg(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitProg" ):
                listener.exitProg(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitProg" ):
                return visitor.visitProg(self)
            else:
                return visitor.visitChildren(self)




    def prog(self):

        localctx = CardParser.ProgContext(self, self._ctx, self.state)
        self.enterRule(localctx, 0, self.RULE_prog)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 67
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while (((_la) & ~0x3f) == 0 and ((1 << _la) & 576460752303423608) != 0) or ((((_la - 64)) & ~0x3f) == 0 and ((1 << (_la - 64)) & 335564801) != 0):
                self.state = 64
                self.effect()
                self.state = 69
                self._errHandler.sync(self)
                _la = self._input.LA(1)

            self.state = 70
            self.match(CardParser.EOF)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class EffectContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def trigger(self):
            return self.getTypedRuleContext(CardParser.TriggerContext,0)


        def action(self):
            return self.getTypedRuleContext(CardParser.ActionContext,0)


        def mod(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(CardParser.ModContext)
            else:
                return self.getTypedRuleContext(CardParser.ModContext,i)


        def getRuleIndex(self):
            return CardParser.RULE_effect

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterEffect" ):
                listener.enterEffect(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitEffect" ):
                listener.exitEffect(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitEffect" ):
                return visitor.visitEffect(self)
            else:
                return visitor.visitChildren(self)




    def effect(self):

        localctx = CardParser.EffectContext(self, self._ctx, self.state)
        self.enterRule(localctx, 2, self.RULE_effect)
        self._la = 0 # Token type
        try:
            self.state = 86
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [3, 4, 5, 6, 90, 92]:
                self.enterOuterAlt(localctx, 1)
                self.state = 73
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==90 or _la==92:
                    self.state = 72
                    self.mod()


                self.state = 75
                self.trigger()
                self.state = 76
                self.match(CardParser.T__0)
                self.state = 78
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==90 or _la==92:
                    self.state = 77
                    self.mod()


                self.state = 80
                self.action(0)
                self.state = 81
                self.match(CardParser.T__1)
                pass
            elif token in [59, 64, 76, 78]:
                self.enterOuterAlt(localctx, 2)
                self.state = 83
                self.action(0)
                self.state = 84
                self.match(CardParser.T__1)
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TriggerContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return CardParser.RULE_trigger

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)



    class EndOfTurnTriggerContext(TriggerContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a CardParser.TriggerContext
            super().__init__(parser)
            self.copyFrom(ctx)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterEndOfTurnTrigger" ):
                listener.enterEndOfTurnTrigger(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitEndOfTurnTrigger" ):
                listener.exitEndOfTurnTrigger(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitEndOfTurnTrigger" ):
                return visitor.visitEndOfTurnTrigger(self)
            else:
                return visitor.visitChildren(self)


    class EventTriggerContext(TriggerContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a CardParser.TriggerContext
            super().__init__(parser)
            self.trigger_word = None # Token
            self.copyFrom(ctx)

        def event(self):
            return self.getTypedRuleContext(CardParser.EventContext,0)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterEventTrigger" ):
                listener.enterEventTrigger(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitEventTrigger" ):
                listener.exitEventTrigger(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitEventTrigger" ):
                return visitor.visitEventTrigger(self)
            else:
                return visitor.visitChildren(self)


    class AfterCombatTriggerContext(TriggerContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a CardParser.TriggerContext
            super().__init__(parser)
            self.copyFrom(ctx)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAfterCombatTrigger" ):
                listener.enterAfterCombatTrigger(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAfterCombatTrigger" ):
                listener.exitAfterCombatTrigger(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAfterCombatTrigger" ):
                return visitor.visitAfterCombatTrigger(self)
            else:
                return visitor.visitChildren(self)



    def trigger(self):

        localctx = CardParser.TriggerContext(self, self._ctx, self.state)
        self.enterRule(localctx, 4, self.RULE_trigger)
        self._la = 0 # Token type
        try:
            self.state = 92
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [3, 4]:
                localctx = CardParser.EventTriggerContext(self, localctx)
                self.enterOuterAlt(localctx, 1)
                self.state = 88
                localctx.trigger_word = self._input.LT(1)
                _la = self._input.LA(1)
                if not(_la==3 or _la==4):
                    localctx.trigger_word = self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 89
                self.event()
                pass
            elif token in [5]:
                localctx = CardParser.AfterCombatTriggerContext(self, localctx)
                self.enterOuterAlt(localctx, 2)
                self.state = 90
                self.match(CardParser.T__4)
                pass
            elif token in [6]:
                localctx = CardParser.EndOfTurnTriggerContext(self, localctx)
                self.enterOuterAlt(localctx, 3)
                self.state = 91
                self.match(CardParser.T__5)
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class EventContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def unit_event(self):
            return self.getTypedRuleContext(CardParser.Unit_eventContext,0)


        def board_event(self):
            return self.getTypedRuleContext(CardParser.Board_eventContext,0)


        def you_event(self):
            return self.getTypedRuleContext(CardParser.You_eventContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_event

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterEvent" ):
                listener.enterEvent(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitEvent" ):
                listener.exitEvent(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitEvent" ):
                return visitor.visitEvent(self)
            else:
                return visitor.visitChildren(self)




    def event(self):

        localctx = CardParser.EventContext(self, self._ctx, self.state)
        self.enterRule(localctx, 6, self.RULE_event)
        try:
            self.state = 97
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [7, 9, 10, 11]:
                self.enterOuterAlt(localctx, 1)
                self.state = 94
                self.unit_event()
                pass
            elif token in [23, 24]:
                self.enterOuterAlt(localctx, 2)
                self.state = 95
                self.board_event()
                pass
            elif token in [44]:
                self.enterOuterAlt(localctx, 3)
                self.state = 96
                self.you_event()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Unit_eventContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def unit_event_inner(self):
            return self.getTypedRuleContext(CardParser.Unit_event_innerContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_unit_event

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterUnit_event" ):
                listener.enterUnit_event(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitUnit_event" ):
                listener.exitUnit_event(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitUnit_event" ):
                return visitor.visitUnit_event(self)
            else:
                return visitor.visitChildren(self)




    def unit_event(self):

        localctx = CardParser.Unit_eventContext(self, self._ctx, self.state)
        self.enterRule(localctx, 8, self.RULE_unit_event)
        self._la = 0 # Token type
        try:
            self.state = 107
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [7]:
                self.enterOuterAlt(localctx, 1)
                self.state = 99
                self.match(CardParser.T__6)
                self.state = 101
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==8:
                    self.state = 100
                    self.match(CardParser.T__7)


                self.state = 103
                self.unit_event_inner()
                pass
            elif token in [9]:
                self.enterOuterAlt(localctx, 2)
                self.state = 104
                self.match(CardParser.T__8)
                pass
            elif token in [10]:
                self.enterOuterAlt(localctx, 3)
                self.state = 105
                self.match(CardParser.T__9)
                pass
            elif token in [11]:
                self.enterOuterAlt(localctx, 4)
                self.state = 106
                self.match(CardParser.T__10)
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Unit_event_innerContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def amount(self):
            return self.getTypedRuleContext(CardParser.AmountContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_unit_event_inner

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterUnit_event_inner" ):
                listener.enterUnit_event_inner(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitUnit_event_inner" ):
                listener.exitUnit_event_inner(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitUnit_event_inner" ):
                return visitor.visitUnit_event_inner(self)
            else:
                return visitor.visitChildren(self)




    def unit_event_inner(self):

        localctx = CardParser.Unit_event_innerContext(self, self._ctx, self.state)
        self.enterRule(localctx, 10, self.RULE_unit_event_inner)
        try:
            self.state = 122
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [12]:
                self.enterOuterAlt(localctx, 1)
                self.state = 109
                self.match(CardParser.T__11)
                pass
            elif token in [13]:
                self.enterOuterAlt(localctx, 2)
                self.state = 110
                self.match(CardParser.T__12)
                pass
            elif token in [14]:
                self.enterOuterAlt(localctx, 3)
                self.state = 111
                self.match(CardParser.T__13)
                pass
            elif token in [15]:
                self.enterOuterAlt(localctx, 4)
                self.state = 112
                self.match(CardParser.T__14)
                pass
            elif token in [16]:
                self.enterOuterAlt(localctx, 5)
                self.state = 113
                self.match(CardParser.T__15)
                pass
            elif token in [17]:
                self.enterOuterAlt(localctx, 6)
                self.state = 114
                self.match(CardParser.T__16)
                pass
            elif token in [18]:
                self.enterOuterAlt(localctx, 7)
                self.state = 115
                self.match(CardParser.T__17)
                pass
            elif token in [19]:
                self.enterOuterAlt(localctx, 8)
                self.state = 116
                self.match(CardParser.T__18)
                pass
            elif token in [20]:
                self.enterOuterAlt(localctx, 9)
                self.state = 117
                self.match(CardParser.T__19)
                self.state = 118
                self.amount()
                self.state = 119
                self.match(CardParser.T__20)
                pass
            elif token in [22]:
                self.enterOuterAlt(localctx, 10)
                self.state = 121
                self.match(CardParser.T__21)
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Board_eventContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.subject = None # Token
            self.inner = None # Board_event_innerContext

        def board_event_inner(self):
            return self.getTypedRuleContext(CardParser.Board_event_innerContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_board_event

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterBoard_event" ):
                listener.enterBoard_event(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitBoard_event" ):
                listener.exitBoard_event(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBoard_event" ):
                return visitor.visitBoard_event(self)
            else:
                return visitor.visitChildren(self)




    def board_event(self):

        localctx = CardParser.Board_eventContext(self, self._ctx, self.state)
        self.enterRule(localctx, 12, self.RULE_board_event)
        try:
            self.state = 128
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [23]:
                self.enterOuterAlt(localctx, 1)
                self.state = 124
                localctx.subject = self.match(CardParser.T__22)
                self.state = 125
                localctx.inner = self.board_event_inner()
                pass
            elif token in [24]:
                self.enterOuterAlt(localctx, 2)
                self.state = 126
                localctx.subject = self.match(CardParser.T__23)
                self.state = 127
                localctx.inner = self.board_event_inner()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Board_event_innerContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.type_ = None # Token
            self.event_ = None # Token


        def getRuleIndex(self):
            return CardParser.RULE_board_event_inner

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterBoard_event_inner" ):
                listener.enterBoard_event_inner(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitBoard_event_inner" ):
                listener.exitBoard_event_inner(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBoard_event_inner" ):
                return visitor.visitBoard_event_inner(self)
            else:
                return visitor.visitChildren(self)




    def board_event_inner(self):

        localctx = CardParser.Board_event_innerContext(self, self._ctx, self.state)
        self.enterRule(localctx, 14, self.RULE_board_event_inner)
        try:
            self.state = 146
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,10,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 130
                localctx.type_ = self.match(CardParser.T__24)
                self.state = 131
                localctx.event_ = self.match(CardParser.T__25)
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 132
                localctx.type_ = self.match(CardParser.T__26)
                self.state = 133
                localctx.event_ = self.match(CardParser.T__27)
                pass

            elif la_ == 3:
                self.enterOuterAlt(localctx, 3)
                self.state = 134
                localctx.type_ = self.match(CardParser.T__28)
                self.state = 135
                localctx.event_ = self.match(CardParser.T__27)
                pass

            elif la_ == 4:
                self.enterOuterAlt(localctx, 4)
                self.state = 136
                localctx.type_ = self.match(CardParser.T__29)
                self.state = 137
                localctx.event_ = self.match(CardParser.T__30)
                pass

            elif la_ == 5:
                self.enterOuterAlt(localctx, 5)
                self.state = 138
                localctx.type_ = self.match(CardParser.T__31)
                self.state = 139
                localctx.event_ = self.match(CardParser.T__27)
                pass

            elif la_ == 6:
                self.enterOuterAlt(localctx, 6)
                self.state = 140
                localctx.type_ = self.match(CardParser.T__32)
                self.state = 141
                localctx.event_ = self.match(CardParser.T__30)
                pass

            elif la_ == 7:
                self.enterOuterAlt(localctx, 7)
                self.state = 142
                localctx.type_ = self.match(CardParser.T__32)
                self.state = 143
                localctx.event_ = self.match(CardParser.T__33)
                pass

            elif la_ == 8:
                self.enterOuterAlt(localctx, 8)
                self.state = 144
                localctx.type_ = self.match(CardParser.T__34)
                self.state = 145
                localctx.event_ = self.match(CardParser.T__35)
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Player_eventContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.subject = None # Token
            self.inner = None # Player_event_innerContext

        def player_event_inner(self):
            return self.getTypedRuleContext(CardParser.Player_event_innerContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_player_event

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterPlayer_event" ):
                listener.enterPlayer_event(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitPlayer_event" ):
                listener.exitPlayer_event(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitPlayer_event" ):
                return visitor.visitPlayer_event(self)
            else:
                return visitor.visitChildren(self)




    def player_event(self):

        localctx = CardParser.Player_eventContext(self, self._ctx, self.state)
        self.enterRule(localctx, 16, self.RULE_player_event)
        try:
            self.state = 152
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [37]:
                self.enterOuterAlt(localctx, 1)
                self.state = 148
                localctx.subject = self.match(CardParser.T__36)
                self.state = 149
                localctx.inner = self.player_event_inner()
                pass
            elif token in [38]:
                self.enterOuterAlt(localctx, 2)
                self.state = 150
                localctx.subject = self.match(CardParser.T__37)
                self.state = 151
                localctx.inner = self.player_event_inner()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Player_event_innerContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return CardParser.RULE_player_event_inner

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterPlayer_event_inner" ):
                listener.enterPlayer_event_inner(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitPlayer_event_inner" ):
                listener.exitPlayer_event_inner(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitPlayer_event_inner" ):
                return visitor.visitPlayer_event_inner(self)
            else:
                return visitor.visitChildren(self)




    def player_event_inner(self):

        localctx = CardParser.Player_event_innerContext(self, self._ctx, self.state)
        self.enterRule(localctx, 18, self.RULE_player_event_inner)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 154
            _la = self._input.LA(1)
            if not((((_la) & ~0x3f) == 0 and ((1 << _la) & 17042430230528) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class You_eventContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.subject = None # Token
            self.inner = None # You_event_innerContext

        def you_event_inner(self):
            return self.getTypedRuleContext(CardParser.You_event_innerContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_you_event

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterYou_event" ):
                listener.enterYou_event(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitYou_event" ):
                listener.exitYou_event(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitYou_event" ):
                return visitor.visitYou_event(self)
            else:
                return visitor.visitChildren(self)




    def you_event(self):

        localctx = CardParser.You_eventContext(self, self._ctx, self.state)
        self.enterRule(localctx, 20, self.RULE_you_event)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 156
            localctx.subject = self.match(CardParser.T__43)
            self.state = 157
            localctx.inner = self.you_event_inner()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class You_event_innerContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return CardParser.RULE_you_event_inner

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterYou_event_inner" ):
                listener.enterYou_event_inner(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitYou_event_inner" ):
                listener.exitYou_event_inner(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitYou_event_inner" ):
                return visitor.visitYou_event_inner(self)
            else:
                return visitor.visitChildren(self)




    def you_event_inner(self):

        localctx = CardParser.You_event_innerContext(self, self._ctx, self.state)
        self.enterRule(localctx, 22, self.RULE_you_event_inner)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 159
            _la = self._input.LA(1)
            if not((((_la) & ~0x3f) == 0 and ((1 << _la) & 288195191779622912) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ActionContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def action_put_counter(self):
            return self.getTypedRuleContext(CardParser.Action_put_counterContext,0)


        def action_buff(self):
            return self.getTypedRuleContext(CardParser.Action_buffContext,0)


        def action_stat_change(self):
            return self.getTypedRuleContext(CardParser.Action_stat_changeContext,0)


        def action_deal_damage(self):
            return self.getTypedRuleContext(CardParser.Action_deal_damageContext,0)


        def action(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(CardParser.ActionContext)
            else:
                return self.getTypedRuleContext(CardParser.ActionContext,i)


        def getRuleIndex(self):
            return CardParser.RULE_action

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAction" ):
                listener.enterAction(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAction" ):
                listener.exitAction(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAction" ):
                return visitor.visitAction(self)
            else:
                return visitor.visitChildren(self)



    def action(self, _p:int=0):
        _parentctx = self._ctx
        _parentState = self.state
        localctx = CardParser.ActionContext(self, self._ctx, _parentState)
        _prevctx = localctx
        _startState = 24
        self.enterRecursionRule(localctx, 24, self.RULE_action, _p)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 166
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,12,self._ctx)
            if la_ == 1:
                self.state = 162
                self.action_put_counter()
                pass

            elif la_ == 2:
                self.state = 163
                self.action_buff()
                pass

            elif la_ == 3:
                self.state = 164
                self.action_stat_change()
                pass

            elif la_ == 4:
                self.state = 165
                self.action_deal_damage()
                pass


            self._ctx.stop = self._input.LT(-1)
            self.state = 173
            self._errHandler.sync(self)
            _alt = self._interp.adaptivePredict(self._input,13,self._ctx)
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt==1:
                    if self._parseListeners is not None:
                        self.triggerExitRuleEvent()
                    _prevctx = localctx
                    localctx = CardParser.ActionContext(self, _parentctx, _parentState)
                    self.pushNewRecursionContext(localctx, _startState, self.RULE_action)
                    self.state = 168
                    if not self.precpred(self._ctx, 1):
                        from antlr4.error.Errors import FailedPredicateException
                        raise FailedPredicateException(self, "self.precpred(self._ctx, 1)")
                    self.state = 169
                    self.match(CardParser.T__57)
                    self.state = 170
                    self.action(2) 
                self.state = 175
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,13,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.unrollRecursionContexts(_parentctx)
        return localctx


    class Action_buffContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.buff_target = None # Token
            self.derived_quantity = None # Token

        def stat(self):
            return self.getTypedRuleContext(CardParser.StatContext,0)


        def evergreen_keyword(self):
            return self.getTypedRuleContext(CardParser.Evergreen_keywordContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_action_buff

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAction_buff" ):
                listener.enterAction_buff(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAction_buff" ):
                listener.exitAction_buff(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAction_buff" ):
                return visitor.visitAction_buff(self)
            else:
                return visitor.visitChildren(self)




    def action_buff(self):

        localctx = CardParser.Action_buffContext(self, self._ctx, self.state)
        self.enterRule(localctx, 26, self.RULE_action_buff)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 176
            self.match(CardParser.T__58)
            self.state = 177
            localctx.buff_target = self._input.LT(1)
            _la = self._input.LA(1)
            if not(_la==32 or _la==33):
                localctx.buff_target = self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
            self.state = 178
            self.match(CardParser.T__59)
            self.state = 179
            self.stat()
            self.state = 181
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==61:
                self.state = 180
                localctx.derived_quantity = self.match(CardParser.T__60)


            self.state = 185
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==62:
                self.state = 183
                self.match(CardParser.T__61)
                self.state = 184
                self.evergreen_keyword()


            self.state = 187
            self.match(CardParser.T__62)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Action_deal_damageContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def unit_derived_quantity(self):
            return self.getTypedRuleContext(CardParser.Unit_derived_quantityContext,0)


        def DIGIT(self):
            return self.getToken(CardParser.DIGIT, 0)

        def getRuleIndex(self):
            return CardParser.RULE_action_deal_damage

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAction_deal_damage" ):
                listener.enterAction_deal_damage(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAction_deal_damage" ):
                listener.exitAction_deal_damage(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAction_deal_damage" ):
                return visitor.visitAction_deal_damage(self)
            else:
                return visitor.visitChildren(self)




    def action_deal_damage(self):

        localctx = CardParser.Action_deal_damageContext(self, self._ctx, self.state)
        self.enterRule(localctx, 28, self.RULE_action_deal_damage)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 189
            self.match(CardParser.T__63)
            self.state = 194
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [65, 104]:
                self.state = 190
                _la = self._input.LA(1)
                if not(_la==65 or _la==104):
                    self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 191
                self.match(CardParser.T__65)
                pass
            elif token in [66]:
                self.state = 192
                self.match(CardParser.T__65)
                self.state = 193
                self.unit_derived_quantity()
                pass
            else:
                raise NoViableAltException(self)

            self.state = 196
            self.match(CardParser.T__66)

            self.state = 197
            self.match(CardParser.T__67)
            self.state = 198
            _la = self._input.LA(1)
            if not(((((_la - 32)) & ~0x3f) == 0 and ((1 << (_la - 32)) & 412316860417) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
            self.state = 201
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,17,self._ctx)
            if la_ == 1:
                self.state = 200
                self.match(CardParser.T__70)


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Counter_targetContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.self_target = None # Token
            self.target = None # Token
            self.target_each = None # Token

        def region_derived_quantity(self):
            return self.getTypedRuleContext(CardParser.Region_derived_quantityContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_counter_target

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterCounter_target" ):
                listener.enterCounter_target(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitCounter_target" ):
                listener.exitCounter_target(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitCounter_target" ):
                return visitor.visitCounter_target(self)
            else:
                return visitor.visitChildren(self)




    def counter_target(self):

        localctx = CardParser.Counter_targetContext(self, self._ctx, self.state)
        self.enterRule(localctx, 30, self.RULE_counter_target)
        self._la = 0 # Token type
        try:
            self.state = 211
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [72]:
                self.enterOuterAlt(localctx, 1)
                self.state = 203
                localctx.self_target = self.match(CardParser.T__71)
                pass
            elif token in [59]:
                self.enterOuterAlt(localctx, 2)
                self.state = 204
                self.match(CardParser.T__58)
                self.state = 205
                localctx.target = self._input.LT(1)
                _la = self._input.LA(1)
                if not(_la==32 or _la==33):
                    localctx.target = self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 207
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,18,self._ctx)
                if la_ == 1:
                    self.state = 206
                    self.region_derived_quantity()


                pass
            elif token in [68]:
                self.enterOuterAlt(localctx, 3)
                self.state = 209
                self.match(CardParser.T__67)
                self.state = 210
                localctx.target_each = self._input.LT(1)
                _la = self._input.LA(1)
                if not(((((_la - 32)) & ~0x3f) == 0 and ((1 << (_la - 32)) & 15393162788865) != 0)):
                    localctx.target_each = self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Action_put_counterContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def amount_item(self):
            return self.getTypedRuleContext(CardParser.Amount_itemContext,0)


        def counter(self):
            return self.getTypedRuleContext(CardParser.CounterContext,0)


        def counter_target(self):
            return self.getTypedRuleContext(CardParser.Counter_targetContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_action_put_counter

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAction_put_counter" ):
                listener.enterAction_put_counter(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAction_put_counter" ):
                listener.exitAction_put_counter(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAction_put_counter" ):
                return visitor.visitAction_put_counter(self)
            else:
                return visitor.visitChildren(self)




    def action_put_counter(self):

        localctx = CardParser.Action_put_counterContext(self, self._ctx, self.state)
        self.enterRule(localctx, 32, self.RULE_action_put_counter)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 213
            self.match(CardParser.T__75)
            self.state = 214
            self.amount_item()
            self.state = 215
            self.counter()
            self.state = 216
            self.match(CardParser.T__76)
            self.state = 217
            self.counter_target()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Action_stat_changeContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def lifetime(self):
            return self.getTypedRuleContext(CardParser.LifetimeContext,0)


        def stat(self):
            return self.getTypedRuleContext(CardParser.StatContext,0)


        def evergreen_keyword(self):
            return self.getTypedRuleContext(CardParser.Evergreen_keywordContext,0)


        def region_derived_quantity(self):
            return self.getTypedRuleContext(CardParser.Region_derived_quantityContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_action_stat_change

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAction_stat_change" ):
                listener.enterAction_stat_change(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAction_stat_change" ):
                listener.exitAction_stat_change(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAction_stat_change" ):
                return visitor.visitAction_stat_change(self)
            else:
                return visitor.visitChildren(self)




    def action_stat_change(self):

        localctx = CardParser.Action_stat_changeContext(self, self._ctx, self.state)
        self.enterRule(localctx, 34, self.RULE_action_stat_change)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 223
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [59]:
                self.state = 219
                self.match(CardParser.T__58)
                self.state = 220
                _la = self._input.LA(1)
                if not(_la==32 or _la==33):
                    self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 221
                self.match(CardParser.T__59)
                pass
            elif token in [78]:
                self.state = 222
                self.match(CardParser.T__77)
                pass
            else:
                raise NoViableAltException(self)

            self.state = 231
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,21,self._ctx)
            if la_ == 1:
                self.state = 225
                self.stat()
                self.state = 226
                self.match(CardParser.T__61)
                self.state = 227
                self.evergreen_keyword()
                pass

            elif la_ == 2:
                self.state = 229
                self.stat()
                pass

            elif la_ == 3:
                self.state = 230
                self.evergreen_keyword()
                pass


            self.state = 234
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==79:
                self.state = 233
                self.region_derived_quantity()


            self.state = 236
            self.lifetime()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Region_derived_quantityContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def affinity(self):
            return self.getTypedRuleContext(CardParser.AffinityContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_region_derived_quantity

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterRegion_derived_quantity" ):
                listener.enterRegion_derived_quantity(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitRegion_derived_quantity" ):
                listener.exitRegion_derived_quantity(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitRegion_derived_quantity" ):
                return visitor.visitRegion_derived_quantity(self)
            else:
                return visitor.visitChildren(self)




    def region_derived_quantity(self):

        localctx = CardParser.Region_derived_quantityContext(self, self._ctx, self.state)
        self.enterRule(localctx, 36, self.RULE_region_derived_quantity)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 238
            self.match(CardParser.T__78)
            self.state = 241
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [80]:
                self.state = 239
                self.match(CardParser.T__79)
                pass
            elif token in [94]:
                self.state = 240
                self.affinity()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class LifetimeContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return CardParser.RULE_lifetime

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterLifetime" ):
                listener.enterLifetime(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitLifetime" ):
                listener.exitLifetime(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitLifetime" ):
                return visitor.visitLifetime(self)
            else:
                return visitor.visitChildren(self)




    def lifetime(self):

        localctx = CardParser.LifetimeContext(self, self._ctx, self.state)
        self.enterRule(localctx, 38, self.RULE_lifetime)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 243
            self.match(CardParser.T__62)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class CounterContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def stat(self):
            return self.getTypedRuleContext(CardParser.StatContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_counter

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterCounter" ):
                listener.enterCounter(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitCounter" ):
                listener.exitCounter(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitCounter" ):
                return visitor.visitCounter(self)
            else:
                return visitor.visitChildren(self)




    def counter(self):

        localctx = CardParser.CounterContext(self, self._ctx, self.state)
        self.enterRule(localctx, 40, self.RULE_counter)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 245
            self.stat()
            self.state = 246
            self.match(CardParser.T__80)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class StatContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.power = None # Signed_intContext
            self.defence = None # Signed_intContext

        def signed_int(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(CardParser.Signed_intContext)
            else:
                return self.getTypedRuleContext(CardParser.Signed_intContext,i)


        def getRuleIndex(self):
            return CardParser.RULE_stat

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterStat" ):
                listener.enterStat(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitStat" ):
                listener.exitStat(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitStat" ):
                return visitor.visitStat(self)
            else:
                return visitor.visitChildren(self)




    def stat(self):

        localctx = CardParser.StatContext(self, self._ctx, self.state)
        self.enterRule(localctx, 42, self.RULE_stat)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 248
            localctx.power = self.signed_int()
            self.state = 249
            self.match(CardParser.T__81)
            self.state = 250
            localctx.defence = self.signed_int()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Evergreen_keywordContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return CardParser.RULE_evergreen_keyword

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterEvergreen_keyword" ):
                listener.enterEvergreen_keyword(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitEvergreen_keyword" ):
                listener.exitEvergreen_keyword(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitEvergreen_keyword" ):
                return visitor.visitEvergreen_keyword(self)
            else:
                return visitor.visitChildren(self)




    def evergreen_keyword(self):

        localctx = CardParser.Evergreen_keywordContext(self, self._ctx, self.state)
        self.enterRule(localctx, 44, self.RULE_evergreen_keyword)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 252
            _la = self._input.LA(1)
            if not(_la==83 or _la==84):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class KeywordContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def amount(self):
            return self.getTypedRuleContext(CardParser.AmountContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_keyword

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterKeyword" ):
                listener.enterKeyword(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitKeyword" ):
                listener.exitKeyword(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitKeyword" ):
                return visitor.visitKeyword(self)
            else:
                return visitor.visitChildren(self)




    def keyword(self):

        localctx = CardParser.KeywordContext(self, self._ctx, self.state)
        self.enterRule(localctx, 46, self.RULE_keyword)
        try:
            self.state = 261
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [85]:
                self.enterOuterAlt(localctx, 1)
                self.state = 254
                self.match(CardParser.T__84)
                pass
            elif token in [86]:
                self.enterOuterAlt(localctx, 2)
                self.state = 255
                self.match(CardParser.T__85)
                pass
            elif token in [87]:
                self.enterOuterAlt(localctx, 3)
                self.state = 256
                self.match(CardParser.T__86)
                pass
            elif token in [88]:
                self.enterOuterAlt(localctx, 4)
                self.state = 257
                self.match(CardParser.T__87)
                self.state = 258
                self.amount()
                pass
            elif token in [89]:
                self.enterOuterAlt(localctx, 5)
                self.state = 259
                self.match(CardParser.T__88)
                self.state = 260
                self.amount()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ModContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def graft(self):
            return self.getTypedRuleContext(CardParser.GraftContext,0)


        def augment(self):
            return self.getTypedRuleContext(CardParser.AugmentContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_mod

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterMod" ):
                listener.enterMod(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitMod" ):
                listener.exitMod(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitMod" ):
                return visitor.visitMod(self)
            else:
                return visitor.visitChildren(self)




    def mod(self):

        localctx = CardParser.ModContext(self, self._ctx, self.state)
        self.enterRule(localctx, 48, self.RULE_mod)
        try:
            self.state = 265
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [90]:
                self.enterOuterAlt(localctx, 1)
                self.state = 263
                self.graft()
                pass
            elif token in [92]:
                self.enterOuterAlt(localctx, 2)
                self.state = 264
                self.augment()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class GraftContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.limit = None # AmountContext

        def amount(self):
            return self.getTypedRuleContext(CardParser.AmountContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_graft

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterGraft" ):
                listener.enterGraft(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitGraft" ):
                listener.exitGraft(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitGraft" ):
                return visitor.visitGraft(self)
            else:
                return visitor.visitChildren(self)




    def graft(self):

        localctx = CardParser.GraftContext(self, self._ctx, self.state)
        self.enterRule(localctx, 50, self.RULE_graft)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 267
            self.match(CardParser.T__89)
            self.state = 269
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==103 or _la==104:
                self.state = 268
                localctx.limit = self.amount()


            self.state = 271
            self.match(CardParser.T__90)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class AugmentContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.limit = None # Token


        def getRuleIndex(self):
            return CardParser.RULE_augment

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAugment" ):
                listener.enterAugment(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAugment" ):
                listener.exitAugment(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAugment" ):
                return visitor.visitAugment(self)
            else:
                return visitor.visitChildren(self)




    def augment(self):

        localctx = CardParser.AugmentContext(self, self._ctx, self.state)
        self.enterRule(localctx, 52, self.RULE_augment)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 273
            self.match(CardParser.T__91)
            self.state = 275
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==93:
                self.state = 274
                localctx.limit = self.match(CardParser.T__92)


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class AffinityContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return CardParser.RULE_affinity

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAffinity" ):
                listener.enterAffinity(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAffinity" ):
                listener.exitAffinity(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAffinity" ):
                return visitor.visitAffinity(self)
            else:
                return visitor.visitChildren(self)




    def affinity(self):

        localctx = CardParser.AffinityContext(self, self._ctx, self.state)
        self.enterRule(localctx, 54, self.RULE_affinity)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 277
            self.match(CardParser.T__93)
            self.state = 278
            _la = self._input.LA(1)
            if not(((((_la - 95)) & ~0x3f) == 0 and ((1 << (_la - 95)) & 31) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
            self.state = 279
            self.match(CardParser.T__90)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Signed_intContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def SIGN(self):
            return self.getToken(CardParser.SIGN, 0)

        def DIGIT(self):
            return self.getToken(CardParser.DIGIT, 0)

        def getRuleIndex(self):
            return CardParser.RULE_signed_int

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterSigned_int" ):
                listener.enterSigned_int(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitSigned_int" ):
                listener.exitSigned_int(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitSigned_int" ):
                return visitor.visitSigned_int(self)
            else:
                return visitor.visitChildren(self)




    def signed_int(self):

        localctx = CardParser.Signed_intContext(self, self._ctx, self.state)
        self.enterRule(localctx, 56, self.RULE_signed_int)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 281
            self.match(CardParser.SIGN)
            self.state = 282
            self.match(CardParser.DIGIT)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class AmountContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def DIGIT(self):
            return self.getToken(CardParser.DIGIT, 0)

        def NUMBER_WORD(self):
            return self.getToken(CardParser.NUMBER_WORD, 0)

        def getRuleIndex(self):
            return CardParser.RULE_amount

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAmount" ):
                listener.enterAmount(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAmount" ):
                listener.exitAmount(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAmount" ):
                return visitor.visitAmount(self)
            else:
                return visitor.visitChildren(self)




    def amount(self):

        localctx = CardParser.AmountContext(self, self._ctx, self.state)
        self.enterRule(localctx, 58, self.RULE_amount)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 284
            _la = self._input.LA(1)
            if not(_la==103 or _la==104):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Amount_itemContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def amount(self):
            return self.getTypedRuleContext(CardParser.AmountContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_amount_item

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAmount_item" ):
                listener.enterAmount_item(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAmount_item" ):
                listener.exitAmount_item(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAmount_item" ):
                return visitor.visitAmount_item(self)
            else:
                return visitor.visitChildren(self)




    def amount_item(self):

        localctx = CardParser.Amount_itemContext(self, self._ctx, self.state)
        self.enterRule(localctx, 60, self.RULE_amount_item)
        try:
            self.state = 288
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [23]:
                self.enterOuterAlt(localctx, 1)
                self.state = 286
                self.match(CardParser.T__22)
                pass
            elif token in [103, 104]:
                self.enterOuterAlt(localctx, 2)
                self.state = 287
                self.amount()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Unit_derived_quantityContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def affinity(self):
            return self.getTypedRuleContext(CardParser.AffinityContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_unit_derived_quantity

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterUnit_derived_quantity" ):
                listener.enterUnit_derived_quantity(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitUnit_derived_quantity" ):
                listener.exitUnit_derived_quantity(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitUnit_derived_quantity" ):
                return visitor.visitUnit_derived_quantity(self)
            else:
                return visitor.visitChildren(self)




    def unit_derived_quantity(self):

        localctx = CardParser.Unit_derived_quantityContext(self, self._ctx, self.state)
        self.enterRule(localctx, 62, self.RULE_unit_derived_quantity)
        try:
            self.state = 294
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [100]:
                self.enterOuterAlt(localctx, 1)
                self.state = 290
                self.match(CardParser.T__99)

                self.state = 291
                self.match(CardParser.T__100)
                pass
            elif token in [102]:
                self.enterOuterAlt(localctx, 2)
                self.state = 292
                self.match(CardParser.T__101)
                self.state = 293
                self.affinity()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx



    def sempred(self, localctx:RuleContext, ruleIndex:int, predIndex:int):
        if self._predicates == None:
            self._predicates = dict()
        self._predicates[12] = self.action_sempred
        pred = self._predicates.get(ruleIndex, None)
        if pred is None:
            raise Exception("No predicate with index:" + str(ruleIndex))
        else:
            return pred(localctx, predIndex)

    def action_sempred(self, localctx:ActionContext, predIndex:int):
            if predIndex == 0:
                return self.precpred(self._ctx, 1)
         




