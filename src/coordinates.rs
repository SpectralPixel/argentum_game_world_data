mod chunk_coords;
mod local_coords;

pub use argentum_game_coordinate_system::{
    Coordinate as GlobalCoord, CoordinateType as GlobalCoordType,
};
pub use chunk_coords::ChunkCoord;
pub use local_coords::LocalCoord;
