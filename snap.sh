#!/bin/bash

rm -rf lalrpop-snap/src
cp -r lalrpop/src lalrpop-snap/
rm -f lalrpop-snap/src/parser/lrgrammar.lalrpop
