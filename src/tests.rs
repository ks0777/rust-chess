#[cfg(test)]
mod tests {
    use crate::utils::*;
    use crate::models::*;
    use crate::engine::*;

    #[test]
    fn index_to_position_test() {
        let position = translate_index_to_position(12);        

        assert_eq!(position, "e7");
    }

    #[test]
    fn position_to_index_test() {
        let index = translate_position_to_index("e7");        

        assert_eq!(index, 12);
    }

    #[test]
    fn translation_test() {
        for i in 0..64 {
            let position = translate_index_to_position(i);
            let index = translate_position_to_index(&position);

            assert_eq!(i, index as u8);
        }
    }


    #[test]
    fn perft_test() {
        let mut next_move = FigureColor::NONE;
        let mut en_passant = -1;
        let mut board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut next_move, &mut en_passant);

        let mut perft_score = 0;

        for index in 0..64 {
            let field = board.fields[index];
            if field.figure_color == next_move {
                let legal_moves = calc_legal_moves(index as i8, &board, &mut en_passant);
                perft_score += legal_moves.len();
                for m in legal_moves {
                    println!("{}{}\t{}{}", translate_index_to_position(index as u8), translate_index_to_position(m as u8), index, m);
                }
            }
        }

        assert_eq!(perft_score, 20);
    }
}
