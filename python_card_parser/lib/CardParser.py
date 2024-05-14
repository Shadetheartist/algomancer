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
        4,1,103,279,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,2,6,
        7,6,2,7,7,7,2,8,7,8,2,9,7,9,2,10,7,10,2,11,7,11,2,12,7,12,2,13,7,
        13,2,14,7,14,2,15,7,15,2,16,7,16,2,17,7,17,2,18,7,18,2,19,7,19,2,
        20,7,20,2,21,7,21,2,22,7,22,2,23,7,23,2,24,7,24,2,25,7,25,2,26,7,
        26,2,27,7,27,1,0,4,0,58,8,0,11,0,12,0,59,1,0,1,0,1,1,3,1,65,8,1,
        1,1,1,1,1,1,3,1,70,8,1,1,1,1,1,1,1,1,1,1,1,1,1,3,1,78,8,1,1,2,1,
        2,1,2,1,2,3,2,84,8,2,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,
        1,3,1,3,3,3,99,8,3,1,4,1,4,3,4,103,8,4,1,4,1,4,1,4,1,4,3,4,109,8,
        4,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,
        5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,1,5,3,5,137,8,5,1,6,1,6,1,
        6,1,6,3,6,143,8,6,1,7,1,7,1,7,1,8,1,8,1,8,3,8,151,8,8,1,9,1,9,1,
        9,1,9,1,9,3,9,158,8,9,1,9,1,9,1,9,5,9,163,8,9,10,9,12,9,166,9,9,
        1,10,1,10,1,10,1,10,1,10,3,10,173,8,10,1,10,1,10,3,10,177,8,10,1,
        10,1,10,1,11,1,11,1,11,1,11,1,11,3,11,186,8,11,1,11,1,11,1,11,1,
        11,1,11,3,11,193,8,11,1,12,1,12,1,12,1,12,3,12,199,8,12,1,13,1,13,
        1,13,1,13,3,13,205,8,13,1,13,1,13,3,13,209,8,13,1,14,1,14,1,14,1,
        14,1,14,1,14,1,15,1,15,1,15,1,15,3,15,221,8,15,1,15,1,15,1,15,1,
        15,1,15,1,15,3,15,229,8,15,1,15,3,15,232,8,15,1,15,1,15,1,16,1,16,
        1,16,3,16,239,8,16,1,17,1,17,1,18,1,18,1,18,1,19,1,19,1,19,1,19,
        1,20,1,20,1,21,1,21,3,21,254,8,21,1,22,1,22,3,22,258,8,22,1,22,1,
        22,1,23,1,23,3,23,264,8,23,1,24,1,24,1,24,1,24,1,25,1,25,1,25,1,
        26,1,26,1,27,1,27,3,27,277,8,27,1,27,0,1,18,28,0,2,4,6,8,10,12,14,
        16,18,20,22,24,26,28,30,32,34,36,38,40,42,44,46,48,50,52,54,0,9,
        1,0,3,4,1,0,44,56,1,0,36,37,2,0,64,64,97,97,3,0,25,25,36,36,68,68,
        2,0,36,36,74,76,1,0,84,85,1,0,91,95,1,0,96,97,305,0,57,1,0,0,0,2,
        77,1,0,0,0,4,83,1,0,0,0,6,98,1,0,0,0,8,108,1,0,0,0,10,136,1,0,0,
        0,12,142,1,0,0,0,14,144,1,0,0,0,16,150,1,0,0,0,18,157,1,0,0,0,20,
        167,1,0,0,0,22,180,1,0,0,0,24,198,1,0,0,0,26,208,1,0,0,0,28,210,
        1,0,0,0,30,220,1,0,0,0,32,235,1,0,0,0,34,240,1,0,0,0,36,242,1,0,
        0,0,38,245,1,0,0,0,40,249,1,0,0,0,42,253,1,0,0,0,44,255,1,0,0,0,
        46,261,1,0,0,0,48,265,1,0,0,0,50,269,1,0,0,0,52,272,1,0,0,0,54,276,
        1,0,0,0,56,58,3,2,1,0,57,56,1,0,0,0,58,59,1,0,0,0,59,57,1,0,0,0,
        59,60,1,0,0,0,60,61,1,0,0,0,61,62,5,0,0,1,62,1,1,0,0,0,63,65,3,42,
        21,0,64,63,1,0,0,0,64,65,1,0,0,0,65,66,1,0,0,0,66,67,3,4,2,0,67,
        69,5,1,0,0,68,70,3,42,21,0,69,68,1,0,0,0,69,70,1,0,0,0,70,71,1,0,
        0,0,71,72,3,18,9,0,72,73,5,2,0,0,73,78,1,0,0,0,74,75,3,18,9,0,75,
        76,5,2,0,0,76,78,1,0,0,0,77,64,1,0,0,0,77,74,1,0,0,0,78,3,1,0,0,
        0,79,80,7,0,0,0,80,84,3,16,8,0,81,84,5,5,0,0,82,84,5,6,0,0,83,79,
        1,0,0,0,83,81,1,0,0,0,83,82,1,0,0,0,84,5,1,0,0,0,85,99,5,7,0,0,86,
        99,5,8,0,0,87,99,5,9,0,0,88,99,5,10,0,0,89,99,5,11,0,0,90,99,5,12,
        0,0,91,99,5,13,0,0,92,99,5,14,0,0,93,94,5,15,0,0,94,95,3,52,26,0,
        95,96,5,16,0,0,96,99,1,0,0,0,97,99,5,17,0,0,98,85,1,0,0,0,98,86,
        1,0,0,0,98,87,1,0,0,0,98,88,1,0,0,0,98,89,1,0,0,0,98,90,1,0,0,0,
        98,91,1,0,0,0,98,92,1,0,0,0,98,93,1,0,0,0,98,97,1,0,0,0,99,7,1,0,
        0,0,100,102,5,18,0,0,101,103,5,19,0,0,102,101,1,0,0,0,102,103,1,
        0,0,0,103,104,1,0,0,0,104,109,3,6,3,0,105,109,5,20,0,0,106,109,5,
        21,0,0,107,109,5,22,0,0,108,100,1,0,0,0,108,105,1,0,0,0,108,106,
        1,0,0,0,108,107,1,0,0,0,109,9,1,0,0,0,110,111,5,23,0,0,111,137,5,
        24,0,0,112,113,5,25,0,0,113,137,5,26,0,0,114,115,5,25,0,0,115,137,
        5,27,0,0,116,117,5,25,0,0,117,137,5,28,0,0,118,119,5,25,0,0,119,
        137,5,29,0,0,120,121,5,25,0,0,121,137,5,30,0,0,122,123,5,31,0,0,
        123,137,5,32,0,0,124,125,5,33,0,0,125,137,5,32,0,0,126,127,5,34,
        0,0,127,137,5,35,0,0,128,129,5,36,0,0,129,137,5,32,0,0,130,131,5,
        37,0,0,131,137,5,35,0,0,132,133,5,37,0,0,133,137,5,38,0,0,134,135,
        5,39,0,0,135,137,5,40,0,0,136,110,1,0,0,0,136,112,1,0,0,0,136,114,
        1,0,0,0,136,116,1,0,0,0,136,118,1,0,0,0,136,120,1,0,0,0,136,122,
        1,0,0,0,136,124,1,0,0,0,136,126,1,0,0,0,136,128,1,0,0,0,136,130,
        1,0,0,0,136,132,1,0,0,0,136,134,1,0,0,0,137,11,1,0,0,0,138,139,5,
        41,0,0,139,143,3,10,5,0,140,141,5,42,0,0,141,143,3,10,5,0,142,138,
        1,0,0,0,142,140,1,0,0,0,143,13,1,0,0,0,144,145,5,43,0,0,145,146,
        7,1,0,0,146,15,1,0,0,0,147,151,3,8,4,0,148,151,3,12,6,0,149,151,
        3,14,7,0,150,147,1,0,0,0,150,148,1,0,0,0,150,149,1,0,0,0,151,17,
        1,0,0,0,152,153,6,9,-1,0,153,158,3,28,14,0,154,158,3,20,10,0,155,
        158,3,30,15,0,156,158,3,22,11,0,157,152,1,0,0,0,157,154,1,0,0,0,
        157,155,1,0,0,0,157,156,1,0,0,0,158,164,1,0,0,0,159,160,10,1,0,0,
        160,161,5,57,0,0,161,163,3,18,9,2,162,159,1,0,0,0,163,166,1,0,0,
        0,164,162,1,0,0,0,164,165,1,0,0,0,165,19,1,0,0,0,166,164,1,0,0,0,
        167,168,5,58,0,0,168,169,7,2,0,0,169,170,5,59,0,0,170,172,3,38,19,
        0,171,173,5,60,0,0,172,171,1,0,0,0,172,173,1,0,0,0,173,176,1,0,0,
        0,174,175,5,61,0,0,175,177,3,40,20,0,176,174,1,0,0,0,176,177,1,0,
        0,0,177,178,1,0,0,0,178,179,5,62,0,0,179,21,1,0,0,0,180,185,5,63,
        0,0,181,182,7,3,0,0,182,186,5,65,0,0,183,184,5,65,0,0,184,186,3,
        24,12,0,185,181,1,0,0,0,185,183,1,0,0,0,186,187,1,0,0,0,187,188,
        5,66,0,0,188,189,5,67,0,0,189,190,7,4,0,0,190,192,1,0,0,0,191,193,
        5,69,0,0,192,191,1,0,0,0,192,193,1,0,0,0,193,23,1,0,0,0,194,195,
        5,70,0,0,195,199,5,71,0,0,196,197,5,72,0,0,197,199,3,48,24,0,198,
        194,1,0,0,0,198,196,1,0,0,0,199,25,1,0,0,0,200,209,5,73,0,0,201,
        202,5,58,0,0,202,204,7,2,0,0,203,205,3,32,16,0,204,203,1,0,0,0,204,
        205,1,0,0,0,205,209,1,0,0,0,206,207,5,67,0,0,207,209,7,5,0,0,208,
        200,1,0,0,0,208,201,1,0,0,0,208,206,1,0,0,0,209,27,1,0,0,0,210,211,
        5,77,0,0,211,212,3,54,27,0,212,213,3,36,18,0,213,214,5,78,0,0,214,
        215,3,26,13,0,215,29,1,0,0,0,216,217,5,58,0,0,217,218,7,2,0,0,218,
        221,5,59,0,0,219,221,5,79,0,0,220,216,1,0,0,0,220,219,1,0,0,0,221,
        228,1,0,0,0,222,223,3,38,19,0,223,224,5,61,0,0,224,225,3,40,20,0,
        225,229,1,0,0,0,226,229,3,38,19,0,227,229,3,40,20,0,228,222,1,0,
        0,0,228,226,1,0,0,0,228,227,1,0,0,0,229,231,1,0,0,0,230,232,3,32,
        16,0,231,230,1,0,0,0,231,232,1,0,0,0,232,233,1,0,0,0,233,234,3,34,
        17,0,234,31,1,0,0,0,235,238,5,80,0,0,236,239,5,81,0,0,237,239,3,
        48,24,0,238,236,1,0,0,0,238,237,1,0,0,0,239,33,1,0,0,0,240,241,5,
        62,0,0,241,35,1,0,0,0,242,243,3,38,19,0,243,244,5,82,0,0,244,37,
        1,0,0,0,245,246,3,50,25,0,246,247,5,83,0,0,247,248,3,50,25,0,248,
        39,1,0,0,0,249,250,7,6,0,0,250,41,1,0,0,0,251,254,3,44,22,0,252,
        254,3,46,23,0,253,251,1,0,0,0,253,252,1,0,0,0,254,43,1,0,0,0,255,
        257,5,86,0,0,256,258,3,52,26,0,257,256,1,0,0,0,257,258,1,0,0,0,258,
        259,1,0,0,0,259,260,5,87,0,0,260,45,1,0,0,0,261,263,5,88,0,0,262,
        264,5,89,0,0,263,262,1,0,0,0,263,264,1,0,0,0,264,47,1,0,0,0,265,
        266,5,90,0,0,266,267,7,7,0,0,267,268,5,87,0,0,268,49,1,0,0,0,269,
        270,5,98,0,0,270,271,5,97,0,0,271,51,1,0,0,0,272,273,7,8,0,0,273,
        53,1,0,0,0,274,277,5,41,0,0,275,277,3,52,26,0,276,274,1,0,0,0,276,
        275,1,0,0,0,277,55,1,0,0,0,28,59,64,69,77,83,98,102,108,136,142,
        150,157,164,172,176,185,192,198,204,208,220,228,231,238,253,257,
        263,276
    ]

