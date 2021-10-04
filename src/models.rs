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
}

#[derive(Clone, Copy, Debug)]
pub struct CastleRights {
    pub K: bool,
    pub Q: bool,
    pub k: bool,
    pub q: bool
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub fields: [Field; 64],    
    pub active: FigureColor,
    pub castle_rights: CastleRights,
    pub en_passant: i8
}
