use ggez::*;
use ggez::graphics::Text;
use ggez::graphics::Color;
use ggez::input;
use mint::{Point2, Vector2};
use std::vec::Vec;
use std::env;

mod engine;
use engine::calc_legal_moves;
use engine::play_move;
//use engine::is_checked;
use engine::nega_max;
use engine::nega_max_ab;

mod models;
use models::{Figure, FigureType, FigureColor, Board};

mod utils;
use utils::board_from_fen;

mod tests;

struct PromotionState {
    src_index: i8,
    dst_index: i8,
    figure_type: FigureType,
    show_menu: bool
}

struct State {
    dt: std::time::Duration, 
    board: Board,    
    figures: [Figure; 13],
    source_field_index: i8,
    legal_moves: Vec<(i8, FigureType)>,
    promo_state: PromotionState,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        println!("Creating figures");
        let figures: [Figure; 13] = [
            Figure { image: graphics::Image::new(ctx, "/king_b.png").unwrap(), figure_type: FigureType::KING, figure_color: FigureColor::BLACK },
            Figure { image: graphics::Image::new(ctx, "/queen_b.png").unwrap(), figure_type: FigureType::QUEEN, figure_color: FigureColor::BLACK },
            Figure { image: graphics::Image::new(ctx, "/bishop_b.png").unwrap(), figure_type: FigureType::BISHOP, figure_color: FigureColor::BLACK },
            Figure { image: graphics::Image::new(ctx, "/knight_b.png").unwrap(), figure_type: FigureType::KNIGHT, figure_color: FigureColor::BLACK },
            Figure { image: graphics::Image::new(ctx, "/rook_b.png").unwrap(), figure_type: FigureType::ROOK, figure_color: FigureColor::BLACK },
            Figure { image: graphics::Image::new(ctx, "/pawn_b.png").unwrap(), figure_type: FigureType::PAWN, figure_color: FigureColor::BLACK },
            Figure { image: graphics::Image::new(ctx, "/king_w.png").unwrap(), figure_type: FigureType::KING, figure_color: FigureColor::WHITE },
            Figure { image: graphics::Image::new(ctx, "/queen_w.png").unwrap(), figure_type: FigureType::QUEEN, figure_color: FigureColor::WHITE },
            Figure { image: graphics::Image::new(ctx, "/bishop_w.png").unwrap(), figure_type: FigureType::BISHOP, figure_color: FigureColor::WHITE },
            Figure { image: graphics::Image::new(ctx, "/knight_w.png").unwrap(), figure_type: FigureType::KNIGHT, figure_color: FigureColor::WHITE },
            Figure { image: graphics::Image::new(ctx, "/rook_w.png").unwrap(), figure_type: FigureType::ROOK, figure_color: FigureColor::WHITE },
            Figure { image: graphics::Image::new(ctx, "/pawn_w.png").unwrap(), figure_type: FigureType::PAWN, figure_color: FigureColor::WHITE },
            Figure { image: graphics::Image::solid(ctx, 1, Color::from_rgba(0,0,0,0)).unwrap(), figure_type: FigureType::NONE, figure_color: FigureColor::NONE },
        ];
        println!("done!");

        // Set up board
        let args: Vec<String> = env::args().collect();
        let mut board = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        // load board from supplied fen
        if args.len() > 1 {
            board = board_from_fen(args[1].as_str());
        }

        let s = State {
            dt: std::time::Duration::new(0,0),
            board : board,
            figures: figures,
            source_field_index: -1,
            legal_moves: Vec::new(),
            promo_state: PromotionState { src_index: -1, dst_index: -1, show_menu: false, figure_type: FigureType::NONE },
        };
        Ok(s)
    }
}

const CHECKER_1: Color = Color{r: 0.431, g: 0.313, b: 0.313, a: 1.0};
const CHECKER_2: Color = Color{r: 0.878, g: 0.756, b: 0.756, a: 1.0};
const HIGHLIGHT: Color = Color{r: 0.043, g: 0.530, b: 0.016, a: 0.8};

//const CHECK_1: Color = Color{r: 0.0, g: 0.0, b: 1.0, a: 0.5};
//const CHECK_2: Color = Color{r: 1.0, g: 0.0, b: 0.0, a: 0.5};

