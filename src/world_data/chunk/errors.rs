use std::{error::Error, fmt};

use crate::coordinates::ChunkCoord;

#[derive(Debug, Clone)]
pub struct ChunkNotFoundError(pub ChunkCoord);

impl fmt::Display for ChunkNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk at {} doesn't exist!", self.0)
    }
}

impl Error for ChunkNotFoundError {}
