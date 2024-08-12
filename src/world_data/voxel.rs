pub mod errors;

pub use errors::*;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Voxel(pub u8);

impl Voxel {
    pub fn new(voxel_type: u8) -> Self {
        Voxel(voxel_type)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn new_cube(voxel_type: u8) -> bool {
            let result = Voxel::new(voxel_type);
            let expected = Voxel(voxel_type);
            result == expected
        }
    }
}
