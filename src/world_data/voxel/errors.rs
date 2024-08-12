use std::{error::Error, fmt};

use crate::coordinates::{GlobalCoord, LocalCoord};

#[derive(Debug, Clone)]
pub struct VoxelNotFoundError(pub LocalCoord);

impl fmt::Display for VoxelNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Voxel at {} doesn't exist!", self.0)
    }
}

impl Error for VoxelNotFoundError {}

#[derive(Debug, Clone)]
pub struct WrappedPositionOutOfBoundsError(pub LocalCoord);

impl fmt::Display for WrappedPositionOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Local coordinate {} is out of bounds! Are you using a 32-bit system?",
            self.0
        )
    }
}

impl Error for WrappedPositionOutOfBoundsError {}
