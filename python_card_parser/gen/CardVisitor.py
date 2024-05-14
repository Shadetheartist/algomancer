# Generated from Card.g4 by ANTLR 4.13.0
from antlr4 import *
if "." in __name__:
    from .CardParser import CardParser
else:
    from CardParser import CardParser

# This class defines a complete generic visitor for a parse tree produced by CardParser.

class CardVisitor(ParseTreeVisitor):

    # Visit a parse tree produced by CardParser#prog.
    def visitProg(self, ctx:CardParser.ProgContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#effect.
    def visitEffect(self, ctx:CardParser.EffectContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#eventTrigger.
    def visitEventTrigger(self, ctx:CardParser.EventTriggerContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#afterCombatTrigger.
    def visitAfterCombatTrigger(self, ctx:CardParser.AfterCombatTriggerContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#endOfTurnTrigger.
    def visitEndOfTurnTrigger(self, ctx:CardParser.EndOfTurnTriggerContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#event.
    def visitEvent(self, ctx:CardParser.EventContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#unit_event.
    def visitUnit_event(self, ctx:CardParser.Unit_eventContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#unit_event_inner.
    def visitUnit_event_inner(self, ctx:CardParser.Unit_event_innerContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#board_event.
    def visitBoard_event(self, ctx:CardParser.Board_eventContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#board_event_inner.
    def visitBoard_event_inner(self, ctx:CardParser.Board_event_innerContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#player_event.
    def visitPlayer_event(self, ctx:CardParser.Player_eventContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#player_event_inner.
    def visitPlayer_event_inner(self, ctx:CardParser.Player_event_innerContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#you_event.
    def visitYou_event(self, ctx:CardParser.You_eventContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#you_event_inner.
    def visitYou_event_inner(self, ctx:CardParser.You_event_innerContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#action.
    def visitAction(self, ctx:CardParser.ActionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#action_buff.
    def visitAction_buff(self, ctx:CardParser.Action_buffContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#action_deal_damage.
    def visitAction_deal_damage(self, ctx:CardParser.Action_deal_damageContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#counter_target.
    def visitCounter_target(self, ctx:CardParser.Counter_targetContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#action_put_counter.
    def visitAction_put_counter(self, ctx:CardParser.Action_put_counterContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#action_stat_change.
    def visitAction_stat_change(self, ctx:CardParser.Action_stat_changeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#region_derived_quantity.
    def visitRegion_derived_quantity(self, ctx:CardParser.Region_derived_quantityContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#lifetime.
    def visitLifetime(self, ctx:CardParser.LifetimeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#counter.
    def visitCounter(self, ctx:CardParser.CounterContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#stat.
    def visitStat(self, ctx:CardParser.StatContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#evergreen_keyword.
    def visitEvergreen_keyword(self, ctx:CardParser.Evergreen_keywordContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#keyword.
    def visitKeyword(self, ctx:CardParser.KeywordContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#mod.
    def visitMod(self, ctx:CardParser.ModContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#graft.
    def visitGraft(self, ctx:CardParser.GraftContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#augment.
    def visitAugment(self, ctx:CardParser.AugmentContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#affinity.
    def visitAffinity(self, ctx:CardParser.AffinityContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#signed_int.
    def visitSigned_int(self, ctx:CardParser.Signed_intContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#amount.
    def visitAmount(self, ctx:CardParser.AmountContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#amount_item.
    def visitAmount_item(self, ctx:CardParser.Amount_itemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by CardParser#unit_derived_quantity.
    def visitUnit_derived_quantity(self, ctx:CardParser.Unit_derived_quantityContext):
        return self.visitChildren(ctx)



del CardParser