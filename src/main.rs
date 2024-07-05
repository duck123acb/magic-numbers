fn generate_sliding_piece_mask(square: &i32, orthagonal: bool, diagonal: bool) -> u64 { // returns a bitboard with all the squares it attacks
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
      
      // we only need to go to the second-whatever file because we are treating like everyting is a capture, so we can just add that move later
      if new_square & (0xFF00000000000000 >> 8) != 0 { // second top rank
        break;
      }
      if new_square & (0x00000000000000FF << 8) != 0 { // second bottom rank
        break;
      }
      if new_square & (0x8080808080808080 >> 1) != 0 { // second left file
        break;
      }
      if new_square & (0x0101010101010101 << 1) != 0 { // second right file
        break;
      }
    }
  }

  moves
}
fn main() {
  println!("Hello World!");
}