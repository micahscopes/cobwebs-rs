use std::hash::Hash;
use std::cmp::PartialOrd;
use im::{HashMap, HashSet};

pub trait Possible: Hash {
  fn generate(&self) -> Self;
}

pub enum Assessment<P: Possible, M: PartialOrd> {
  Unknown(P),
  Okay(P, M),
  Avoid(P, M)
}

pub trait Deliberate<P: Possible, M: PartialOrd> {
  fn deliberate(&self);
  fn consider(&self, possibility: P) -> Assessment<P, M>;
}

pub struct Deliberation<P: Possible, M: PartialOrd> {
  possibilities: HashSet<Assessment<P, M>>,
  iteration: u32,

  avoid: HashMap<u32, Assessment<P, M>>,
  favorites: HashSet<Assessment<P, M>>,
  choice: Assessment<P, M>,
}