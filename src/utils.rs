use crate::models::{Figure, FigureType, FigureColor, Field, Board, CastleRights};

pub fn translate_position_to_index(pos: &str) -> i8 {
    let pos_chars: Vec<char> = pos.chars().collect();
    if pos_chars.len() == 2 {
        let file = (pos_chars[0] as i8) - 97;
        let rank = 7 - ((pos_chars[1] as i8) - 49);
        return file + rank * 8;
    }
    return -1;
}

pub fn translate_index_to_position(index: u8) -> String {
    let mut file = index % 8;
    let mut rank = (index-file) / 8;

    file += 97;
    rank = (7 - rank) + 49;
    return format!("{}{}", file as char, rank as char);
}

pub fn board_from_fen(fen: &str) -> Board {
    let fen_split: Vec<&str> = fen.split(' ').collect();

    let fen_board = fen_split[0];
    let fen_active = fen_split[1];
    let fen_castle = fen_split[2];
    let fen_en_passant = fen_split[3];

    // active color
    let mut active = FigureColor::NONE;
    match fen_active {
        "w" => active = FigureColor::WHITE,
        "b" => active = FigureColor::BLACK,
        _ => ()
    }

    // castle rights
    let mut castle_rights = CastleRights { K: false, Q: false, k: false, q: false};
    if fen_castle.contains("K") { castle_rights.K = true; }
    if fen_castle.contains("Q") { castle_rights.Q = true; }
    if fen_castle.contains("k") { castle_rights.k = true; }
    if fen_castle.contains("q") { castle_rights.q = true; }
    println!("{} {} {} {}", castle_rights.K, castle_rights.Q, castle_rights.k, castle_rights.q);

    // en passant

    let mut i = 0;
    let mut board: Board = Board {
        fields: [Field {figure_type: FigureType::NONE, figure_color: FigureColor::NONE}; 64],
        active: active,
        castle_rights: castle_rights,
        en_passant: translate_position_to_index(fen_en_passant)
    };

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
                'k' => board.fields[i as usize] = Field { figure_type: FigureType::KING, figure_color: color },
                'q' => board.fields[i as usize] = Field { figure_type: FigureType::QUEEN, figure_color: color },
                'r' => board.fields[i as usize] = Field { figure_type: FigureType::ROOK, figure_color: color },
                'n' => board.fields[i as usize] = Field { figure_type: FigureType::KNIGHT, figure_color: color },
                'b' => board.fields[i as usize] = Field { figure_type: FigureType::BISHOP, figure_color: color },
                'p' => board.fields[i as usize] = Field { figure_type: FigureType::PAWN, figure_color: color },
                _ => ()
            }
            i += 1;
        }
    }

    return board;
}
