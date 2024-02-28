use rand::{seq::SliceRandom, Rng};
use std::ops::Range;

pub trait AcmRand: Rng {
    fn gen_scale(&mut self, scale: usize) -> usize {
        let low = (0.9 * scale as f64).ceil() as usize;
        self.gen_range(low..scale)
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
    fn gen_tree(&mut self, n: usize) -> Vec<(usize, usize)> {
        let mut perm: Vec<_> = (1..n).collect();
        perm.shuffle(self);
        let mut edges = Vec::new();
        for &v in &perm {
            edges.push((v, self.gen_scale(v - 1)));
        }
        edges
    }
}
impl<T> AcmRand for T where T: Rng {}
