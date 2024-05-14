# Generated from Card.g4 by ANTLR 4.13.0
from antlr4 import *
if "." in __name__:
    from .CardParser import CardParser
else:
    from CardParser import CardParser

# This class defines a complete listener for a parse tree produced by CardParser.
class CardListener(ParseTreeListener):

    # Enter a parse tree produced by CardParser#prog.
    def enterProg(self, ctx:CardParser.ProgContext):
        pass

    # Exit a parse tree produced by CardParser#prog.
    def exitProg(self, ctx:CardParser.ProgContext):
        pass


    # Enter a parse tree produced by CardParser#effect.
    def enterEffect(self, ctx:CardParser.EffectContext):
        pass

    # Exit a parse tree produced by CardParser#effect.
    def exitEffect(self, ctx:CardParser.EffectContext):
        pass


    # Enter a parse tree produced by CardParser#eventTrigger.
    def enterEventTrigger(self, ctx:CardParser.EventTriggerContext):
        pass

    # Exit a parse tree produced by CardParser#eventTrigger.
    def exitEventTrigger(self, ctx:CardParser.EventTriggerContext):
        pass


    # Enter a parse tree produced by CardParser#afterCombatTrigger.
    def enterAfterCombatTrigger(self, ctx:CardParser.AfterCombatTriggerContext):
        pass

    # Exit a parse tree produced by CardParser#afterCombatTrigger.
    def exitAfterCombatTrigger(self, ctx:CardParser.AfterCombatTriggerContext):
        pass


    # Enter a parse tree produced by CardParser#endOfTurnTrigger.
    def enterEndOfTurnTrigger(self, ctx:CardParser.EndOfTurnTriggerContext):
        pass

    # Exit a parse tree produced by CardParser#endOfTurnTrigger.
    def exitEndOfTurnTrigger(self, ctx:CardParser.EndOfTurnTriggerContext):
        pass


    # Enter a parse tree produced by CardParser#event.
    def enterEvent(self, ctx:CardParser.EventContext):
        pass

    # Exit a parse tree produced by CardParser#event.
    def exitEvent(self, ctx:CardParser.EventContext):
        pass


    # Enter a parse tree produced by CardParser#unit_event.
    def enterUnit_event(self, ctx:CardParser.Unit_eventContext):
        pass

    # Exit a parse tree produced by CardParser#unit_event.
    def exitUnit_event(self, ctx:CardParser.Unit_eventContext):
        pass


    # Enter a parse tree produced by CardParser#unit_event_inner.
    def enterUnit_event_inner(self, ctx:CardParser.Unit_event_innerContext):
        pass

    # Exit a parse tree produced by CardParser#unit_event_inner.
    def exitUnit_event_inner(self, ctx:CardParser.Unit_event_innerContext):
        pass


    # Enter a parse tree produced by CardParser#board_event.
    def enterBoard_event(self, ctx:CardParser.Board_eventContext):
        pass

    # Exit a parse tree produced by CardParser#board_event.
    def exitBoard_event(self, ctx:CardParser.Board_eventContext):
        pass


    # Enter a parse tree produced by CardParser#board_event_inner.
    def enterBoard_event_inner(self, ctx:CardParser.Board_event_innerContext):
        pass

    # Exit a parse tree produced by CardParser#board_event_inner.
    def exitBoard_event_inner(self, ctx:CardParser.Board_event_innerContext):
        pass


    # Enter a parse tree produced by CardParser#player_event.
    def enterPlayer_event(self, ctx:CardParser.Player_eventContext):
        pass

    # Exit a parse tree produced by CardParser#player_event.
    def exitPlayer_event(self, ctx:CardParser.Player_eventContext):
        pass


    # Enter a parse tree produced by CardParser#player_event_inner.
    def enterPlayer_event_inner(self, ctx:CardParser.Player_event_innerContext):
        pass

    # Exit a parse tree produced by CardParser#player_event_inner.
    def exitPlayer_event_inner(self, ctx:CardParser.Player_event_innerContext):
        pass


    # Enter a parse tree produced by CardParser#you_event.
    def enterYou_event(self, ctx:CardParser.You_eventContext):
        pass

    # Exit a parse tree produced by CardParser#you_event.
    def exitYou_event(self, ctx:CardParser.You_eventContext):
        pass


    # Enter a parse tree produced by CardParser#you_event_inner.
    def enterYou_event_inner(self, ctx:CardParser.You_event_innerContext):
        pass

    # Exit a parse tree produced by CardParser#you_event_inner.
    def exitYou_event_inner(self, ctx:CardParser.You_event_innerContext):
        pass


    # Enter a parse tree produced by CardParser#action.
    def enterAction(self, ctx:CardParser.ActionContext):
        pass

    # Exit a parse tree produced by CardParser#action.
    def exitAction(self, ctx:CardParser.ActionContext):
        pass


    # Enter a parse tree produced by CardParser#action_buff.
    def enterAction_buff(self, ctx:CardParser.Action_buffContext):
        pass

    # Exit a parse tree produced by CardParser#action_buff.
    def exitAction_buff(self, ctx:CardParser.Action_buffContext):
        pass


    # Enter a parse tree produced by CardParser#action_deal_damage.
    def enterAction_deal_damage(self, ctx:CardParser.Action_deal_damageContext):
        pass

    # Exit a parse tree produced by CardParser#action_deal_damage.
    def exitAction_deal_damage(self, ctx:CardParser.Action_deal_damageContext):
        pass


    # Enter a parse tree produced by CardParser#counter_target.
    def enterCounter_target(self, ctx:CardParser.Counter_targetContext):
        pass

    # Exit a parse tree produced by CardParser#counter_target.
    def exitCounter_target(self, ctx:CardParser.Counter_targetContext):
        pass


    # Enter a parse tree produced by CardParser#action_put_counter.
    def enterAction_put_counter(self, ctx:CardParser.Action_put_counterContext):
        pass

    # Exit a parse tree produced by CardParser#action_put_counter.
    def exitAction_put_counter(self, ctx:CardParser.Action_put_counterContext):
        pass


    # Enter a parse tree produced by CardParser#action_stat_change.
    def enterAction_stat_change(self, ctx:CardParser.Action_stat_changeContext):
        pass

    # Exit a parse tree produced by CardParser#action_stat_change.
    def exitAction_stat_change(self, ctx:CardParser.Action_stat_changeContext):
        pass


    # Enter a parse tree produced by CardParser#region_derived_quantity.
    def enterRegion_derived_quantity(self, ctx:CardParser.Region_derived_quantityContext):
        pass

    # Exit a parse tree produced by CardParser#region_derived_quantity.
    def exitRegion_derived_quantity(self, ctx:CardParser.Region_derived_quantityContext):
        pass


    # Enter a parse tree produced by CardParser#lifetime.
    def enterLifetime(self, ctx:CardParser.LifetimeContext):
        pass

    # Exit a parse tree produced by CardParser#lifetime.
    def exitLifetime(self, ctx:CardParser.LifetimeContext):
        pass


    # Enter a parse tree produced by CardParser#counter.
    def enterCounter(self, ctx:CardParser.CounterContext):
        pass

    # Exit a parse tree produced by CardParser#counter.
    def exitCounter(self, ctx:CardParser.CounterContext):
        pass


    # Enter a parse tree produced by CardParser#stat.
    def enterStat(self, ctx:CardParser.StatContext):
        pass

    # Exit a parse tree produced by CardParser#stat.
    def exitStat(self, ctx:CardParser.StatContext):
        pass


    # Enter a parse tree produced by CardParser#evergreen_keyword.
    def enterEvergreen_keyword(self, ctx:CardParser.Evergreen_keywordContext):
        pass

    # Exit a parse tree produced by CardParser#evergreen_keyword.
    def exitEvergreen_keyword(self, ctx:CardParser.Evergreen_keywordContext):
        pass


    # Enter a parse tree produced by CardParser#keyword.
    def enterKeyword(self, ctx:CardParser.KeywordContext):
        pass

    # Exit a parse tree produced by CardParser#keyword.
    def exitKeyword(self, ctx:CardParser.KeywordContext):
        pass


    # Enter a parse tree produced by CardParser#mod.
    def enterMod(self, ctx:CardParser.ModContext):
        pass

    # Exit a parse tree produced by CardParser#mod.
    def exitMod(self, ctx:CardParser.ModContext):
        pass


    # Enter a parse tree produced by CardParser#graft.
    def enterGraft(self, ctx:CardParser.GraftContext):
        pass

    # Exit a parse tree produced by CardParser#graft.
    def exitGraft(self, ctx:CardParser.GraftContext):
        pass


    # Enter a parse tree produced by CardParser#augment.
    def enterAugment(self, ctx:CardParser.AugmentContext):
        pass

    # Exit a parse tree produced by CardParser#augment.
    def exitAugment(self, ctx:CardParser.AugmentContext):
        pass


    # Enter a parse tree produced by CardParser#affinity.
    def enterAffinity(self, ctx:CardParser.AffinityContext):
        pass

    # Exit a parse tree produced by CardParser#affinity.
    def exitAffinity(self, ctx:CardParser.AffinityContext):
        pass


    # Enter a parse tree produced by CardParser#signed_int.
    def enterSigned_int(self, ctx:CardParser.Signed_intContext):
        pass

    # Exit a parse tree produced by CardParser#signed_int.
    def exitSigned_int(self, ctx:CardParser.Signed_intContext):
        pass


    # Enter a parse tree produced by CardParser#amount.
    def enterAmount(self, ctx:CardParser.AmountContext):
        pass

    # Exit a parse tree produced by CardParser#amount.
    def exitAmount(self, ctx:CardParser.AmountContext):
        pass


    # Enter a parse tree produced by CardParser#amount_item.
    def enterAmount_item(self, ctx:CardParser.Amount_itemContext):
        pass

    # Exit a parse tree produced by CardParser#amount_item.
    def exitAmount_item(self, ctx:CardParser.Amount_itemContext):
        pass


    # Enter a parse tree produced by CardParser#unit_derived_quantity.
    def enterUnit_derived_quantity(self, ctx:CardParser.Unit_derived_quantityContext):
        pass

    # Exit a parse tree produced by CardParser#unit_derived_quantity.
    def exitUnit_derived_quantity(self, ctx:CardParser.Unit_derived_quantityContext):
        pass



del CardParser