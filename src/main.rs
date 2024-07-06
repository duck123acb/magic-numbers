use std::collections::HashMap;
use rand::random;
use rand::Rng;
use rand::thread_rng;
use std::fs::File;
use std::io::Write;
use std::io::Error;

fn hashmap_to_bitboard_array(hashmap: &HashMap<usize, u64>) -> Vec<u64> {
  let mut bitboards = Vec::new();

  for (_, &bitboard) in hashmap.iter() {
      bitboards.push(bitboard);
  }

  bitboards
}
fn generate_sliding_piece_mask(square: &i32, orthagonal: bool, diagonal: bool) -> u64 {
  let piece_bitboard = 1 << square;
  let mut moves = 0;
  let mut directions = Vec::new();

  if orthagonal {
    directions.push(8); // up
    directions.push(-8); // down
    directions.push(1); // left
    directions.push(-1); // right
  }
  if diagonal {
    directions.push(9); // up left
    directions.push(7); // up right
    directions.push(-9); // down right
    directions.push(-7); // down left
  }

  for direction in directions {
    for shift in 1..7 {
      let new_square = if direction > 0 { 
        piece_bitboard << shift * direction
      } else {
        piece_bitboard >> shift * (direction * -1)
      };

      // we only need to go to the second-whatever file because we are treating like everyting is a capture, so we can just add that move later in the move gen function
      if new_square & (0xFF00000000000000 & 0x00000000000000FF & 0x8080808080808080 & 0x0101010101010101) != 0 { // second top rank
        break;
      }
  
      moves |= new_square;
      
    }
  }

  moves
}
fn find_legal_moves(square: &i32, occupation: &u64, orthagonal: bool, diagonal: bool) -> u64 {
  let piece_bitboard = 1 << square;
  let mut moves = 0;
  let mut directions = Vec::new();

  if orthagonal {
    directions.push(8); // up
    directions.push(-8); // down
    directions.push(1); // left
    directions.push(-1); // right
  }
  if diagonal {
    directions.push(9); // up left
    directions.push(7); // up right
    directions.push(-9); // down right
    directions.push(-7); // down left
  }

  for direction in directions {
    for shift in 1..7 {
      let new_square = if direction > 0 { 
        piece_bitboard << shift * direction
      } else {
        piece_bitboard >> shift * (direction * -1)
      };
  
      moves |= new_square;

      if new_square & occupation != 0 {
        break;
      }
      if new_square & (0xFF00000000000000 & 0x00000000000000FF & 0x8080808080808080 & 0x0101010101010101) != 0 { // second top rank
        break;
      }
    }
  }

  moves
}

fn count_bits(bitboard: u64) -> u32 {
  bitboard.count_ones()
}

fn set_occupancy(index: u64, attack_mask: &u64) -> u64 { // shift bits into the mask
  let mut occupancy:u64 = 0;
  let mut bit_index = 0;

  for square in 0..64 {
    if attack_mask & (1 << square) != 0 { // if the square is in the attack mask
      if index & (1 << bit_index) != 0 { // if the current bit in the index is not zero
        occupancy |= 1 << square;
      }
      bit_index += 1;
    }
  }

  occupancy
}
fn generate_occupancies(attack_mask: &u64) -> Vec<u64> {
  let mut occupancies = Vec::new();

  // Iterate through all possible bit combinations within the attack mask
  for index in 0..(1 << count_bits(*attack_mask)) { // 0 to 2^number of bits on
    let occupancy = set_occupancy(index, attack_mask);
    occupancies.push(occupancy);
  }

  occupancies
}

fn find_magic_number(square: i32, orthagonal: bool, diagonal: bool) -> (u64, u64, u32, Vec<u64>) {
  let attack_mask = generate_sliding_piece_mask(&square, orthagonal, diagonal);
  let occupancies = generate_occupancies(&attack_mask);
  let relevant_bits = count_bits(attack_mask);
  let mut rng = thread_rng(); // init the rng

  loop {
    let magic_candidate = random::<u64>() & random::<u64>() & random::<u64>(); // AND 3 different random numbers to reduce the active bits. this is done to hopefully find a smaller candidate
    println!("{}",  magic_candidate); // just to see that the program is actually running

    if count_bits(attack_mask.wrapping_mul(magic_candidate) & 0xFF00000000000000) < 6 { // if the number of bits set to 1 are greater than 6, the candidate is too large
      continue;
    }

    let mut used_attacks = HashMap::new();
    let mut fail = false;

    for occupancy in &occupancies {
      let attack_index = (occupancy.wrapping_mul(magic_candidate) >> (64 - relevant_bits)) as usize; // this is the hash function for the key to the attack. https://analog-hors.github.io/site/magic-bitboards/
      let attacks_bitboard = find_legal_moves(&square, occupancy, orthagonal, diagonal);

      // check for collisions in the hashmap
      if let Some(existing_attack) = used_attacks.get(&attack_index) { // if there is an attack at this index
        if *existing_attack != attacks_bitboard { // if the attack_bitboard is different from the position in the hashmap
          fail = true;
          break;
        }
      } else {
        used_attacks.insert(attack_index, attacks_bitboard);
      }
    }

    if !fail {
      return (attack_mask, magic_candidate, relevant_bits, hashmap_to_bitboard_array(&used_attacks));
    }
  }
}

fn write_to_file(file_path: &str, content: &str) -> Result<(), Error> {
  let mut file = File::create(file_path)?;
  file.write_all(content.as_bytes())?;
  Ok(())
}

fn main() {
  let file_path = "resources/rook_magics.txt";
  let content = "Hello, world!\nThis is written to a file.";

  match write_to_file(file_path, content) {
    Ok(_) => println!("Successfully wrote to {}", file_path),
    Err(e) => eprintln!("Error writing to {}: {}", file_path, e),
  }
  // let mut squares = Vec::new();
  // let mut masks = Vec::new();
  // let mut magics = Vec::new();
  // let mut relevant_bits = Vec::new();
  // let mut attacks = Vec::new();

  // for square in 0..63 {
  //   let rook = (true, false);
  //   let (mask, magic_number, bits, piece_attacks) = find_magic_number(square, rook.0, rook.1);

  //   squares.push(square);
  //   masks.push(mask);
  //   magics.push(magic_number);
  //   relevant_bits.push(bits);
  //   attacks.push(piece_attacks);
  // }
}