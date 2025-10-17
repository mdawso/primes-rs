pub struct Bounds {
    pub lower: u64,
    pub upper: u64
}


pub struct BinResult {
    pub bins: Vec<u64>,
    pub bounds: Vec<Bounds>
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

pub fn bin_dataset(dataset: &Vec<u64>, range: u64) -> BinResult {


    let mut bins: Vec<u64> = Vec::new();
    let bounds = get_bounds(range, *dataset.iter().max().unwrap());

    for bound in &bounds {
        bins.push(bin_bounds(&dataset, bound.lower, bound.upper));
    }

    BinResult { bins: bins, bounds: bounds }

}

pub fn get_bounds(range: u64, max: u64) -> Vec<Bounds> {
    let mut vec_bounds: Vec<Bounds> = Vec::new();
    for i in (0..max).step_by(range as usize) {
        vec_bounds.push(Bounds { lower: i, upper: i + range });
    }
    vec_bounds
}

pub fn print_bin_sizes(bin_result: &BinResult) {
    for (i, bound) in bin_result.bounds.iter().enumerate() {
        println!("{}-{}:{}", bound.lower, bound.upper, bin_result.bins[i]);
    }
}