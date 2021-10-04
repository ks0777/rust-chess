use std::vec::Vec;

use crate::models::{Figure, FigureType, FigureColor, Field, Board};

fn is_occupied(x: i8, y: i8, board: &Board) -> FigureColor {
    board.fields[(x + y*8) as usize].figure_color
}

fn is_checked(field_index: i8, figure_color: FigureColor, board: Board) -> bool {
    for i in 0..64 {
        let field = board.fields[i as usize];
        if field.figure_color != FigureColor::NONE && field.figure_color != figure_color {
            let reachable_fields = calc_reachable_fields(i, &board, false);
            let checking_field: Vec<&(i8, FigureType)> = reachable_fields.iter().filter(|field| field.0 == field_index).collect();
            if checking_field.len() > 0 { return true; }
        }
    }
    return false;
}

pub fn is_king_checked(figure_color: FigureColor, board: Board) -> bool {
    let mut king_field_index = 0;
    for i in 0..64 {
        let field = board.fields[i as usize];
        if field.figure_color == figure_color && field.figure_type == FigureType::KING {
            king_field_index = i; 
        }
    }
    return is_checked(king_field_index, board.fields[king_field_index as usize].figure_color, board);
}

pub fn calc_reachable_fields(src_field: i8, board: &Board, check: bool) -> Vec<(i8, FigureType)> {
    let mut vec = Vec::new();

    let field = &board.fields[src_field as usize];

    let pos = (src_field % 8, src_field / 8);
    let (x, y) = pos; 

    match field.figure_type {
        FigureType::KING => {
            for dx in -1..2 {
                for dy in -1..2 {
                    if dx == 0 && dy == 0 { continue; }

                    if dx + x >= 0 && dx + x < 8 && dy + y >= 0 && dy + y < 8 {
                        if is_occupied(dx + x, dy + y, board) == field.figure_color { continue; }
                        vec.push((dx + x + ((dy + y) * 8), FigureType::NONE));
                    }
                }
            }

            // Castle
            let i = if field.figure_color == FigureColor::BLACK { 0 } else { 1 };
            if src_field != 4+i*56 { return vec; }
            for corner_index in [i*56, i*56+7].iter() {
                if corner_index % 8 == 0 {
                    // queen-side 
                    if !((field.figure_color == FigureColor::BLACK && board.castle_rights.q) || (field.figure_color == FigureColor::WHITE && board.castle_rights.Q)) { continue; }
                } else  {
                    // king-side
                    if !((field.figure_color == FigureColor::BLACK && board.castle_rights.k) || (field.figure_color == FigureColor::WHITE && board.castle_rights.K)) { continue; }
                }

                let dx_range = if corner_index % 8 == 0 { vec![-1,-2,-3] } else { vec![1,2] };

                let mut obstacle = false;
                for dx in dx_range { 
                    if is_occupied(x+dx, y, board) != FigureColor::NONE { obstacle = true; break; } 
                }

                if !obstacle {
                    let dx_range = if corner_index % 8 == 0 { vec![0,-1,-2] } else { vec![0,1,2] };
                    for dx in dx_range {
                        if check && is_checked(dx + x + y*8, field.figure_color, *board) { obstacle = true; break; }
                    }
                }

                if !obstacle { vec.push((*corner_index, FigureType::NONE)); }
            }
        },
        FigureType::PAWN => {
            let dy = if field.figure_color == FigureColor::WHITE { 1 } else { -1 };
            if is_occupied(x, y-dy, board) == FigureColor::NONE {
                if y-dy == 0 || y-dy == 7 {
                    vec.push((x + (y-dy)*8, FigureType::KNIGHT));
                    vec.push((x + (y-dy)*8, FigureType::BISHOP));
                    vec.push((x + (y-dy)*8, FigureType::ROOK));
                    vec.push((x + (y-dy)*8, FigureType::QUEEN));
                } else {
                    vec.push((x + (y-dy)*8, FigureType::NONE));
                }

                if (y+dy == 0 || y+dy == 7) && is_occupied(x, y-dy*2, board) == FigureColor::NONE {
                    vec.push((x + (y-dy*2)*8, FigureType::NONE));
                }
            }
            for dx in [-1, 1].iter() {
                if x+dx >= 0 && x+dx < 8 {
                    let occupation = is_occupied(x+dx, y-dy, board);
                    if (occupation != FigureColor::NONE || (x+dx + (y-dy)*8) == board.en_passant) && occupation != field.figure_color {
                        if y-dy == 0 || y-dy == 7 {
                            vec.push((x+dx + (y-dy)*8, FigureType::KNIGHT));
                            vec.push((x+dx + (y-dy)*8, FigureType::BISHOP));
                            vec.push((x+dx + (y-dy)*8, FigureType::ROOK));
                            vec.push((x+dx + (y-dy)*8, FigureType::QUEEN));
                        } else {
                            vec.push((x+dx + (y-dy)*8, FigureType::NONE));
                        }
                    }
                }
            }
        },
        FigureType::KNIGHT => {
            for dx in -2..3i8 {
                for dy in -2..3i8 {
                    if dy.abs() + dx.abs() == 3 {
                        if dx + x >= 0 && dx + x < 8 && dy + y >= 0 && dy + y < 8 {
                            if is_occupied(dx + x, dy + y, board) == field.figure_color { continue; }
                            vec.push((dx + x + ((dy + y) * 8), FigureType::NONE));
                        }
                    }
                }
            }
        }
        FigureType::ROOK => {
            for dx in -1..2 {
                for dy in -1..2 {
                    if (dy != 0 && dx != 0) || (dx == 0 && dy == 0 ) { continue; }
                    for i in 1..8 {
                        if dx*i + x >= 0 && dx*i + x < 8 && dy*i + y >= 0 && dy*i + y < 8 {
                            if is_occupied(dx*i + x, dy*i + y, board) == field.figure_color { break; }
                            if is_occupied(dx*i + x, dy*i + y, board) != FigureColor::NONE { 
                                vec.push((dx*i + x + ((dy*i + y) * 8), FigureType::NONE));
                                break;
                            }
                            vec.push((dx*i + x + ((dy*i + y) * 8), FigureType::NONE));
                        }
                    }
                }
            }            
        }
        FigureType::BISHOP => {
            for dx in -1..2 {
                for dy in -1..2 {
                    if !(dy != 0 && dx != 0) || (dx == 0 && dy == 0 ) { continue; }
                    for i in 1..8 {
                        if dx*i + x >= 0 && dx*i + x < 8 && dy*i + y >= 0 && dy*i + y < 8 {
                            if is_occupied(dx*i + x, dy*i + y, board) == field.figure_color { break; }
                            if is_occupied(dx*i + x, dy*i + y, board) != FigureColor::NONE { 
                                vec.push((dx*i + x + ((dy*i + y) * 8), FigureType::NONE));
                                break;
                            }
                            vec.push((dx*i + x + ((dy*i + y) * 8), FigureType::NONE));
                        }
                    }
                }
            }            
        }
        FigureType::QUEEN => {
            for dx in -1..2 {
                for dy in -1..2 {
                    if dx == 0 && dy == 0 { continue; }
                    for i in 1..8 {
                        if dx*i + x >= 0 && dx*i + x < 8 && dy*i + y >= 0 && dy*i + y < 8 {
                            if is_occupied(dx*i + x, dy*i + y, board) == field.figure_color { break; }
                            if is_occupied(dx*i + x, dy*i + y, board) != FigureColor::NONE { 
                                vec.push((dx*i + x + ((dy*i + y) * 8), FigureType::NONE));
                                break;
                            }
                            vec.push((dx*i + x + ((dy*i + y) * 8), FigureType::NONE));
                        }
                    }
                }
            }            
        }
        _ => vec.push((-1, FigureType::NONE))
    }

    vec
}

