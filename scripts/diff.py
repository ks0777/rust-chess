#!/usr/bin/python3
import subprocess

fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
x = 'echo "position fen ' + fen + '\nd"'
output = subprocess.check_output(x + ' | stockfish')
