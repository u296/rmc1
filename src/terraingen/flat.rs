pub struct FlatGenerator {
    height: usize,
}

impl FlatGenerator {
    pub fn new(height: usize) -> Self {
        FlatGenerator { height: height }
    }
}

impl super::TerrainGenerator for FlatGenerator {
    fn get_height_at(&self, _: (isize, isize)) -> usize {
        self.height
    }
}