class CardParser ( Parser ):

    grammarFileName = "Card.g4"

    atn = ATNDeserializer().deserialize(serializedATN())

    decisionsToDFA = [ DFA(ds, i) for i, ds in enumerate(atn.decisionToState) ]

    sharedContextCache = PredictionContextCache()

    literalNames = [ "<INVALID>", "','", "'.'", "'whenever'", "'when'", 
                     "'after combat'", "'at the end of the turn'", "'dealt damage'", 
                     "'modded or applied as a mod'", "'survive damage'", 
                     "'die'", "'spawn'", "'despawn'", "'attack'", "'block'", 
                     "'attack in a formation of'", "'or more units'", "'become targeted'", 
                     "'i'", "'am'", "'a player plays a nonunit spell targeting me'", 
                     "'you play me'", "'my column deals combat damage'", 
                     "'unit token'", "'is created'", "'player'", "'loses life'", 
                     "'loses life during battle'", "'loses is dealt combat damage'", 
                     "'plays a spell'", "'plays their first spell in this battle'", 
                     "'nontoken unit'", "'dies'", "'nontoken enemy'", "'nontoken ally'", 
                     "'spawns'", "'unit'", "'ally'", "'spawns during battle'", 
                     "'card'", "'enters a player's hand during battle'", 
                     "'a'", "'another'", "'you'", "'play a spell'", "'play a spell during battle'", 
                     "'put a counter on an enemy'", "'create a token'", 
                     "'sacrifice a unit'", "'play a nontoken spell'", "'are dealt combat damage'", 
                     "'deal combat damage to an opponent'", "'put one or more counters on an ally'", 
                     "'play a unit'", "'apply an augment during battle'", 
                     "'do'", "'play a token spell'", "', then'", "'target'", 
                     "'gains'", "'for each of your units'", "'and'", "'until regroup'", 
                     "'i deal'", "'x'", "'damage'", "'to'", "'each'", "'opponent'", 
                     "'for each blocked column'", "'equal to my'", "'defense'", 
                     "'equal to your'", "'me'", "'enemy'", "'of your units'", 
                     "'of the chosen units'", "'put'", "'on'", "'your units gain'", 
                     "'for each of your'", "'units'", "'counter'", "'/'", 
                     "'flying'", "'piercing'", "'[graft'", "']'", "'[augment]'", 
                     "'[once]'", "'['", "'r'", "'b'", "'e'", "'g'", "'m'", 
                     "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                     "<INVALID>", "<INVALID>", "' '" ]

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
                      "NUMBER_WORD", "DIGIT", "SIGN", "COMMENT", "META", 
                      "PSUDO_NEWLINE", "SPACE", "NEWLINE" ]

    RULE_prog = 0
    RULE_effect = 1
    RULE_trigger = 2
    RULE_unit_event_inner = 3
    RULE_unit_event = 4
    RULE_board_event_inner = 5
    RULE_board_event = 6
    RULE_player_event = 7
    RULE_event = 8
    RULE_action = 9
    RULE_action_buff = 10
    RULE_action_deal_damage = 11
    RULE_unit_derived_quantity = 12
    RULE_counter_target = 13
    RULE_action_put_counter = 14
    RULE_action_stat_change = 15
    RULE_board_state_derived_quantity = 16
    RULE_lifetime = 17
    RULE_counter = 18
    RULE_stat = 19
    RULE_keyword = 20
    RULE_mod = 21
    RULE_graft = 22
    RULE_augment = 23
    RULE_affinity = 24
    RULE_signed_int = 25
    RULE_amount = 26
    RULE_amount_item = 27

    ruleNames =  [ "prog", "effect", "trigger", "unit_event_inner", "unit_event", 
                   "board_event_inner", "board_event", "player_event", "event", 
                   "action", "action_buff", "action_deal_damage", "unit_derived_quantity", 
                   "counter_target", "action_put_counter", "action_stat_change", 
                   "board_state_derived_quantity", "lifetime", "counter", 
                   "stat", "keyword", "mod", "graft", "augment", "affinity", 
                   "signed_int", "amount", "amount_item" ]

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
    NUMBER_WORD=96
    DIGIT=97
    SIGN=98
    COMMENT=99
    META=100
    PSUDO_NEWLINE=101
    SPACE=102
    NEWLINE=103

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
            self.state = 57 
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while True:
                self.state = 56
                self.effect()
                self.state = 59 
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if not ((((_la) & ~0x3f) == 0 and ((1 << _la) & -8935141660703063944) != 0) or ((((_la - 77)) & ~0x3f) == 0 and ((1 << (_la - 77)) & 2565) != 0)):
                    break

            self.state = 61
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
            self.state = 77
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [3, 4, 5, 6, 86, 88]:
                self.enterOuterAlt(localctx, 1)
                self.state = 64
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==86 or _la==88:
                    self.state = 63
                    self.mod()


                self.state = 66
                self.trigger()
                self.state = 67
                self.match(CardParser.T__0)
                self.state = 69
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==86 or _la==88:
                    self.state = 68
                    self.mod()


                self.state = 71
                self.action(0)
                self.state = 72
                self.match(CardParser.T__1)
                pass
            elif token in [58, 63, 77, 79]:
                self.enterOuterAlt(localctx, 2)
                self.state = 74
                self.action(0)
                self.state = 75
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
            self.state = 83
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [3, 4]:
                localctx = CardParser.EventTriggerContext(self, localctx)
                self.enterOuterAlt(localctx, 1)
                self.state = 79
                localctx.trigger_word = self._input.LT(1)
                _la = self._input.LA(1)
                if not(_la==3 or _la==4):
                    localctx.trigger_word = self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 80
                self.event()
                pass
            elif token in [5]:
                localctx = CardParser.AfterCombatTriggerContext(self, localctx)
                self.enterOuterAlt(localctx, 2)
                self.state = 81
                self.match(CardParser.T__4)
                pass
            elif token in [6]:
                localctx = CardParser.EndOfTurnTriggerContext(self, localctx)
                self.enterOuterAlt(localctx, 3)
                self.state = 82
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
        self.enterRule(localctx, 6, self.RULE_unit_event_inner)
        try:
            self.state = 98
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [7]:
                self.enterOuterAlt(localctx, 1)
                self.state = 85
                self.match(CardParser.T__6)
                pass
            elif token in [8]:
                self.enterOuterAlt(localctx, 2)
                self.state = 86
                self.match(CardParser.T__7)
                pass
            elif token in [9]:
                self.enterOuterAlt(localctx, 3)
                self.state = 87
                self.match(CardParser.T__8)
                pass
            elif token in [10]:
                self.enterOuterAlt(localctx, 4)
                self.state = 88
                self.match(CardParser.T__9)
                pass
            elif token in [11]:
                self.enterOuterAlt(localctx, 5)
                self.state = 89
                self.match(CardParser.T__10)
                pass
            elif token in [12]:
                self.enterOuterAlt(localctx, 6)
                self.state = 90
                self.match(CardParser.T__11)
                pass
            elif token in [13]:
                self.enterOuterAlt(localctx, 7)
                self.state = 91
                self.match(CardParser.T__12)
                pass
            elif token in [14]:
                self.enterOuterAlt(localctx, 8)
                self.state = 92
                self.match(CardParser.T__13)
                pass
            elif token in [15]:
                self.enterOuterAlt(localctx, 9)
                self.state = 93
                self.match(CardParser.T__14)
                self.state = 94
                self.amount()
                self.state = 95
                self.match(CardParser.T__15)
                pass
            elif token in [17]:
                self.enterOuterAlt(localctx, 10)
                self.state = 97
                self.match(CardParser.T__16)
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
            self.state = 108
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [18]:
                self.enterOuterAlt(localctx, 1)
                self.state = 100
                self.match(CardParser.T__17)
                self.state = 102
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==19:
                    self.state = 101
                    self.match(CardParser.T__18)


                self.state = 104
                self.unit_event_inner()
                pass
            elif token in [20]:
                self.enterOuterAlt(localctx, 2)
                self.state = 105
                self.match(CardParser.T__19)
                pass
            elif token in [21]:
                self.enterOuterAlt(localctx, 3)
                self.state = 106
                self.match(CardParser.T__20)
                pass
            elif token in [22]:
                self.enterOuterAlt(localctx, 4)
                self.state = 107
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
        self.enterRule(localctx, 10, self.RULE_board_event_inner)
        try:
            self.state = 136
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,8,self._ctx)
            if la_ == 1:
                self.enterOuterAlt(localctx, 1)
                self.state = 110
                localctx.type_ = self.match(CardParser.T__22)
                self.state = 111
                localctx.event_ = self.match(CardParser.T__23)
                pass

            elif la_ == 2:
                self.enterOuterAlt(localctx, 2)
                self.state = 112
                localctx.type_ = self.match(CardParser.T__24)
                self.state = 113
                localctx.event_ = self.match(CardParser.T__25)
                pass

            elif la_ == 3:
                self.enterOuterAlt(localctx, 3)
                self.state = 114
                localctx.type_ = self.match(CardParser.T__24)
                self.state = 115
                localctx.event_ = self.match(CardParser.T__26)
                pass

            elif la_ == 4:
                self.enterOuterAlt(localctx, 4)
                self.state = 116
                localctx.type_ = self.match(CardParser.T__24)
                self.state = 117
                localctx.event_ = self.match(CardParser.T__27)
                pass

            elif la_ == 5:
                self.enterOuterAlt(localctx, 5)
                self.state = 118
                localctx.type_ = self.match(CardParser.T__24)
                self.state = 119
                localctx.event_ = self.match(CardParser.T__28)
                pass

            elif la_ == 6:
                self.enterOuterAlt(localctx, 6)
                self.state = 120
                localctx.type_ = self.match(CardParser.T__24)
                self.state = 121
                localctx.event_ = self.match(CardParser.T__29)
                pass

            elif la_ == 7:
                self.enterOuterAlt(localctx, 7)
                self.state = 122
                localctx.type_ = self.match(CardParser.T__30)
                self.state = 123
                localctx.event_ = self.match(CardParser.T__31)
                pass

            elif la_ == 8:
                self.enterOuterAlt(localctx, 8)
                self.state = 124
                localctx.type_ = self.match(CardParser.T__32)
                self.state = 125
                localctx.event_ = self.match(CardParser.T__31)
                pass

            elif la_ == 9:
                self.enterOuterAlt(localctx, 9)
                self.state = 126
                localctx.type_ = self.match(CardParser.T__33)
                self.state = 127
                localctx.event_ = self.match(CardParser.T__34)
                pass

            elif la_ == 10:
                self.enterOuterAlt(localctx, 10)
                self.state = 128
                localctx.type_ = self.match(CardParser.T__35)
                self.state = 129
                localctx.event_ = self.match(CardParser.T__31)
                pass

            elif la_ == 11:
                self.enterOuterAlt(localctx, 11)
                self.state = 130
                localctx.type_ = self.match(CardParser.T__36)
                self.state = 131
                localctx.event_ = self.match(CardParser.T__34)
                pass

            elif la_ == 12:
                self.enterOuterAlt(localctx, 12)
                self.state = 132
                localctx.type_ = self.match(CardParser.T__36)
                self.state = 133
                localctx.event_ = self.match(CardParser.T__37)
                pass

            elif la_ == 13:
                self.enterOuterAlt(localctx, 13)
                self.state = 134
                localctx.type_ = self.match(CardParser.T__38)
                self.state = 135
                localctx.event_ = self.match(CardParser.T__39)
                pass


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
            self.state = 142
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [41]:
                self.enterOuterAlt(localctx, 1)
                self.state = 138
                localctx.subject = self.match(CardParser.T__40)
                self.state = 139
                localctx.inner = self.board_event_inner()
                pass
            elif token in [42]:
                self.enterOuterAlt(localctx, 2)
                self.state = 140
                localctx.subject = self.match(CardParser.T__41)
                self.state = 141
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


    class Player_eventContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.player = None # Token
            self.player_action = None # Token


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
        self.enterRule(localctx, 14, self.RULE_player_event)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 144
            localctx.player = self.match(CardParser.T__42)
            self.state = 145
            localctx.player_action = self._input.LT(1)
            _la = self._input.LA(1)
            if not((((_la) & ~0x3f) == 0 and ((1 << _la) & 144097595889811456) != 0)):
                localctx.player_action = self._errHandler.recoverInline(self)
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


    class EventContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def unit_event(self):
            return self.getTypedRuleContext(CardParser.Unit_eventContext,0)


        def board_event(self):
            return self.getTypedRuleContext(CardParser.Board_eventContext,0)


        def player_event(self):
            return self.getTypedRuleContext(CardParser.Player_eventContext,0)


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
        self.enterRule(localctx, 16, self.RULE_event)
        try:
            self.state = 150
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [18, 20, 21, 22]:
                self.enterOuterAlt(localctx, 1)
                self.state = 147
                self.unit_event()
                pass
            elif token in [41, 42]:
                self.enterOuterAlt(localctx, 2)
                self.state = 148
                self.board_event()
                pass
            elif token in [43]:
                self.enterOuterAlt(localctx, 3)
                self.state = 149
                self.player_event()
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
        _startState = 18
        self.enterRecursionRule(localctx, 18, self.RULE_action, _p)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 157
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,11,self._ctx)
            if la_ == 1:
                self.state = 153
                self.action_put_counter()
                pass

            elif la_ == 2:
                self.state = 154
                self.action_buff()
                pass

            elif la_ == 3:
                self.state = 155
                self.action_stat_change()
                pass

            elif la_ == 4:
                self.state = 156
                self.action_deal_damage()
                pass


            self._ctx.stop = self._input.LT(-1)
            self.state = 164
            self._errHandler.sync(self)
            _alt = self._interp.adaptivePredict(self._input,12,self._ctx)
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt==1:
                    if self._parseListeners is not None:
                        self.triggerExitRuleEvent()
                    _prevctx = localctx
                    localctx = CardParser.ActionContext(self, _parentctx, _parentState)
                    self.pushNewRecursionContext(localctx, _startState, self.RULE_action)
                    self.state = 159
                    if not self.precpred(self._ctx, 1):
                        from antlr4.error.Errors import FailedPredicateException
                        raise FailedPredicateException(self, "self.precpred(self._ctx, 1)")
                    self.state = 160
                    self.match(CardParser.T__56)
                    self.state = 161
                    self.action(2) 
                self.state = 166
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,12,self._ctx)

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


        def keyword(self):
            return self.getTypedRuleContext(CardParser.KeywordContext,0)


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
        self.enterRule(localctx, 20, self.RULE_action_buff)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 167
            self.match(CardParser.T__57)
            self.state = 168
            localctx.buff_target = self._input.LT(1)
            _la = self._input.LA(1)
            if not(_la==36 or _la==37):
                localctx.buff_target = self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
            self.state = 169
            self.match(CardParser.T__58)
            self.state = 170
            self.stat()
            self.state = 172
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==60:
                self.state = 171
                localctx.derived_quantity = self.match(CardParser.T__59)


            self.state = 176
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==61:
                self.state = 174
                self.match(CardParser.T__60)
                self.state = 175
                self.keyword()


            self.state = 178
            self.match(CardParser.T__61)
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
        self.enterRule(localctx, 22, self.RULE_action_deal_damage)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 180
            self.match(CardParser.T__62)
            self.state = 185
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [64, 97]:
                self.state = 181
                _la = self._input.LA(1)
                if not(_la==64 or _la==97):
                    self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 182
                self.match(CardParser.T__64)
                pass
            elif token in [65]:
                self.state = 183
                self.match(CardParser.T__64)
                self.state = 184
                self.unit_derived_quantity()
                pass
            else:
                raise NoViableAltException(self)

            self.state = 187
            self.match(CardParser.T__65)

            self.state = 188
            self.match(CardParser.T__66)
            self.state = 189
            _la = self._input.LA(1)
            if not(((((_la - 25)) & ~0x3f) == 0 and ((1 << (_la - 25)) & 8796093024257) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
            self.state = 192
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,16,self._ctx)
            if la_ == 1:
                self.state = 191
                self.match(CardParser.T__68)


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
        self.enterRule(localctx, 24, self.RULE_unit_derived_quantity)
        try:
            self.state = 198
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [70]:
                self.enterOuterAlt(localctx, 1)
                self.state = 194
                self.match(CardParser.T__69)

                self.state = 195
                self.match(CardParser.T__70)
                pass
            elif token in [72]:
                self.enterOuterAlt(localctx, 2)
                self.state = 196
                self.match(CardParser.T__71)
                self.state = 197
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


    class Counter_targetContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.self_target = None # Token
            self.target = None # Token
            self.target_each = None # Token

        def board_state_derived_quantity(self):
            return self.getTypedRuleContext(CardParser.Board_state_derived_quantityContext,0)


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
        self.enterRule(localctx, 26, self.RULE_counter_target)
        self._la = 0 # Token type
        try:
            self.state = 208
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [73]:
                self.enterOuterAlt(localctx, 1)
                self.state = 200
                localctx.self_target = self.match(CardParser.T__72)
                pass
            elif token in [58]:
                self.enterOuterAlt(localctx, 2)
                self.state = 201
                self.match(CardParser.T__57)
                self.state = 202
                localctx.target = self._input.LT(1)
                _la = self._input.LA(1)
                if not(_la==36 or _la==37):
                    localctx.target = self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 204
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,18,self._ctx)
                if la_ == 1:
                    self.state = 203
                    self.board_state_derived_quantity()


                pass
            elif token in [67]:
                self.enterOuterAlt(localctx, 3)
                self.state = 206
                self.match(CardParser.T__66)
                self.state = 207
                localctx.target_each = self._input.LT(1)
                _la = self._input.LA(1)
                if not(((((_la - 36)) & ~0x3f) == 0 and ((1 << (_la - 36)) & 1924145348609) != 0)):
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
        self.enterRule(localctx, 28, self.RULE_action_put_counter)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 210
            self.match(CardParser.T__76)
            self.state = 211
            self.amount_item()
            self.state = 212
            self.counter()
            self.state = 213
            self.match(CardParser.T__77)
            self.state = 214
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


        def keyword(self):
            return self.getTypedRuleContext(CardParser.KeywordContext,0)


        def board_state_derived_quantity(self):
            return self.getTypedRuleContext(CardParser.Board_state_derived_quantityContext,0)


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
        self.enterRule(localctx, 30, self.RULE_action_stat_change)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 220
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [58]:
                self.state = 216
                self.match(CardParser.T__57)
                self.state = 217
                _la = self._input.LA(1)
                if not(_la==36 or _la==37):
                    self._errHandler.recoverInline(self)
                else:
                    self._errHandler.reportMatch(self)
                    self.consume()
                self.state = 218
                self.match(CardParser.T__58)
                pass
            elif token in [79]:
                self.state = 219
                self.match(CardParser.T__78)
                pass
            else:
                raise NoViableAltException(self)

            self.state = 228
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,21,self._ctx)
            if la_ == 1:
                self.state = 222
                self.stat()
                self.state = 223
                self.match(CardParser.T__60)
                self.state = 224
                self.keyword()
                pass

            elif la_ == 2:
                self.state = 226
                self.stat()
                pass

            elif la_ == 3:
                self.state = 227
                self.keyword()
                pass


            self.state = 231
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==80:
                self.state = 230
                self.board_state_derived_quantity()


            self.state = 233
            self.lifetime()
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Board_state_derived_quantityContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def affinity(self):
            return self.getTypedRuleContext(CardParser.AffinityContext,0)


        def getRuleIndex(self):
            return CardParser.RULE_board_state_derived_quantity

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterBoard_state_derived_quantity" ):
                listener.enterBoard_state_derived_quantity(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitBoard_state_derived_quantity" ):
                listener.exitBoard_state_derived_quantity(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBoard_state_derived_quantity" ):
                return visitor.visitBoard_state_derived_quantity(self)
            else:
                return visitor.visitChildren(self)




    def board_state_derived_quantity(self):

        localctx = CardParser.Board_state_derived_quantityContext(self, self._ctx, self.state)
        self.enterRule(localctx, 32, self.RULE_board_state_derived_quantity)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 235
            self.match(CardParser.T__79)
            self.state = 238
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [81]:
                self.state = 236
                self.match(CardParser.T__80)
                pass
            elif token in [90]:
                self.state = 237
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
        self.enterRule(localctx, 34, self.RULE_lifetime)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 240
            self.match(CardParser.T__61)
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
        self.enterRule(localctx, 36, self.RULE_counter)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 242
            self.stat()
            self.state = 243
            self.match(CardParser.T__81)
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
        self.enterRule(localctx, 38, self.RULE_stat)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 245
            localctx.power = self.signed_int()
            self.state = 246
            self.match(CardParser.T__82)
            self.state = 247
            localctx.defence = self.signed_int()
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
        self.enterRule(localctx, 40, self.RULE_keyword)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 249
            _la = self._input.LA(1)
            if not(_la==84 or _la==85):
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
        self.enterRule(localctx, 42, self.RULE_mod)
        try:
            self.state = 253
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [86]:
                self.enterOuterAlt(localctx, 1)
                self.state = 251
                self.graft()
                pass
            elif token in [88]:
                self.enterOuterAlt(localctx, 2)
                self.state = 252
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
        self.enterRule(localctx, 44, self.RULE_graft)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 255
            self.match(CardParser.T__85)
            self.state = 257
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==96 or _la==97:
                self.state = 256
                localctx.limit = self.amount()


            self.state = 259
            self.match(CardParser.T__86)
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
        self.enterRule(localctx, 46, self.RULE_augment)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 261
            self.match(CardParser.T__87)
            self.state = 263
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            if _la==89:
                self.state = 262
                localctx.limit = self.match(CardParser.T__88)


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
        self.enterRule(localctx, 48, self.RULE_affinity)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 265
            self.match(CardParser.T__89)
            self.state = 266
            _la = self._input.LA(1)
            if not(((((_la - 91)) & ~0x3f) == 0 and ((1 << (_la - 91)) & 31) != 0)):
                self._errHandler.recoverInline(self)
            else:
                self._errHandler.reportMatch(self)
                self.consume()
            self.state = 267
            self.match(CardParser.T__86)
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
        self.enterRule(localctx, 50, self.RULE_signed_int)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 269
            self.match(CardParser.SIGN)
            self.state = 270
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
        self.enterRule(localctx, 52, self.RULE_amount)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 272
            _la = self._input.LA(1)
            if not(_la==96 or _la==97):
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
        self.enterRule(localctx, 54, self.RULE_amount_item)
        try:
            self.state = 276
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [41]:
                self.enterOuterAlt(localctx, 1)
                self.state = 274
                self.match(CardParser.T__40)
                pass
            elif token in [96, 97]:
                self.enterOuterAlt(localctx, 2)
                self.state = 275
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



    def sempred(self, localctx:RuleContext, ruleIndex:int, predIndex:int):
        if self._predicates == None:
            self._predicates = dict()
        self._predicates[9] = self.action_sempred
        pred = self._predicates.get(ruleIndex, None)
        if pred is None:
            raise Exception("No predicate with index:" + str(ruleIndex))
        else:
            return pred(localctx, predIndex)

    def action_sempred(self, localctx:ActionContext, predIndex:int):
            if predIndex == 0:
                return self.precpred(self._ctx, 1)
         




