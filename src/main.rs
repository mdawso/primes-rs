use std::time::Instant;

mod primes;
mod bins;


fn main() {
    
    let primes_max = 1000000000;
    let range = primes_max / 10; // range = bin size

    println!("Started calculating primes");
    let primes_start_time = Instant::now();
    let primes: Vec<u64> = primes::primes_to(primes_max);
    let primes_duration = primes_start_time.elapsed();

    // for prime in &primes {
    //     print!("{}, ", prime);
    // }

    println!("Started binning");
    let bin_start_time = Instant::now();
    let bin_result = bins::bin_dataset(&primes, range);
    let bin_duration = bin_start_time.elapsed();

    bins::print_bin_sizes(&bin_result);
    println!("-----------------------------------------");
    println!("Number of primes: {}", &primes.len());
    println!("Number of bins: {}", &bin_result.bins.len());
    println!("-----------------------------------------");
    println!("Time to calc primes: {:?}", &primes_duration);
    println!("Time to calc bins: {:?}", &bin_duration);
    println!("-----------------------------------------");
}
