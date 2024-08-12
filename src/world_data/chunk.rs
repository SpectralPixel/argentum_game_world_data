use std::error::Error;

use argentum_game_voxel::Voxel;
use ndarray::{Array3, Ix3};

use crate::{
    coordinates::LocalCoord,
    errors::{VoxelNotFoundError, WrappedPositionOutOfBoundsError},
};

use super::World;

#[derive(PartialEq, Debug)]
pub struct Chunk {
    data: Array3<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        let size = World::CHUNK_SIZE as usize;
        let empty_array: Array3<Voxel> = Array3::from_elem(Ix3(size, size, size), Voxel::default());
        Chunk { data: empty_array }
    }

    pub fn get_voxel(&self, local_position: &LocalCoord) -> Result<Voxel, Box<dyn Error>> {
        let LocalCoord { x, y, z } = *local_position;
        let x = usize::try_from(x).unwrap();
        let y = usize::try_from(y).unwrap();
        let z = usize::try_from(z).unwrap();
        match self.data.get(Ix3(x, y, z)) {
            Some(voxel_reference) => Ok(voxel_reference.to_owned()),
            None => Err(Box::new(VoxelNotFoundError(local_position.to_owned()))),
        }
    }

    pub fn set_voxel(
        &mut self,
        local_position: &LocalCoord,
        voxel: Voxel,
    ) -> Result<(), Box<dyn Error>> {
        let LocalCoord { x, y, z } = *local_position;
        let x = usize::try_from(x)?;
        let y = usize::try_from(y)?;
        let z = usize::try_from(z)?;
        let current_voxel = self
            .data
            .get_mut(Ix3(x, y, z))
            .ok_or(WrappedPositionOutOfBoundsError(local_position.clone()))?;
        *current_voxel = voxel;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_chunk() {
        let size = World::CHUNK_SIZE as usize;
        let result = Chunk::new();
        let expected = Chunk {
            data: Array3::from_elem(Ix3(size, size, size), Voxel::default()),
        };
        assert_eq!(result, expected);
    }
}
