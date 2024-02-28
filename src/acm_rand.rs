use rand::{seq::SliceRandom, Rng};
use std::ops::Range;

pub trait AcmRand: Rng {
    fn gen_scale(&mut self, scale: usize) -> usize {
        let low = (0.9 * scale as f64).ceil() as usize;
        self.gen_range(low..=scale)
    }
    fn gen_ordered_pair(&mut self, range: Range<usize>) -> (usize, usize) {
        let l = self.gen_range(range.clone());
        let r = self.gen_range(range);
        (l.min(r), l.max(r))
    }
    fn gen_distinct_pair(&mut self, range: Range<usize>) -> (usize, usize) {
        let l = self.gen_range(range.clone());
        let r = self.gen_range(range.start..range.end - 1);
        (l, r + (r >= l) as usize)
    }
    fn gen_tree(&mut self, n: usize, index: usize) -> Vec<(usize, usize)> {
        let mut perm: Vec<_> = (index..n + index).collect();
        perm.shuffle(self);
        let mut edges = Vec::new();
        for (i, &v) in perm.iter().skip(1).enumerate() {
            edges.push((v, perm[self.gen_scale(i)]));
        }
        edges
    }
}
impl<T> AcmRand for T where T: Rng {}
