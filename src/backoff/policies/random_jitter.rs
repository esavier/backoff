pub use rand::distr;
use rand::{Rng, rng};

use crate::backoff::{
    clock::Clock, core::BackoffTimer, policies::traits::BackoffPolicyResult,
    policies::traits::RetryPolicy, runnable::Runnable,
};

pub struct RandomJitter {
    max: u64,
    distribution: RandomnessDistribution,
}

pub enum RandomnessDistribution {
    // even distribution, same cahnces for anything from min to max
    Uniform,
    // another example would be
    // exponent
    // hypergeometric
    // binomial
    Gaussian,
    // poisson
    // gamma
    // todo: add more
}

// random jitter in miliseconds,
// find out random value from between 0 to max
// and applies it to the current milis
impl RandomJitter {
    pub fn new(max: u64, distribution: RandomnessDistribution) -> Self {
        Self { max, distribution }
    }

    fn randomize(&self, milis: u64) -> u64 {
        match self.distribution {
            RandomnessDistribution::Uniform => {
                let modifier: f64 = rng().random_range(0.0..1.0);
                let deviation = (modifier * (self.max as f64)) as u64;
                milis + deviation
            }
            // rust lacks gaussian distribution by default, we can do that with either
            // ziggurat alghorithm or Box-Muller transform
            RandomnessDistribution::Gaussian => {
                // we try to do it with Box-Muller transform
                // ziggurat alghorithm is a bit harder to implement/test
                let normal_1: f64 = rng().random_range(0.0..1.0);
                let normal_2: f64 = rng().random_range(0.0..1.0);
                let deviation = (normal_1.sqrt() * normal_2.sqrt() * self.max as f64) as u64;
                milis + deviation
            }
        }
    }
}

impl<C, R> RetryPolicy<C, R> for RandomJitter
where
    C: Clock,
    R: Runnable,
{
    fn apply(&self, backoff: &BackoffTimer<C, R>) -> BackoffPolicyResult {
        let new_milis = self.randomize(backoff.current_loop_milis);
        println!(
            "   Policy::RandomJitter: old milis {}, new millis {}",
            backoff.current_loop_milis, new_milis
        );
        BackoffPolicyResult::SetMilis(new_milis)
    }
}
