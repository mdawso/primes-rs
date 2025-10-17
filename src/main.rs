mod primes;
mod bins;

fn main() {

    let primes_max = 10000;

    let primes: Vec<u64> = primes::primes_to(primes_max);

    // for prime in &primes {
    //     print!("{}, ", prime);
    // }

    println!();
    println!("Number of primes: {}", &primes.len());

    let bin_result = bins::bin_dataset(primes, 1000);
    // print_bins(&bin_result);
    bins::print_bin_sizes(&bin_result);

}
