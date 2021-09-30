use ggez::graphics;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FigureColor {
    BLACK,
    WHITE,
    NONE
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FigureType {
    KING,
    QUEEN,
    BISHOP,
    KNIGHT,
    ROOK,
    PAWN,
    NONE
}

pub struct Figure {
    pub image: graphics::Image,
    pub figure_type: FigureType,
    pub figure_color: FigureColor,
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub figure_type: FigureType,
    pub figure_color: FigureColor,
    pub dirty: bool
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub fields: [Field; 64],    
}
