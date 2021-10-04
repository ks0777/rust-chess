#[cfg(test)]
mod tests {
    use crate::utils::*;
    use crate::models::*;
    use crate::engine::*;
    use std::time::Instant;
    use std::boxed::Box;

    #[test]
    fn index_to_position_test() {
        let position = translate_index_to_position(12);        

        assert_eq!(position, "e7");
    }

    #[test]
    fn position_to_index_test() {
        let index = translate_position_to_index("f1");        

        assert_eq!(index, 61);
    }

    #[test]
    fn translation_test() {
        for i in 0..64 {
            let position = translate_index_to_position(i);
            let index = translate_position_to_index(&position);

            assert_eq!(i, index as u8);
        }
    }

    fn perft_test_rec(board: &mut Board, depth: u8, max_depth: u8) -> usize {
        if depth == 0 { return 1; }
        let mut perft_score = 0;
        for index in 0..64 {
            let field = board.fields[index];
            if field.figure_color == board.active {
                let legal_moves = calc_reachable_fields(index as i8, &board, true);
                for m in legal_moves {
                    let mut board_cpy = Box::new(board.clone());
                    play_move(index as i8, m, &mut board_cpy);
                    if !is_king_checked(board.active, *board_cpy) {
                        let score = perft_test_rec(&mut board_cpy, depth-1, max_depth);
                        if depth == max_depth {
                            let mut promotion = "";
                            match m.1 {
                                FigureType::KNIGHT => promotion = "n",
                                FigureType::BISHOP => promotion = "b",
                                FigureType::ROOK => promotion = "r",
                                FigureType::QUEEN => promotion = "q",
                                _ => ()
                            }
                            println!("{}{}{}: {}", translate_index_to_position(index as u8), translate_index_to_position(m.0 as u8), promotion, score);
                        }
                        perft_score += score;
                    }
                }
            }
        }

        return perft_score;
    }
/*
    #[test]
    fn perft_test() {
        let max_depth = 5;
        let mut board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    
        let start = Instant::now();
        let perft_score = perft_test_rec(&mut board, max_depth, max_depth);
        println!("perft test @ depth {} took {}ms", max_depth, start.elapsed().as_millis());

        assert_eq!(perft_score, 4865609);
    }
*/
    #[test]
    fn perft_test2() {
        let max_depth = 4;
        let mut board = board_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    
        let start = Instant::now();
        let perft_score = perft_test_rec(&mut board, max_depth, max_depth);
        println!("perft test @ depth {} took {}ms", max_depth, start.elapsed().as_millis());

        assert_eq!(perft_score, 97862);
    }

}
