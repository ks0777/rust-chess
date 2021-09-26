use std::vec::Vec;

pub mod models;
use models::{Figure, FigureType, FigureColor, Field, Board};

fn is_occupied(x: i8, y: i8, board: &Board) -> FigureColor {
    board.fields[(x + y*8) as usize].figure_color
}

fn is_checked(figure_color: FigureColor, board: Board) -> bool {
    let mut king_field_index = 0;
    for i in 0..64 {
        let field = board.fields[i as usize];
        if field.figure_color == figure_color && field.figure_type == FigureType::KING {
            king_field_index = i; 
        }
    }

    for i in 0..64 {
        let field = board.fields[i as usize];
        if field.figure_color != FigureColor::NONE && field.figure_color != figure_color && field.figure_type != FigureType::KING {
            let reachable_fields = calc_reachable_fields(i, &board);
            if reachable_fields.contains(&king_field_index) { return true; }
        }
    }
//    for dx in -1..2 {
//        for dy in -1..2 {
//            if dx == 0 && dy == 0 { continue; }
//            for i in 1..8 {
//                if dx*i + x >= 0 && dx*i + x < 8 && dy*i + y >= 0 && dy*i + y < 8 {
//                    let field = board.fields[(x+dx + (y+dy)*8) as usize];
//                    if field.figure_color == figure_color { break; }
//                    if field.figure_color != FigureColor::NONE && field.figure_color != figure_color {
//                        if field.figure_type == FigureType::QUEEN { return true; }
//                        if (dy == 0 || dx == 0) && (dx != 0 || dy != 0 ) { 
//                            if field.figure_type == FigureType::ROOK { return true; }
//                        }
//                        if !(!(dy != 0 && dx != 0) || (dx == 0 && dy == 0 )) { 
//                            if field.figure_type == FigureType::BISHOP { return true; }
//                        }
//                    }
//                }
//            }
//        }
//    } 
    return false;
}

fn calc_reachable_fields(src_field: i8, board: &Board) -> Vec<i8> {
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
                        vec.push(dx + x + ((dy + y) * 8));
                    }
                }
            }

            // Castle
            let i = if field.figure_color == FigureColor::BLACK { 0 } else { 1 };
            if src_field != 4+i*56 || field.dirty { return vec; }
            for corner_index in [i*56, i*56+7].iter() {
                let corner_field = board.fields[*corner_index as usize];
                if corner_field.dirty || corner_field.figure_type != FigureType::ROOK { continue; }

                let dx_range = if corner_index % 8 == 0 { vec![-1,-2,-3] } else { vec![1,2] };

                let mut obstacle = false;
                for dx in dx_range { 
                    if is_occupied(x+dx, y, board) != FigureColor::NONE { obstacle = true; break; } 
                }
                if !obstacle { vec.push(*corner_index); }
            }
        },
        FigureType::PAWN => {
            let dy = if field.figure_color == FigureColor::WHITE { 1 } else { -1 };
            if is_occupied(x, y-dy, board) == FigureColor::NONE {
                vec.push(x + (y-dy)*8);
            }
            if !field.dirty && is_occupied(x, y-dy*2, board) == FigureColor::NONE {
                vec.push(x + (y-dy*2)*8);
            }
            for dx in [-1, 1].iter() {
                let occupation = is_occupied(x+dx, y-dy, board);
                if !(occupation == FigureColor::NONE) && occupation != field.figure_color {
                    vec.push(x+dx + (y-dy)*8);
                }
            }
        },
        FigureType::KNIGHT => {
            for dx in -2..3i8 {
                for dy in -2..3i8 {
                    if dy.abs() + dx.abs() == 3 {
                        if dx + x >= 0 && dx + x < 8 && dy + y >= 0 && dy + y < 8 {
                            if is_occupied(dx + x, dy + y, board) == field.figure_color { continue; }
                            vec.push(dx + x + ((dy + y) * 8));
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
                                vec.push(dx*i + x + ((dy*i + y) * 8));
                                break;
                            }
                            vec.push(dx*i + x + ((dy*i + y) * 8));
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
                                vec.push(dx*i + x + ((dy*i + y) * 8));
                                break;
                            }
                            vec.push(dx*i + x + ((dy*i + y) * 8));
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
                                vec.push(dx*i + x + ((dy*i + y) * 8));
                                break;
                            }
                            vec.push(dx*i + x + ((dy*i + y) * 8));
                        }
                    }
                }
            }            
        }
        _ => vec.push(0)
    }

    vec
}

fn is_legal(src_field_id: i8, dst_field_id: i8, board: Board) -> bool {
    let mut new_board = board.clone();
    let src_field = board.fields[src_field_id as usize];
    play_move(src_field_id, dst_field_id, &mut new_board);
    return !is_checked(src_field.figure_color, new_board);
}

pub fn calc_legal_moves(src_field: i8, board: &Board) -> Vec<i8> {
    let mut reachable_fields = calc_reachable_fields(src_field, board);

    reachable_fields = reachable_fields.into_iter().filter(|field_id| is_legal(src_field, *field_id, *board)).collect();

    return reachable_fields;
}

pub fn play_move (source_field_index: i8, target_field_index: i8, board: &mut Board) {
    let source_field = board.fields[source_field_index as usize];
    let target_field = board.fields[target_field_index as usize];

    if source_field.figure_type == FigureType::KING && target_field.figure_type == FigureType::ROOK {
        // Castle

        let side = if target_field_index % 8 != 0 { 1 } else { -1 };
        board.fields[source_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false }; 
        board.fields[target_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false }; 
        board.fields[(source_field_index + 1*side) as usize] = Field { figure_type: FigureType::ROOK, figure_color: source_field.figure_color, dirty: true }; 
        board.fields[(source_field_index + 2*side) as usize] = Field { figure_type: FigureType::KING, figure_color: source_field.figure_color, dirty: true }; 
    } else if source_field.figure_type == FigureType::PAWN && (target_field_index < 8 || target_field_index > 56) {
        // Promotion

        board.fields[target_field_index as usize] = Field { figure_type: FigureType::QUEEN, figure_color: source_field.figure_color, dirty: true };
        board.fields[source_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false }; 
    } else {
        board.fields[target_field_index as usize] = board.fields[source_field_index as usize];
        board.fields[target_field_index as usize].dirty = true; 
        board.fields[source_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false }; 
    }
}
