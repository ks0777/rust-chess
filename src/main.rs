use ggez::*;
use ggez::graphics::Text;
use ggez::graphics::Color;
use ggez::input;
use mint::{Point2, Vector2};
use std::vec::Vec;

mod engine;
use engine::calc_legal_moves;
use engine::models::{Figure, FigureType, FigureColor, Field, Board};

struct State {
    dt: std::time::Duration, 
    board: Board,    
    figures: [Figure; 13],
    source_field_index: i8,
    legal_moves: Vec<i8>
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
        let mut board: Board = Board { fields: [Field {figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false}; 64] };

        board.fields[0] = Field {figure_type: FigureType::ROOK, figure_color: FigureColor::BLACK, dirty: false};
        board.fields[1] = Field {figure_type: FigureType::KNIGHT, figure_color: FigureColor::BLACK, dirty: false};
        board.fields[2] = Field {figure_type: FigureType::BISHOP, figure_color: FigureColor::BLACK, dirty: false};
        board.fields[3] = Field {figure_type: FigureType::QUEEN, figure_color: FigureColor::BLACK, dirty: false};
        board.fields[4] = Field {figure_type: FigureType::KING, figure_color: FigureColor::BLACK, dirty: false};
        board.fields[5] = Field {figure_type: FigureType::BISHOP, figure_color: FigureColor::BLACK, dirty: false};
        board.fields[6] = Field {figure_type: FigureType::KNIGHT, figure_color: FigureColor::BLACK, dirty: false};
        board.fields[7] = Field {figure_type: FigureType::ROOK, figure_color: FigureColor::BLACK, dirty: false};

        board.fields[56] = Field {figure_type: FigureType::ROOK, figure_color: FigureColor::WHITE, dirty: false};
        board.fields[57] = Field {figure_type: FigureType::KNIGHT, figure_color: FigureColor::WHITE, dirty: false};
        board.fields[58] = Field {figure_type: FigureType::BISHOP, figure_color: FigureColor::WHITE, dirty: false};
        board.fields[59] = Field {figure_type: FigureType::QUEEN, figure_color: FigureColor::WHITE, dirty: false};
        board.fields[60] = Field {figure_type: FigureType::KING, figure_color: FigureColor::WHITE, dirty: false};
        board.fields[61] = Field {figure_type: FigureType::BISHOP, figure_color: FigureColor::WHITE, dirty: false};
        board.fields[62] = Field {figure_type: FigureType::KNIGHT, figure_color: FigureColor::WHITE, dirty: false};
        board.fields[63] = Field {figure_type: FigureType::ROOK, figure_color: FigureColor::WHITE, dirty: false};

        for i in 8..16 {
            board.fields[i] = Field {figure_type: FigureType::PAWN, figure_color: FigureColor::BLACK, dirty: false};
        }
        for i in 48..56 {
            board.fields[i] = Field {figure_type: FigureType::PAWN, figure_color: FigureColor::WHITE, dirty: false};
        }


        let s = State {
            dt: std::time::Duration::new(0,0),
            board : board,
            figures: figures,
            source_field_index: -1,
            legal_moves: Vec::new()
        };
        Ok(s)
    }
}

const CHECKER_1: Color = Color{r: 0.431, g: 0.313, b: 0.313, a: 1.0};
const CHECKER_2: Color = Color{r: 0.878, g: 0.756, b: 0.756, a: 1.0};
const HIGHLIGHT: Color = Color{r: 0.043, g: 0.530, b: 0.016, a: 0.8};

fn draw_board(ctx: &mut Context, board: &Board, figures: &[Figure; 13], source_field_index: i8, legal_moves: &Vec<i8>) {
        for i in 0..8 {
            for j in 0..8 {
                let color = if (i+j) % 2 != 0 { CHECKER_1 } else { CHECKER_2 };

                // draw checkers
                let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(i as f32 * 100.0, j as f32 * 100.0, 100.0, 100.0), color);
                graphics::draw(ctx, &rectangle.unwrap(), graphics::DrawParam::default()).unwrap();

                let field = &board.fields[(i+j*8) as usize];
                if field.dirty {
                        let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(i as f32 * 100.0, j as f32 * 100.0, 100.0, 100.0), HIGHLIGHT);
                        graphics::draw(ctx, &rectangle.unwrap(), graphics::DrawParam::default()).unwrap();
                }

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
                if legal_moves.contains(&(i + j*8)) { 
                    let circle_highlight = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), [i as f32 * 100.0 + 50.0, j as f32 * 100.0 + 50.0 ], 20.0, 0.01, HIGHLIGHT);
                    graphics::draw(ctx, &circle_highlight.unwrap(), graphics::DrawParam::default()).unwrap();
                }

            } 
        }
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
        let source_field_index = (((x as i32 - (x as i32 % 100)) / 100) + ((y as i32 - (y as i32 % 100)) / 100) * 8) as i8;
        if self.board.fields[source_field_index as usize].figure_type != FigureType::NONE {
            self.source_field_index = source_field_index;
            self.legal_moves = calc_legal_moves(source_field_index, &self.board);
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _btn: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if self.source_field_index == -1 { return; }
        let source_field = self.board.fields[self.source_field_index as usize];

        let target_field_index = (((x as i32 - (x as i32 % 100)) / 100) + ((y as i32 - (y as i32 % 100)) / 100) * 8) as i8;
        let target_field = self.board.fields[target_field_index as usize];
         
        if self.legal_moves.contains(&target_field_index) {
            if source_field.figure_type == FigureType::KING && target_field.figure_type == FigureType::ROOK {
                // Castle

                let side = if target_field_index % 8 != 0 { 1 } else { -1 };
                self.board.fields[self.source_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false }; 
                self.board.fields[target_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false }; 
                self.board.fields[(self.source_field_index + 1*side) as usize] = Field { figure_type: FigureType::ROOK, figure_color: source_field.figure_color, dirty: true }; 
                self.board.fields[(self.source_field_index + 2*side) as usize] = Field { figure_type: FigureType::KING, figure_color: source_field.figure_color, dirty: true }; 
            } else if source_field.figure_type == FigureType::PAWN && target_field_index < 8 || target_field_index > 56 {
                // Promotion

                self.board.fields[target_field_index as usize] = Field { figure_type: FigureType::QUEEN, figure_color: source_field.figure_color, dirty: true };
                self.board.fields[self.source_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false }; 
            } else {
                self.board.fields[target_field_index as usize] = self.board.fields[self.source_field_index as usize];
                self.board.fields[target_field_index as usize].dirty = true; 
                self.board.fields[self.source_field_index as usize] = Field { figure_type: FigureType::NONE, figure_color: FigureColor::NONE, dirty: false }; 
            }
        }
        
        self.source_field_index = -1;
        self.legal_moves = Vec::new();
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
