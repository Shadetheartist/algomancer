import sys
from antlr4 import *
from gen.CardParser import CardParser
from gen.CardVisitor import CardVisitor
from pprint import pprint


class SequenceTrigger:
    def __init__(self, step, timing):
        self.step = step
        self.at = timing

    def __str__(self):
        return str(self.step)


class GameEventTrigger:
    def __init__(self, entity, action, exclude_self):
        self.entity = entity
        self.action = action
        self.exclude_self = exclude_self

    def __str__(self):
        exclude = ', excluding self' if self.exclude_self else ''
        return f'entity: game({self.entity}{exclude}), action: {self.action}'


class UnitEventTrigger:
    def __init__(self, unit, action):
        self.unit = unit
        self.action = action

    def __str__(self):
        return f'entity: unit({self.unit}), action: {self.action}'


class PlayerEventTrigger:
    def __init__(self, player, reason):
        self.player = player
        self.reason = reason

    def __str__(self):
        return f'entity: player({self.player}), action: {self.reason}'


class ApplyBuffAction:
    def __init__(self, target, stat, keywords):
        self.target = target
        self.stat = stat
        self.keywords = keywords

    def __str__(self):
        keywords = ', ' + ' ,'.join(self.keywords) if self.keywords else None
        return f'target: {self.target}, action: apply_buff({self.stat}{keywords}))'


class ApplyCountersAction:
    def __init__(self, target, counter, quantity):
        self.target = target
        self.counter = counter
        self.quantity = quantity

    def __str__(self):
        return f'target: {self.target}, action: apply_counters({self.counter}, {self.quantity})'


class Effect:
    def __init__(self):
        self.raw = []
        self.trigger_mod = None
        self.action_mod = None
        self.triggers = []
        self.actions = []

    def print(self):

        print(f'"{self.raw}"')
        print('trigger_mod: ' + str(self.trigger_mod))
        print('triggers:')
        for t in self.triggers:
            print('\t' + str(t))

        print('action_mod: ' + str(self.action_mod))
        print('actions:')
        for a in self.actions:
            print('\t' + str(a))


class VisitorInterp(CardVisitor):
    def __init__(self):
        self.effects = []


    def visitEffect(self, ctx:CardParser.EffectContext):
        effect = Effect()
        effect.raw = ctx.getText()

        if ctx.trigger() is not None:
            effect.triggers.append(self.visit(ctx.trigger()))
        else:
            effect.triggers.append('play')

        effect.actions.append(self.visit(ctx.action()))

        effect.trigger_mod = self.visit(ctx.mod(0)) if ctx.mod(0) is not None else None
        effect.action_mod = self.visit(ctx.mod(1)) if ctx.mod(1) is not None else None

        self.effects.append(effect)

        return self.visitChildren(ctx)


    def visitMod(self, ctx:CardParser.ModContext):
        return ctx.getText()


    def visitEventTrigger(self, ctx:CardParser.EventTriggerContext):
        return self.visitChildren(ctx)


    def visitEvent(self, ctx:CardParser.EventContext):
        if ctx.unit_event() is not None:
            unit_event = self.visit(ctx.unit_event())
            return UnitEventTrigger('self', unit_event)

        return self.visitChildren(ctx)


    def visitUnit_event_inner(self, ctx:CardParser.Unit_event_innerContext):
        return ctx.getText()


    def visitPlayer_event(self, ctx:CardParser.Player_eventContext):
        if ctx.player is not None:
            return PlayerEventTrigger(ctx.player.text, ctx.getChild(1).getText())


    def visitAction_buff(self, ctx:CardParser.Action_buffContext):
        target = ctx.buff_target.text
        stat = self.visit(ctx.stat())
        keywords = []
        if ctx.evergreen_keyword():
            keywords.append(self.visit(ctx.evergreen_keyword()))

        return ApplyBuffAction(target, stat, keywords)


    def visitEvergreen_keyword(self, ctx:CardParser.Evergreen_keywordContext):
        return ctx.getText()


    def visitAction_put_counter(self, ctx:CardParser.Action_put_counterContext):
        target = self.visit(ctx.counter_target())
        counter = self.visit(ctx.counter())
        quantity = self.visit(ctx.amount_item())
        return ApplyCountersAction(target, counter, quantity)


    def visitUnit_event(self, ctx:CardParser.Unit_eventContext):
        return self.visitChildren(ctx)

    def visitBoard_event(self, ctx:CardParser.Board_eventContext):
        exclude_self = ctx.subject.text == 'another'
        event = self.visit(ctx.inner)

        return GameEventTrigger(event[0], event[1], exclude_self)

    def visitBoard_event_inner(self, ctx:CardParser.Board_event_innerContext):
        return ctx.type_.text, ctx.event_.text


    def visitCounter_target(self, ctx:CardParser.Counter_targetContext):
        if ctx.self_target is not None:
            return 'self'

        if ctx.target is not None:
            return ctx.target.text

        if ctx.target_each is not None:
            return 'each ' + str(ctx.target_each.text)

    def visitCounter(self, ctx:CardParser.CounterContext):
        counter = self.visit(ctx.stat())
        return counter


    def visitStat(self, ctx:CardParser.StatContext):
        power = self.parse_signed_digit(ctx.power)
        defense = self.parse_signed_digit(ctx.defence)
        return power, defense


    def visitAmount(self, ctx:CardParser.AmountContext):
        if ctx.DIGIT() is not None:
            return int(ctx.DIGIT().getText())
        elif ctx.NUMBER_WORD():
            return self.parse_number_word(ctx.NUMBER_WORD())


    def visitAmount_item(self, ctx:CardParser.Amount_itemContext):
        if ctx.amount() is not None:
            return self.visit(ctx.amount())
        elif ctx.getText() == 'a':
            return 1
        else:
            raise f'amount [{ctx.getText()}] is not parsable'


    @staticmethod
    def parse_signed_digit(ctx: CardParser.Signed_intContext):
        sign = ctx.SIGN().getText()
        digit = ctx.DIGIT().getText()
        digit_int = int(digit)
        if sign == '+':
            return digit_int
        elif sign == '-':
            return digit_int * -1
        else:
            raise 'sign should be + or -'


    @staticmethod
    def parse_number_word(token: CommonTokenStream):
        num_word = token.getText()
        if num_word == 'one':
            return 1
        elif num_word == 'two':
            return 2
        elif num_word == 'three':
            return 3
        elif num_word == 'four':
            return 4
        elif num_word == 'five':
            return 5
        elif num_word == 'six':
            return 6
        elif num_word == 'seven':
            return 7
        elif num_word == 'eight':
            return 8
        elif num_word == 'nine':
            return 9
        elif num_word == 'ten':
            return 10
        else:
            raise 'NUMBER_WORD not valid'