
struct Bounds {
    lower: u64,
    upper: u64
}


struct BinResult {
    bins: Vec<u64>,
    bounds: Vec<Bounds>
}

fn bin_bounds(dataset: &Vec<u64>, lower: u64, upper: u64) -> u64 {
    let mut count: u64 = 0;
    for num in dataset {
        if lower < *num && *num < upper {
            count += 1
        }
    }
    count
}

fn bin_dataset(dataset: Vec<u64>, range: u64) -> BinResult {


    let mut bins: Vec<u64> = Vec::new();
    let bounds = get_bounds(range, *dataset.iter().max().unwrap());

    for bound in &bounds {
        bins.push(bin_bounds(&dataset, bound.lower, bound.upper));
    }

    BinResult { bins: bins, bounds: bounds }

}

fn get_bounds(range: u64, max: u64) -> Vec<Bounds> {
    let mut vec_bounds: Vec<Bounds> = Vec::new();
    for i in (0..max).step_by(range as usize) {
        vec_bounds.push(Bounds { lower: i, upper: i + range });
    }
    vec_bounds
}

fn print_bin(lower: u64, upper: u64, count: u64) {
    println!(
        "Primes between {}, {}: {}",
        lower,
        upper,
        count
    );
}

fn print_bins(bin_result: &BinResult) {
    for (i, bin) in bin_result.bins.iter().enumerate() {
        let bound = &bin_result.bounds[i];
        print_bin(bound.lower, bound.upper, *bin);
    }
}

fn print_bin_sizes(bin_result: &BinResult) {
    let sizes: Vec<String> = bin_result.bins.iter().map(|bin| bin.to_string()).collect();
    println!("{}", sizes.join(","));
}

fn primes_to(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();

    for i in 2..=n {
        if (2..=(
                (i as f64).sqrt() as u64)
                ).all
                (
                    |j: u64| i % j != 0
                ) {
            primes.push(i);
        }
    }

    primes
}

fn main() {

    let primes_max = 10000;

    let primes: Vec<u64> = primes_to(primes_max);

    // for prime in &primes {
    //     print!("{}, ", prime);
    // }

    println!();
    println!("Number of primes: {}", &primes.len());

    let bin_result = bin_dataset(primes, 1000);
    // print_bins(&bin_result);
    print_bin_sizes(&bin_result);

}
