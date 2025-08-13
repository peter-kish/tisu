use std::fmt::Display;

const FLIPPED_HORIZONTALLY_FLAG: u32 = 0x80000000;
const FLIPPED_VERTICALLY_FLAG: u32 = 0x40000000;
const FLIPPED_DIAGONALLY_FLAG: u32 = 0x20000000;
const ALL_FLIP_FLAGS: u32 =
    FLIPPED_HORIZONTALLY_FLAG | FLIPPED_VERTICALLY_FLAG | FLIPPED_DIAGONALLY_FLAG;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TiledTile {
    pub index: Option<u32>,
    pub flip_h: bool,
    pub flip_v: bool,
    pub flip_d: bool,
}

impl From<&TiledTile> for u32 {
    fn from(value: &TiledTile) -> Self {
        let mut result = match value.index {
            Some(i) => i + 1,
            None => 0,
        };
        if value.flip_h {
            result |= FLIPPED_HORIZONTALLY_FLAG
        }
        if value.flip_v {
            result |= FLIPPED_VERTICALLY_FLAG
        }
        if value.flip_d {
            result |= FLIPPED_DIAGONALLY_FLAG
        }

        result
    }
}

impl Display for TiledTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u32::from(self))
    }
}

impl From<u32> for TiledTile {
    fn from(value: u32) -> Self {
        let flags = value & ALL_FLIP_FLAGS;
        let gid = value & !ALL_FLIP_FLAGS;
        Self {
            index: Some(gid),
            flip_h: flags & FLIPPED_HORIZONTALLY_FLAG == FLIPPED_HORIZONTALLY_FLAG,
            flip_v: flags & FLIPPED_VERTICALLY_FLAG == FLIPPED_VERTICALLY_FLAG,
            flip_d: flags & FLIPPED_DIAGONALLY_FLAG == FLIPPED_DIAGONALLY_FLAG,
        }
    }
}
