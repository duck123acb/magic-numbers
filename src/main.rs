use std::collections::HashMap;
use rand::random;
use std::fs::File;
use std::io::Write;
use std::io::Error;

const ATTACK_ARRAY_SIZE: usize = 4096; // 4096 if its a rook, can be brought down to 512 for the bishops

fn hashmap_to_bitboard_array(hashmap: &HashMap<usize, u64>) -> [u64; ATTACK_ARRAY_SIZE] {
  let mut bitboards = [0; ATTACK_ARRAY_SIZE];

  for &key in hashmap.keys() {
    bitboards[key] = hashmap[&key];
  }

  bitboards
}

fn generate_rook_mask(square: &i32) -> u64 {
  let piece_bitboard = 1 << square;
  let mut moves = 0;
  let mut directions = Vec::new();

  if piece_bitboard & 0x8080808080808080 == 0 {
    directions.push(1); // left
  }
  if piece_bitboard & 0x0101010101010101 == 0 {
    directions.push(-1); // right
  }
  if piece_bitboard & 0xFF00000000000000 == 0 {
    directions.push(8); // up
  }
  if piece_bitboard & 0x00000000000000FF == 0 {
    directions.push(-8); // down
  }
  

  for direction in directions {
    for shift in 1..7 {
      let new_square = if direction > 0 { 
        piece_bitboard << shift * direction
      } else {
        piece_bitboard >> shift * (direction * -1)
      };

      if new_square & 0x8080808080808080 != 0 && direction == 1 {
        break;
      }
      if new_square & 0x0101010101010101 != 0 && direction == -1 {
        break;
      }
      if new_square & 0xFF00000000000000 != 0 && direction == 8 {
        break;
      }
      if new_square & 0x00000000000000FF != 0 && direction == -8 {
        break;
      }

      moves |= new_square;
    }
  }

  moves
}
fn generate_bishop_mask(square: &i32) -> u64 {
  let piece_bitboard = 1 << square;
  let mut moves = 0;
  let mut directions = Vec::new();

  if piece_bitboard & 0xFF00000000000000 == 0 && piece_bitboard & 0x8080808080808080 == 0 {
    directions.push(9); // up left
  }
  if piece_bitboard & 0xFF00000000000000 == 0 && piece_bitboard & 0x0101010101010101 == 0 {
    directions.push(7); // up right
  }
  if piece_bitboard & 0x00000000000000FF == 0 && piece_bitboard & 0x8080808080808080 == 0 {
    directions.push(-7); // down left
  }
  if piece_bitboard & 0x00000000000000FF == 0 && piece_bitboard & 0x0101010101010101 == 0 {
    directions.push(-9); // down right
  }

  for direction in directions {
    for shift in 1..7 {
      let new_square = if direction > 0 { 
        piece_bitboard << shift * direction
      } else {
        piece_bitboard >> shift * (direction * -1)
      };

      if new_square & 0xFF818181818181FF != 0 {
        break;
      }

      moves |= new_square;
    }
  }

  moves
}

