#[cfg(test)]
mod tests {
    use crate::utils::*;
    use crate::models::*;
    use crate::engine::*;

    #[derive(Clone,Copy)]
    struct State {
        board: Board,
        en_passant: i8,
        next_move: FigureColor,
    }

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

    fn perft_test_rec(state: &mut State, depth: u8) -> usize {
        if depth == 0 { return 1; }
        let mut perft_score = 0;
        for index in 0..64 {
            let field = state.board.fields[index];
            if field.figure_color == state.next_move {
                let legal_moves = calc_legal_moves(index as i8, &state.board, &mut state.en_passant);
                for m in legal_moves {
                    let mut state_cpy = state.clone();
                    state_cpy.next_move = if state_cpy.next_move == FigureColor::WHITE { FigureColor::BLACK } else { FigureColor::WHITE };
                    play_move(index as i8, m, &mut state_cpy.board, &mut state_cpy.en_passant);
                    let score = perft_test_rec(&mut state_cpy, depth-1);
                    if depth == 5 {
                        println!("{}{}: {}", translate_index_to_position(index as u8), translate_index_to_position(m as u8), score);
                    }
                    perft_score += score;
                }
            }
        }

        return perft_score;
    }

    #[test]
    fn perft_test() {
        let max_depth = 5;
        let mut next_move = FigureColor::NONE;
        let mut en_passant = -1;
        let board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut next_move, &mut en_passant);

        
        let mut state = State { board: board, en_passant: en_passant, next_move: next_move };

        let perft_score = perft_test_rec(&mut state, max_depth);


        assert_eq!(perft_score, 4865609);
    }
}
