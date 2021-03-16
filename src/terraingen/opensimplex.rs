use noise::*;
use noise::NoiseFn;
use noise::Seedable;

pub struct OpensimplexGenerator {
    noisegen: Perlin,
    coord_scaling: (f64, f64),
    offset: f64,
    amplitude: f64,
}

impl OpensimplexGenerator {
    pub fn new(seed: Option<i64>, coord_scaling: (f64, f64), offset: f64, amplitude: f64) -> Self {
        let gen = Perlin::new();
        if let Some(seed) = seed {
            gen.set_seed(seed as u32);
        }

        OpensimplexGenerator {
            noisegen: gen,
            coord_scaling: coord_scaling,
            offset: offset,
            amplitude: amplitude,
        }
    }
}

impl super::TerrainGenerator for OpensimplexGenerator {
    fn get_height_at(&self, xz: (isize, isize)) -> usize {

        let raw = self.noisegen.get([xz.0 as f64 * self.coord_scaling.0, xz.1 as f64 * self.coord_scaling.1]);
        let normalized = (raw + 1.0) / 2.0;

        let combined = self.offset + self.amplitude * normalized;

        combined as usize
    }
}