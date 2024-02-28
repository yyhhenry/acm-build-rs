use acm_build::{acm_rand::AcmRand, build_data};
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
        let a = (0..n)
            .map(|_| self.rng.gen_range(1..=20).to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(file, "{}", a)?;
        Ok(())
    }
}
fn main() -> Result<()> {
    let generator = Generator {
        rng: rand::SeedableRng::seed_from_u64(42),
    };
    build_data("a", 1..=10, generator)
}
