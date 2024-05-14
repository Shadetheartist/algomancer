import contextlib
import os
import sys
from antlr4 import *
from gen.CardLexer import CardLexer
from gen.CardParser import CardParser
from visitor import VisitorInterp


def test_file(file):
    with contextlib.redirect_stdout(None):
        with contextlib.redirect_stderr(None):
            input_stream = FileStream('tests/' + file)
            lexer = CardLexer(input_stream)
            stream = CommonTokenStream(lexer)
            parser = CardParser(stream)
            tree = parser.prog()

            if parser.getNumberOfSyntaxErrors() > 0:
                return 'Syntax Error'
            else:
                try:
                    vinterp = VisitorInterp()
                    vinterp.visit(tree)
                except:
                    return "Parse Error"

    return "Ok"


def main(argv):
    ok = []
    err = []

    files = os.listdir('tests')
    files.sort()

    for file in files:
        result = test_file(file)
        if result == "Ok":
            ok.append(file)
        else:
            err.append((file, result))

    def percentage(val):
        return round(len(val) * 100 / len(files), 2)

    print(f"Ok: ({len(ok)}/{len(files)} {percentage(ok)}%)")
    for file in ok:
        print(f'\t{file}: Ok!')
        with open('tests/' + file, 'r') as file_content:
            print(f'\t\t{file_content.read()}')


    print(f"Errors: ({len(err)}/{len(files)} {percentage(err)}%)")
    for (file, result) in err:
        print(f'\t{file}: {result}')
        with open('tests/' + file, 'r') as file_content:
            print(f'\t\t{file_content.read()}')


if __name__ == '__main__':
    main(sys.argv)
