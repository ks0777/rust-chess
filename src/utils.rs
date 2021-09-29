use crate::models::{Figure, FigureType, FigureColor, Field, Board};

fn translate_position_to_index(pos: &str) -> i8 {
    let pos_chars: Vec<char> = pos.chars().collect();
    if pos_chars.len() == 2 {
        let file = (pos_chars[0] as i8) - 97;
        let rank = 8  - ((pos_chars[1] as i8) - 49);
        return file * 8 + rank;
    }
    return -1;
}

pub fn board_from_fen(fen: &str, next_move: &mut FigureColor, en_passant: &mut i8) -> Board {
    let fen_split: Vec<&str> = fen.split(' ').collect();

    let fen_board = fen_split[0];
    let fen_active = fen_split[1];
    let fen_castle = fen_split[2];
    let fen_en_passant = fen_split[3];

    let mut i = 0;
    let mut board: Board = Board { fields: [Field {figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false}; 64] };

    for c in fen_board.chars() {
        if i > 63 { break; }
        let color = if c.is_lowercase() { FigureColor::BLACK } else { FigureColor::WHITE };
        if c.is_digit(10) {
            let offset = c.to_digit(10);
            match offset {
                Some(x) => i += x,
                _ => ()
            }
        } else if c.is_ascii_alphanumeric() { 
            let figure: Vec<_> = c.to_lowercase().collect();
            match figure[0] {
                'k' => board.fields[i as usize] = Field { figure_type: FigureType::KING, figure_color: color, dirty: false },
                'q' => board.fields[i as usize] = Field { figure_type: FigureType::QUEEN, figure_color: color, dirty: false },
                'r' => board.fields[i as usize] = Field { figure_type: FigureType::ROOK, figure_color: color, dirty: false },
                'n' => board.fields[i as usize] = Field { figure_type: FigureType::KNIGHT, figure_color: color, dirty: false },
                'b' => board.fields[i as usize] = Field { figure_type: FigureType::BISHOP, figure_color: color, dirty: false },
                'p' => board.fields[i as usize] = Field { figure_type: FigureType::PAWN, figure_color: color, dirty: false },
                _ => ()
            }
            i += 1;
        }
    }

    // active color
    match fen_active {
        "w" => *next_move = FigureColor::WHITE,
        "b" => *next_move = FigureColor::BLACK,
        _ => ()
    }

    // castle rights
    if !fen_castle.contains("K") { board.fields[63].dirty = true; }
    if !fen_castle.contains("Q") { board.fields[56].dirty = true; }
    if !fen_castle.contains("k") { board.fields[7].dirty = true; }
    if !fen_castle.contains("q") { board.fields[0].dirty = true; }

    // en passant
    *en_passant = translate_position_to_index(fen_en_passant);

    return board;
}
