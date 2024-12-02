#! /bin/bash

cobc -c eval-ln.cbl
# compile & link main
cobc -x -o coboldbuild day-2.cbl eval-ln.o

./coboldbuild