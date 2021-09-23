use std::vec::Vec;

pub mod models;
use models::{Figure, FigureType, FigureColor, Field, Board};

fn is_occupied(x: i8, y: i8, board: &Board) -> FigureColor {
    board.fields[(x + y*8) as usize].figure_color
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
        },
        FigureType::PAWN => {
            let dy = if field.figure_color == FigureColor::WHITE { 1 } else { -1 };
            if is_occupied(x, y-dy, board) == FigureColor::NONE {
                vec.push(x + (y-dy)*8);
            }
            if is_occupied(x, y-dy*2, board) == FigureColor::NONE && !field.dirty {
                vec.push(x + (y-dy*2)*8);
            }
            if !(is_occupied(x+1, y-dy, board) == FigureColor::NONE) || is_occupied(x+1, y-dy, board) == field.figure_color {
                vec.push(x+1 + (y-dy)*8);
            }
            if !(is_occupied(x-1, y-dy, board) == FigureColor::NONE) || is_occupied(x-1, y-dy, board) == field.figure_color {
                vec.push(x-1 + (y-dy)*8);
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

pub fn calc_legal_moves(src_field: i8, board: &Board) -> Vec<i8> {
    let reachable_fields = calc_reachable_fields(src_field, board);

    return reachable_fields;
}