fn find_legal_rook_moves(square: &i32, occupancy: &u64) -> u64 {
  let piece_bitboard = 1 << square;
  let mut moves = 0;
  let mut directions = Vec::new();

  if piece_bitboard & 0x8080808080808080 == 0 {
    directions.push(1); // left
  }
  if piece_bitboard & 0x0101010101010101 == 0 {
    directions.push(-1); // right
  }
  if piece_bitboard & 0xFF00000000000000 == 0 {
    directions.push(8); // up
  }
  if piece_bitboard & 0x00000000000000FF == 0 {
    directions.push(-8); // down
  }

  for direction in directions {
    for shift in 1..7 {
      let new_square = if direction > 0 { 
        piece_bitboard << shift * direction
      } else {
        piece_bitboard >> shift * (direction * -1)
      };
    
      moves |= new_square;

      if new_square & occupancy != 0 {
        break;
      }
      if new_square & 0x8080808080808080 != 0 && direction == 1 {
        break;
      }
      if new_square & 0x0101010101010101 != 0 && direction == -1 {
        break;
      }
      if new_square & 0xFF00000000000000 != 0 && direction == 8 {
        break;
      }
      if new_square & 0x00000000000000FF != 0 && direction == -8 {
        break;
      }
    }
  }

  moves
}
fn find_legal_bishop_moves(square: &i32, occupancy: &u64) -> u64 {
  let piece_bitboard = 1 << square;
  let mut moves = 0;
  let mut directions = Vec::new();

  if piece_bitboard & 0xFF00000000000000 == 0 && piece_bitboard & 0x8080808080808080 == 0 {
    directions.push(9); // up left
  }
  if piece_bitboard & 0xFF00000000000000 == 0 && piece_bitboard & 0x0101010101010101 == 0 {
    directions.push(7); // up right
  }
  if piece_bitboard & 0x00000000000000FF == 0 && piece_bitboard & 0x8080808080808080 == 0 {
    directions.push(-7); // down left
  }
  if piece_bitboard & 0x00000000000000FF == 0 && piece_bitboard & 0x0101010101010101 == 0 {
    directions.push(-9); // down right
  }

  for direction in directions {
    for shift in 1..7 {
      let new_square = if direction > 0 { 
        piece_bitboard << shift * direction
      } else {
        piece_bitboard >> shift * (direction * -1)
      };

      moves |= new_square;

      if new_square & occupancy != 0 {
        break;
      }
      if new_square & 0xFF818181818181FF != 0 {
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

fn find_magic_number(square: i32, attack_mask: &u64, is_bishop: bool) -> (u64, u64, u32, [u64; ATTACK_ARRAY_SIZE]) {
  let occupancies = generate_occupancies(&attack_mask);
  let relevant_bits = 64 - count_bits(*attack_mask);

  loop {
    let magic_candidate = random::<u64>() & random::<u64>() & random::<u64>(); // AND 3 different random numbers to reduce the active bits. this is done to hopefully find a smaller candidate

    if count_bits(attack_mask.wrapping_mul(magic_candidate) & 0xFF00000000000000) < 6 { // if the number of bits set to 1 are greater than 6, the candidate is too large
      continue;
    }

    let mut used_attacks = HashMap::new();
    let mut fail = false;

    for occupancy in &occupancies {
      let attack_index = (occupancy.wrapping_mul(magic_candidate) >> relevant_bits) as usize; // this is the hash function for the key to the attack. https://analog-hors.github.io/site/magic-bitboards/
      let attacks_bitboard = if is_bishop { find_legal_bishop_moves(&square, occupancy) } else { find_legal_rook_moves(&square, occupancy) };

      // check for collisions in the hashmap
      if let Some(existing_attack) = used_attacks.get(&attack_index) { // if there is an attack at this index
        if *existing_attack != attacks_bitboard { // if the attack_bitboard is different from the position in the hashmap
          fail = true;
          continue;
        }
      } else {
        used_attacks.insert(attack_index, attacks_bitboard);
      }
    }

    if !fail {
      return (*attack_mask, magic_candidate, relevant_bits, hashmap_to_bitboard_array(&used_attacks));
    }
  }
}

fn write_to_file(file_path: &str, content: &str) -> Result<(), Error> {
  let mut file = File::create(file_path)?;
  file.write_all(content.as_bytes())?;
  Ok(())
}

fn main() {
  let bishop = ATTACK_ARRAY_SIZE == 512;

  let file_path = if bishop { "resources/bishop_magics.txt" } else { "resources/rook_magics.txt" };

  let mut magics = "Magics: [".to_string();
  let mut masks = "Masks: [".to_string();
  let mut relevant_bits = "Relevant Bits: [".to_string();
  let mut attacks = "Attacks: [".to_string();
  
  let mut mask_table = HashMap::new();
  for square in 0..64 {
    let piece_mask = if bishop { generate_bishop_mask(&square) } else { generate_rook_mask(&square) };
    mask_table.insert(square, piece_mask);
  }

  for square in 0..64 {
    let (mask, magic_number, bits, piece_attacks) = find_magic_number(square, &mask_table[&square], bishop);


    magics.push_str(&magic_number.to_string());
    magics.push_str(", ");

    masks.push_str(&mask.to_string());
    masks.push_str(", ");

    relevant_bits.push_str(&bits.to_string());
    relevant_bits.push_str(", ");

    attacks.push_str("[");
    for attack in piece_attacks {
      attacks.push_str(&attack.to_string());
      attacks.push_str(", ");
    }
    attacks.push_str("], ");

    println!("{}", square);
  }

  magics.push_str("]");
  masks.push_str("]");
  relevant_bits.push_str("]");
  attacks.push_str("]");

  let content = magics + "\n" + &masks + "\n" + &relevant_bits + "\n" + &attacks;

  match write_to_file(file_path, content.as_str()) {
    Ok(_) => println!("Successfully wrote to {}", file_path),
    Err(e) => eprintln!("Error writing to {}: {}", file_path, e),
  }
}