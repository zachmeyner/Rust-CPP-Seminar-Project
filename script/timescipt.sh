#!/bin/bash

#init starting files
echo "input,rusttime,cpptime" > seivetimes.csv


#build seive
(cd ../rust/extended_seive && cargo build --release)
(cd ../cpp/extended_sieve && make clean && make)

SUMR=0
SUMC=0

#sum seive 20 times and average the runtimes
#start at 1,000,000 and inc by 500,000 until reach 5,000,000
INC=100000000

while [ $INC -le 500000000 ]
do
    #Rust program for seive
    for _IT in {1..10}
    do
        START=$(date +%s.%N)
        (cd ../rust/extended_seive && cargo run --release $INC > /dev/null)
        DUR=$(echo "$(date +%s.%N) - $START" | bc)
        SUMR=$(echo "$SUMR + $DUR" | bc)
    done


    #cpp program for seive
    for _IT in {1..10}
    do
        START=$(date +%s.%N)
        (cd ../cpp/extended_sieve && ./sieve $INC > /dev/null)
        DUR=$(echo "$(date +%s.%N) - $START" | bc)
        SUMC=$(echo "$SUMC + $DUR" | bc)
    done

    AVGR=$(echo "scale=10; $SUMR / 10.0" | bc)
    AVGC=$(echo "scale=10; $SUMC / 10.0" | bc)
    echo "$INC,$AVGR,$AVGC" >> seivetimes.csv
    (( INC+=50000000 ))
done