fn draw_board(ctx: &mut Context, board: &Board, figures: &[Figure; 13], source_field_index: i8, legal_moves: &Vec<(i8, FigureType)>) {
        for i in 0..8 {
            for j in 0..8 {
                let color = if (i+j) % 2 != 0 { CHECKER_1 } else { CHECKER_2 };

                // draw checkers
                let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(i as f32 * 100.0, j as f32 * 100.0, 100.0, 100.0), color);
                graphics::draw(ctx, &rectangle.unwrap(), graphics::DrawParam::default()).unwrap();

                /*
                if is_checked(i+j*8, FigureColor::BLACK, *board) || is_checked(i+j*8, FigureColor::WHITE, *board) {
                    let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(i as f32 * 100.0, j as f32 * 100.0, 100.0, 100.0), Color{r:0.0,g:0.0,b:0.0,a:1.0});
                    graphics::draw(ctx, &rectangle.unwrap(), graphics::DrawParam::default()).unwrap();
                }
                if is_checked(i+j*8, FigureColor::WHITE, *board) {
                    let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(i as f32 * 100.0, j as f32 * 100.0, 100.0, 100.0), CHECK_1);
                    graphics::draw(ctx, &rectangle.unwrap(), graphics::DrawParam::default()).unwrap();
                }
                if is_checked(i+j*8, FigureColor::BLACK, *board) {
                    let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(i as f32 * 100.0, j as f32 * 100.0, 100.0, 100.0), CHECK_2);
                    graphics::draw(ctx, &rectangle.unwrap(), graphics::DrawParam::default()).unwrap();
                }
                */

                // draw figures
                let field = &board.fields[(i+j*8) as usize];
                if field.figure_type != FigureType::NONE {
                    for figure in figures {
                        if figure.figure_type == field.figure_type && figure.figure_color == field.figure_color {
                            let figure_dst = Point2 { x: i as f32 * 100.0, y: j as f32 * 100.0};
                            let mut draw_param = graphics::DrawParam::default().dest(figure_dst).scale(Vector2{x: 0.09765625, y: 0.09765625});

                            // draw shadow for dragged figure
                            if i + j*8 == source_field_index {
                                draw_param = draw_param.color(Color::from_rgba(0,0,0,160));
                            }

                            graphics::draw(ctx, &figure.image, draw_param).unwrap();
                        }
                    }
                }

                // highlight legal moves
                let current_pos_move: Vec<&(i8, FigureType)> = legal_moves.iter().filter(|legal_move| legal_move.0 == i+j*8).collect();
                if current_pos_move.len() > 0{ 
                    let circle_highlight = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), [i as f32 * 100.0 + 50.0, j as f32 * 100.0 + 50.0 ], 20.0, 0.01, HIGHLIGHT);
                    graphics::draw(ctx, &circle_highlight.unwrap(), graphics::DrawParam::default()).unwrap();
                }

            } 
        }
}

fn draw_promotion_menu(ctx: &mut Context, figures: &[Figure; 13]) {
    let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(195.0, 295.0, 410.0, 110.0), graphics::Color::BLACK);
    graphics::draw(ctx, &rectangle.unwrap(), graphics::DrawParam::default()).unwrap();
    let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(200.0, 300.0, 400.0, 100.0), graphics::Color::WHITE);
    graphics::draw(ctx, &rectangle.unwrap(), graphics::DrawParam::default()).unwrap();

    graphics::draw(ctx, &figures[1].image, graphics::DrawParam::default().dest(Vector2{x: 200.0, y: 300.0}).scale(Vector2{x: 0.09765625, y: 0.09765625})).unwrap();
    graphics::draw(ctx, &figures[2].image, graphics::DrawParam::default().dest(Vector2{x: 300.0, y: 300.0}).scale(Vector2{x: 0.09765625, y: 0.09765625})).unwrap();
    graphics::draw(ctx, &figures[3].image, graphics::DrawParam::default().dest(Vector2{x: 400.0, y: 300.0}).scale(Vector2{x: 0.09765625, y: 0.09765625})).unwrap();
    graphics::draw(ctx, &figures[4].image, graphics::DrawParam::default().dest(Vector2{x: 500.0, y: 300.0}).scale(Vector2{x: 0.09765625, y: 0.09765625})).unwrap();
}

