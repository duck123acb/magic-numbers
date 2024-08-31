# Magic Bitboard Generator
![ICON](https://github.com/user-attachments/assets/7da7b49e-324a-4e3e-825e-91b9e6545c9b)
Generates magic bitboards for rooks and bishops in a chess engine. <br>
The output includes magic numbers, masks, relevant bits, and attack tables, which are stored in a file for later use.

## Features
- Generate masks for rook and bishop moves.
- Compute legal moves for rooks and bishops considering board occupancy.
- Find suitable magic numbers for indexing attack tables.
- Output the computed data to a file for use in a chess engine.

## Dependencies
[rand](https://docs.rs/rand/latest/rand/): Used to generate random numbers.

## Run Locally

Clone the project

```bash
  git clone https://github.com/duck123acb/magic-numbers.git
```

Go to the project directory

```bash
  cd magic-numbers
```

Install dependencies & Run

```bash
  cargo run
```
