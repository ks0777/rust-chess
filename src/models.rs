use ggez::graphics;

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum FigureColor {
    BLACK,
    WHITE,
    NONE
}

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
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

#[derive(Clone, Copy, Debug, Hash)]
pub struct Field {
    pub figure_type: FigureType,
    pub figure_color: FigureColor,
}

#[derive(Clone, Copy, Debug, Hash)]
#[allow(non_snake_case)]
pub struct CastleRights {
    pub K: bool,
    pub Q: bool,
    pub k: bool,
    pub q: bool
}

#[derive(Clone, Copy, Debug, Hash)]
pub struct Board {
    pub fields: [Field; 64],    
    pub active: FigureColor,
    pub castle_rights: CastleRights,
    pub en_passant: i8
}
