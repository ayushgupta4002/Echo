
use crate::editor::terminal::Position;

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

// impl From<Location> for Position {
//     fn from(loc: Location) -> Self {
//         Self {
//             x: loc.x,
//             y: loc.y,
//         }
//     }
// }

// impl Location {
//     pub const fn subtract(&self, other: &Self) -> Self { 
//         Self {
//             x: self.x.saturating_sub(other.x),
//             y: self.y.saturating_sub(other.y),
//         }
//     }
// }