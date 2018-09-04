#[macro_use(s)]
extern crate ndarray;
extern crate noisy_float;
extern crate num_traits;
extern crate rand;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck;

pub use maybe_nan::{MaybeNan, MaybeNanExt};
pub use min_max::MinMaxExt;
pub use percentile::{interpolate, PercentileExt};
pub use sort::Sort1dExt;

mod maybe_nan;
mod min_max;
mod percentile;
mod sort;
