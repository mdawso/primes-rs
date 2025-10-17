pub fn primes_to(n: u64) -> Vec<u64> {
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