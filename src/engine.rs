use std::vec::Vec;

pub mod models;
use models::{Figure, FigureType, FigureColor, Field, Board};

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
                        vec.push(dx + x + ((dy + y) * 8));
                    }
                }
            }
        },
        FigureType::PAWN => {
            if field.figure_color == FigureColor::WHITE {
                if y > 0 { vec.push(x + (y-1)*8); }  
            } else {
                if y < 8 { vec.push(x + (y+1)*8); }  
            }
        },
        FigureType::KNIGHT => {
            for dx in -2..3i8 {
                for dy in -2..3i8 {
                    if dy.abs() + dx.abs() == 3 {
                        if dx + x >= 0 && dx + x < 8 && dy + y >= 0 && dy + y < 8 {
                            vec.push(dx + x + ((dy + y) * 8));
                        }
                    }
                }
            }
        }
        FigureType::ROOK => {
            
        }
        _ => vec.push(0)
    }

    vec
}

pub fn calc_legal_moves(src_field: i8, board: &Board) -> Vec<i8> {
    let reachable_fields = calc_reachable_fields(src_field, board);

    return reachable_fields;
}
