use std::time::Instant;
use statrs::distribution::{ContinuousCDF, StudentsT};

const ALPHA:f64 = 0.05;

fn ruin_prob(starting_balance: i32, ending_balance: i32, p: f64) -> f64 {
    let a: i32 = starting_balance;
    let b: i32 = ending_balance - starting_balance;
    let r: f64 = (1.0 - p)/p;

    1.0 - (1.0-r.powi(-a)) / (r.powi(b) - r.powi(-a))
}


fn expected_tau(starting_balance: i32, ending_balance: i32, p: f64) -> f64 {
    let a: f64 = starting_balance as f64;
    let b: f64 = (ending_balance - starting_balance) as f64;
    let mu: f64 = 2.0*p-1.0;

    let prob: f64 = 1.0 - ruin_prob(starting_balance, ending_balance, p);

    ((b*prob) - a*(1.0-prob)) / (mu)
}

fn one_sample_t_test(t_stat: f64, df: i32) -> f64 {
    let dist: StudentsT = StudentsT::new(0.0, 1.0, df as f64).expect("invalid parameters");
    let below = dist.cdf(-1.0*t_stat.abs());
    2.0*below
}

fn simulate(starting_balance: i32, ending_balance: i32, p: f64) -> (i32, i32) {
    let mut balance = starting_balance;
    let mut n = 0;

    while balance < ending_balance && balance > 0 {
        let p_sim: f64 = rand::random();
        if p_sim < p {
            balance += 1;
        } else {
            balance -= 1;
        }
        n += 1;
    }

    let success = (balance == ending_balance) as i32;
    (success, n)
}


fn main() {
    let start_time: Instant = Instant::now();

    let starting_balance: i32 = 5;
    let ending_balance: i32 = 10;
    let p: f64 = 0.6;
    let n_sim: i32 = 1_000_000;


    let mut success_sum = 0.0;
    let mut tau_sum = 0.0;
    let mut tau_sqd_sum = 0.0;


    for _ in 1..=n_sim {
        let (success, n) = simulate(starting_balance, ending_balance, p);
        success_sum += success as f64;
        tau_sum += n as f64;
        tau_sqd_sum += n as f64 * n as f64;
    }

    let prob_ruin: f64 = 1.0 - success_sum / n_sim as f64;
    let prob_ruin_se = ((prob_ruin*(1.0-prob_ruin)) / (n_sim as f64)).powf(0.5);
    let prob_theor: f64 = ruin_prob(starting_balance, ending_balance, p);
    let t_stat_prob = (prob_ruin - prob_theor) / (prob_ruin_se);
    let p_value_prob: f64 = one_sample_t_test(t_stat_prob, n_sim-1);
    let hypothesis_test_prob: &str = if p_value_prob < ALPHA {"Reject Null"} else {"Fail to reject Null"};

    let tau_sim: f64 = tau_sum / n_sim as f64;
    let tau_sim_se = (((tau_sqd_sum / n_sim as f64) - (tau_sim * tau_sim)).powf(0.5)) / ((n_sim as f64).powf(0.5));
    let tau_theor: f64 = expected_tau(starting_balance, ending_balance, p);
    let t_stat_tau = (tau_sim - tau_theor) / (tau_sim_se);
    let p_value_tau: f64 = one_sample_t_test(t_stat_tau, n_sim-1);
    let hypothesis_test_tau: &str = if p_value_tau < ALPHA {"Reject Null"} else {"Fail to reject Null"};


    println!("The simulated probability of Ruin is {prob_ruin:.4} with SE {prob_ruin_se:.4} vs the theoratical prob {prob_theor:.4}");
    println!("The t-statistic is {t_stat_prob:.4} with a p-value of {p_value_prob:.4} and verdict {hypothesis_test_prob}\n");

    println!("The mean stopping time is {tau_sim:.4} with SE {tau_sim_se:.4} vs the theoratical expectation {tau_theor:.4}");
    println!("The t-statistic is {t_stat_tau:.4} with a p-value of {p_value_tau:.4} and verdict {hypothesis_test_tau}");

    let elapsed: std::time::Duration = start_time.elapsed();
    println!("{n_sim} simulations took {elapsed:.2?}");   
}
