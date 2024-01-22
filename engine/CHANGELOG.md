## v0.1.0
- transposition table
- piece square tables

Score of v0.1.0 vs v0.0.0: 1582 - 346 - 72  [0.809] 2000 \
&nbsp; v0.1.0 playing White: 774 - 181 - 45  [0.796] 1000 \
&nbsp; White vs Black: 939 - 989 - 72  [0.487] 2000

Elo difference: 250.8 +/- 18.8, LOS: 100.0 %, DrawRatio: 3.6 % \
SPRT: llr 0 (0.0%), lbound -inf, ubound inf

Player: v0.1.0 \
&nbsp; "Draw by 3-fold repetition": 12 \
&nbsp; "Draw by stalemate": 2 \
&nbsp; "Draw by timeout": 58 \
&nbsp; "Loss: Black loses on time": 165 \
&nbsp; "Loss: White loses on time": 181 \
&nbsp; "Win: Black loses on time": 774 \
&nbsp; "Win: White loses on time": 808

STS rating: 749 \
Hash: 128, Threads: 1, MoveTime: 1.0s

## v0.0.0
- board representation
  - legal move generator
  - draw detection
  - checkmate detection
- functional negamax and quiescence search
- full uci compliance
- strength testing
  - cutechess tournament script
  - STS based rating

STS rating 407 \
Hash: 128, Threads: 1, MoveTime: 1.0s
