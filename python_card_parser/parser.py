import sys
from antlr4 import *
from gen.CardLexer import CardLexer
from gen.CardParser import CardParser
from visitor import VisitorInterp


def main(argv):
    input_stream = FileStream(argv[1])
    lexer = CardLexer(input_stream)
    stream = CommonTokenStream(lexer)
    parser = CardParser(stream)
    tree = parser.prog()

    if parser.getNumberOfSyntaxErrors() > 0:
        print("syntax errors")
    else:
        vinterp = VisitorInterp()
        vinterp.visit(tree)
        print('---')
        for effect in vinterp.card.effects:
            effect.print()
            print('---')


if __name__ == '__main__':
    main(sys.argv)
