#!/bin/bash

input_file=test_input.txt

cat test_input.txt

mkdir -p ./tmp
cp Card.g4 ./tmp
cp $input_file ./tmp
cd ./tmp || exit

antlr4-parse Card.g4 prog $input_file -gui