#[cfg(feature = "rand_proposer")]
use rand_core::{RngCore, SeedableRng};
#[cfg(feature = "rand_proposer")]
use rand_pcg::Pcg64Mcg as Pcg;

#[cfg(feature = "rand_proposer")]
pub(crate) fn get_index(seed: u64, weight: &[u64]) -> usize {
    let sum: u64 = weight.iter().sum();
    let x = u64::max_value() / sum;

    let mut rng = Pcg::seed_from_u64(seed);
    let mut res = rng.next_u64();
    while res >= sum * x {
        res = rng.next_u64();
    }
    let mut acc = 0u64;
    for (index, w) in weight.iter().enumerate() {
        acc += *w;
        if res < acc * x {
            return index;
        }
    }
    0
}

#[cfg(not(feature = "rand_proposer"))]
pub(crate) fn get_index(seed: u64, weight: &[u64]) -> usize {
    let sum: u64 = weight.iter().sum();
    let x = seed % sum;

    let mut acc = 0u64;
    for (index, w) in weight.iter().enumerate() {
        acc += *w;
        if x < acc {
            return index;
        }
    }
    0
}
