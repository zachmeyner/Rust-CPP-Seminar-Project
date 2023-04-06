#!/bin/bash

#init starting files
echo "input,Rust,C++" > seivetimes.csv
echo "input,Rust,C++" > pitimes.csv


#build seive
(cd ../rust/extended_seive && cargo build --release)
(cd ../cpp/extended_sieve && make clean && make)

(cd ../rust/pi_approx && cargo build --release)
(cd ../cpp/pi_approx && make clean && make)

SUMR=0
SUMC=0



# sum seive 10 times and average the runtimes
# start at 1,000,000 and inc by 500,000 until reach 5,000,000
INC=100000000

while [ $INC -le 500000000 ]
do
    #Rust program for seive
    echo "Rust sieve with $INC"
    for _IT in {1..10}
    do
        (cd ../rust/extended_seive && /usr/bin/time -f %S -o ../../script/out.out cargo run --release $INC > /dev/null)
        RTIM=$(< out.out)
        SUMR=$(echo "$SUMR + $RTIM" | bc)
    done


    #cpp program for seive
    echo "C++ sieve with $INC"
    for _IT in {1..10}
    do
        (cd ../cpp/extended_sieve && /usr/bin/time -f %S -o ../../script/out.out ./sieve $INC > /dev/null)
        CTIM=$(< out.out)
        SUMC=$(echo "$SUMC + $CTIM" | bc)
    done

    AVGR=$(echo "scale=10; $SUMR / 10.0" | bc)
    AVGC=$(echo "scale=10; $SUMC / 10.0" | bc)
    echo "$INC,$AVGR,$AVGC" >> seivetimes.csv
    (( INC+=50000000 ))
done

INC=50000


# Pi approx 10 times and average it
while [ $INC -le 250000 ]
do
    #Rust program for pi
    echo "Rust Pi with $INC"
    for _IT in {1..10}
    do
        (cd ../rust/pi_approx && /usr/bin/time -f %S -o ../../script/out.out cargo run --release $INC > /dev/null)
        RTIM=$(< out.out)
        SUMR=$(echo "$SUMR + $RTIM" | bc)
    done


    #cpp program for pi
    echo "C++ Pi with $INC"
    for _IT in {1..10}
    do
        (cd ../cpp/pi_approx && /usr/bin/time -f %S -o ../../script/out.out ./pi_approx $INC > /dev/null)
        CTIM=$(< out.out)
        SUMC=$(echo "$SUMC + $CTIM" | bc)
    done

    AVGR=$(echo "scale=10; $SUMR / 10.0" | bc)
    AVGC=$(echo "scale=10; $SUMC / 10.0" | bc)
    echo "$INC,$AVGR,$AVGC" >> pitimes.csv
    (( INC+=50000 ))
done
