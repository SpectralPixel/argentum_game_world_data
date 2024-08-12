use core::fmt;

use crate::world_data::World;

use super::*;

// u8: From 0 to 255
// Might need to increase this number if the chunk size grows beyond 255.
type LocalCoordType = u8;

#[derive(PartialEq, Debug, Clone)]
pub struct LocalCoord {
    pub x: LocalCoordType,
    pub y: LocalCoordType,
    pub z: LocalCoordType,
}

impl fmt::Display for LocalCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LocalCoord ({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<GlobalCoord> for LocalCoord {
    fn from(global_position: GlobalCoord) -> Self {
        fn convert(mut axis_position: GlobalCoordType) -> LocalCoordType {
            let chunk_size = GlobalCoordType::try_from(World::CHUNK_SIZE).unwrap();

            while axis_position < 0 {
                axis_position += chunk_size;
            }

            LocalCoordType::try_from(axis_position % chunk_size).unwrap()
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
        fn wrapped_position_within_chunk_bounds(random_x: global_coords::GlobalCoordType, random_y: global_coords::GlobalCoordType, random_z: global_coords::GlobalCoordType) -> bool {
            let global_position = GlobalCoord::new(random_x, random_y, random_z);
            let local_position = LocalCoord::from(global_position);

            let chunk_size = World::CHUNK_SIZE;

            let x_is_wrapped = local_position.x < chunk_size;
            let y_is_wrapped = local_position.y < chunk_size;
            let z_is_wrapped = local_position.z < chunk_size;

            x_is_wrapped && y_is_wrapped && z_is_wrapped
        }
    }

    #[test]
    fn display() {
        let pos = GlobalCoord { x: 1, y: 2, z: 3 };

        assert_eq!(pos.to_string(), "GlobalCoord (1, 2, 3)")
    }
}
