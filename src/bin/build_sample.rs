use acm_build::{acm_rand::AcmRand, build_data, ACMDisplay};
use anyhow::Result;
use rand::Rng;
use std::io::Write;
struct Generator {
    rng: rand::rngs::StdRng,
}
impl acm_build::Generator for Generator {
    fn generate(&mut self, _id: usize, file: &mut std::fs::File) -> Result<()> {
        let n = self.rng.gen_scale(40);
        writeln!(file, "{}", n)?;
        let a: Vec<_> = (0..n).map(|_| self.rng.gen_range(1..=20)).collect();
        writeln!(file, "{}", a.acm_display())?;
        Ok(())
    }
}
fn main() -> Result<()> {
    let generator = Generator {
        rng: rand::SeedableRng::seed_from_u64(42),
    };
    build_data("sample", 1..=10, generator)
}