fn draw_floating_figure(ctx: &mut Context, board: &Board, figures: &[Figure; 13], source_field_index: i8) {
    if source_field_index != -1 {
        input::mouse::set_cursor_hidden(ctx, true);
        let field = &board.fields[source_field_index as usize];    
        for figure in figures {
            if figure.figure_type == field.figure_type && figure.figure_color == field.figure_color {
                let mut mouse_position = input::mouse::position(ctx);
                mouse_position.x -= 50.0;
                mouse_position.y -= 50.0;
                graphics::draw(ctx, &figure.image, graphics::DrawParam::default().dest(mouse_position).scale(Vector2{x: 0.09765625, y: 0.09765625})).unwrap();
            }
        }
    } else {
        input::mouse::set_cursor_hidden(ctx, false);
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        draw_board(ctx, &self.board, &self.figures, self.source_field_index, &self.legal_moves);

        draw_floating_figure(ctx, &self.board, &self.figures, self.source_field_index);

        if self.promo_state.show_menu {
            draw_promotion_menu(ctx, &self.figures);
        }
        let text = Text::new(format!("{} FPS", 1000000000 / self.dt.as_nanos()));
        let text_dst = Point2 { x: 5.0, y: 5.0};
        graphics::draw(ctx, &text, graphics::DrawParam::default().dest(text_dst).color(Color::BLACK))?;


        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _btn: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if !self.promo_state.show_menu {
            let source_field_index = (((x as i32 - (x as i32 % 100)) / 100) + ((y as i32 - (y as i32 % 100)) / 100) * 8) as i8;
            let source_field = self.board.fields[source_field_index as usize];
            if source_field.figure_type != FigureType::NONE && source_field.figure_color == self.board.active {
                self.source_field_index = source_field_index;
                self.legal_moves = calc_legal_moves(source_field_index, &self.board);
            }
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _btn: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if !self.promo_state.show_menu {
            if self.source_field_index == -1 { return; }

            let target_field_index = (((x as i32 - (x as i32 % 100)) / 100) + ((y as i32 - (y as i32 % 100)) / 100) * 8) as i8;
             
            let selected_move: Vec<&(i8, FigureType)> = self.legal_moves.iter().filter(|target_move| target_move.0 == target_field_index).collect();
            if selected_move.len() == 1 {
                play_move(self.source_field_index, *selected_move[0], &mut self.board);
                let mut best_move = (-1, (-1, FigureType::NONE));
                //nega_max(&self.board, 4, &mut best_move);
                nega_max_ab(&self.board, 5, -32767, 32767, &mut best_move);
                if best_move.0 == -1 { println!("gg!"); } else {
                    play_move(best_move.0, best_move.1, &mut self.board);
                }
            } else if selected_move.len() > 1 {
                self.promo_state.src_index = self.source_field_index;
                self.promo_state.dst_index = target_field_index;
                self.promo_state.figure_type = FigureType::NONE;
                self.promo_state.show_menu = true;
            }
            
            self.source_field_index = -1;
            self.legal_moves = Vec::new();
        } else {
            let source_field_index = (((x as i32 - (x as i32 % 100)) / 100) + ((y as i32 - (y as i32 % 100)) / 100) * 8) as i8;
            match source_field_index {
                26 => self.promo_state.figure_type = FigureType::QUEEN,
                27 => self.promo_state.figure_type = FigureType::BISHOP,
                28 => self.promo_state.figure_type = FigureType::KNIGHT,
                29 => self.promo_state.figure_type = FigureType::ROOK,
                _ => ()
            }

            if self.promo_state.figure_type != FigureType::NONE {
                self.promo_state.show_menu = false;
                play_move(self.promo_state.src_index, (self.promo_state.dst_index, self.promo_state.figure_type), &mut self.board);
                let mut best_move = (-1, (-1, FigureType::NONE));
                //nega_max(&self.board, 4, &mut best_move);
                nega_max_ab(&self.board, 5, -32767, 32767, &mut best_move);
                if best_move.0 == -1 { println!("gg!"); } else {
                    play_move(best_move.0, best_move.1, &mut self.board);
                }
            }
        }
    }

}

pub fn main() -> GameResult {
    let c = conf::Conf::default().window_mode(conf::WindowMode::default().dimensions(800.0, 800.0));

    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "Was weiss ich")
        .default_conf(c)
        .window_setup(
            conf::WindowSetup::default().samples(conf::NumSamples::Eight))
        .build()
        .unwrap();

    let state = State::new(&mut ctx)?;
    event::run(ctx, event_loop, state);
}
