pub fn primes_to(n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }

    let mut is_prime = vec![true; (n + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let limit = (n as f64).sqrt() as u64;
    for i in 2..=limit {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }

    is_prime.iter()
        .enumerate()
        .filter_map(|(idx, &prime)| if prime { Some(idx as u64) } else { None })
        .collect()
}