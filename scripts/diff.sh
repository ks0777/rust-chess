#!/usr/bin/bash
depth=4

fen="r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
output=$(echo -e 'position fen '$fen'\ngo perft '$depth | stockfish)
output=$(echo "$output" | sed '/^$/d')
output=$(echo "$output" | sed '$d')
output=$(echo "$output" | sort)
#echo "$output"


output2=$(../target/release/deps/chess-dd917230f7749151)
output2=$(echo "$output2" | sed '1,10d')
output2=$(echo "$output2" | sed '/^$/d')
output2=$(echo "$output2" | sed '$d')
output2=$(echo "$output2" | sed '$d')
output2=$(echo "$output2" | sed '$d')
output2=$(echo "$output2" | sed '$d')
output2=$(echo "$output2" | sed '$d')
output2=$(echo "$output2" | sed '$d')
output2=$(echo "$output2" | sed '$d')
output2=$(echo "$output2" | sed '$d')
output2=$(echo "$output2" | sort)
#echo "$output2"

vimdiff <(echo "$output") <(echo "$output2")