fn is_legal(src_field_id: i8, target_move: (i8, FigureType), board: Board) -> bool {
    let mut new_board = board.clone();
    let src_field = board.fields[src_field_id as usize];
    play_move(src_field_id, target_move, &mut new_board);
    return !is_king_checked(src_field.figure_color, new_board);
}

pub fn calc_legal_moves(src_field: i8, board: &Board) -> Vec<(i8, FigureType)> {
    let mut reachable_fields = calc_reachable_fields(src_field, board, true);

    reachable_fields = reachable_fields.into_iter().filter(|field| is_legal(src_field, *field, *board)).collect();

    return reachable_fields;
}

pub fn play_move (source_field_index: i8, target_move: (i8, FigureType), board: &mut Board) {
    let target_field_index = target_move.0;
    let source_field = board.fields[source_field_index as usize];
    let target_field = board.fields[target_field_index as usize];

    // en-passant
    if source_field.figure_type == FigureType::PAWN {
        if (source_field_index - target_field_index).abs() == 16 {
            board.en_passant = (target_field_index - source_field_index) / 2 + source_field_index;
        } else {
            board.en_passant = -1;
            if (source_field_index - target_field_index).abs() != 8 && target_field.figure_type == FigureType::NONE {
                if source_field.figure_color == FigureColor::WHITE {
                    board.fields[(target_field_index+8) as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE };
                } else {
                    board.fields[(target_field_index-8) as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE };
                }
            }
        }
    } else { board.en_passant = -1; }

    if source_field.figure_type == FigureType::KING && target_field.figure_type != FigureType::ROOK && source_field.figure_color != target_field.figure_color {
        // non-castle king move
        if source_field.figure_color == FigureColor::WHITE { 
            board.castle_rights.Q = false;
            board.castle_rights.K = false;
        } else {
            board.castle_rights.q = false;
            board.castle_rights.k = false;
        }
    } else if source_field.figure_type == FigureType::KING && target_field.figure_type == FigureType::ROOK && source_field.figure_color == target_field.figure_color {
        // Castle

        let side = if target_field_index % 8 != 0 { 1 } else { -1 };
        board.fields[source_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE }; 
        board.fields[target_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE }; 
        board.fields[(source_field_index + 1*side) as usize] = Field { figure_type: FigureType::ROOK, figure_color: source_field.figure_color }; 
        board.fields[(source_field_index + 2*side) as usize] = Field { figure_type: FigureType::KING, figure_color: source_field.figure_color }; 

        if side == 1 {
            if board.active == FigureColor::WHITE { board.castle_rights.K = false; } else { board.castle_rights.k = false; }
        } else {
            if board.active == FigureColor::WHITE { board.castle_rights.Q = false; } else { board.castle_rights.q = false; }
        }
    } else if source_field.figure_type == FigureType::PAWN && (target_field_index < 8 || target_field_index > 56) {
        // Promotion

        board.fields[target_field_index as usize] = Field { figure_type: target_move.1, figure_color: source_field.figure_color };
        board.fields[source_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE }; 
    } else {
        board.fields[target_field_index as usize] = board.fields[source_field_index as usize];
        board.fields[source_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE }; 
    }

    board.active = if board.active == FigureColor::WHITE { FigureColor::BLACK } else { FigureColor::WHITE } 
}
