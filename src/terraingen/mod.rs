
pub mod opensimplex;
pub mod flat;

pub trait TerrainGenerator {
    //fn new(seed: Option<i64>) -> Self;
    fn get_height_at(&self, xz: (isize, isize)) -> usize;
}