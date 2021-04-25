pub mod flat;
pub mod opensimplex;

pub trait TerrainGenerator {
    //fn new(seed: Option<i64>) -> Self;
    fn get_height_at(&self, xz: (isize, isize)) -> usize;
}
