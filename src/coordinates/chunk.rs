use core::fmt;

use crate::{coordinates::*, world_data::World};

// i32: From −2,147,483,648 to 2,147,483,647
// Had to make i32 because of failing test with i16...
pub type ChunkCoordType = i32;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct ChunkCoord {
    pub x: ChunkCoordType,
    pub y: ChunkCoordType,
    pub z: ChunkCoordType,
}

impl ChunkCoord {
    pub const MIN: Self = Self {
        x: ChunkCoordType::MIN,
        y: ChunkCoordType::MIN,
        z: ChunkCoordType::MIN,
    };
    pub const MAX: Self = Self {
        x: ChunkCoordType::MAX,
        y: ChunkCoordType::MAX,
        z: ChunkCoordType::MAX,
    };

    pub fn new(x: ChunkCoordType, y: ChunkCoordType, z: ChunkCoordType) -> Self {
        Self { x, y, z }
    }
}

impl fmt::Display for ChunkCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChunkCoord ({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<GlobalCoord> for ChunkCoord {
    fn from(global_position: GlobalCoord) -> Self {
        fn convert(axis_position: GlobalCoordType) -> ChunkCoordType {
            ChunkCoordType::try_from(
                axis_position / GlobalCoordType::try_from(World::CHUNK_SIZE).unwrap(),
            )
            .unwrap()
        }

        Self {
            x: convert(global_position.x),
            y: convert(global_position.y),
            z: convert(global_position.z),
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_position(x: ChunkCoordType, y: ChunkCoordType, z: ChunkCoordType) -> bool {
            let result = ChunkCoord::new(x, y, z);
            let expected = ChunkCoord { x, y, z };
            result == expected
        }
    }

    #[test]
    fn min_pos() {
        let expected = ChunkCoord {
            x: ChunkCoordType::MIN,
            y: ChunkCoordType::MIN,
            z: ChunkCoordType::MIN,
        };
        assert_eq!(expected, ChunkCoord::MIN);
        assert_eq!(expected.x, ChunkCoordType::MIN);
        assert_eq!(expected.y, ChunkCoordType::MIN);
        assert_eq!(expected.z, ChunkCoordType::MIN);
    }

    #[test]
    fn max_pos() {
        let expected = ChunkCoord {
            x: ChunkCoordType::MAX,
            y: ChunkCoordType::MAX,
            z: ChunkCoordType::MAX,
        };
        assert_eq!(expected, ChunkCoord::MAX);
        assert_eq!(expected.x, ChunkCoordType::MAX);
        assert_eq!(expected.y, ChunkCoordType::MAX);
        assert_eq!(expected.z, ChunkCoordType::MAX);
    }

    #[test]
    fn display() {
        let pos = ChunkCoord { x: 1, y: 2, z: 3 };

        assert_eq!(pos.to_string(), "ChunkCoord (1, 2, 3)")
    }
}
