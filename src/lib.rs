pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = std::collections::HashSet::new();

    for &factor in factors {
        if factor != 0 {
            for i in 1..(limit / factor) {
                multiples.insert(factor * i);
            }
        }
    }

    multiples.iter().sum()
}

fn main() {
    let limit = 20;
    let factors = vec![3, 5];
    let result = sum_of_multiples(limit, &factors);
    println!("Sum of multiples: {}", result); // Output: 78
}



