// Kasiski analysis module
use std::collections::HashMap;

pub fn kasiski(message: &str) {
    let bytes = message.as_bytes();
    let l = bytes.len();

    // Collect positions of all patterns of length >= 3
    let mut map: HashMap<&[u8], Vec<usize>> = HashMap::new();
    for length in 3..=l / 2 {
        for start in 0..=l - length {
            let slice = &bytes[start..start + length];
            map.entry(slice).or_default().push(start);
        }
    }

    // Keep only patterns with ≥ 3 occurrences
    map.retain(|_, v| v.len() >= 3);

    // Distances between successive occurrences + GCD of these distances
    let mut pgcds: Vec<usize> = Vec::new();
    for positions in map.values() {
        let distances: Vec<usize> = positions.windows(2).map(|w| w[1] - w[0]).collect();
        if let Some(g) = distances.iter().copied().reduce(compute_gcd) {
            if g > 1 {
                pgcds.push(g);
            }
        }
    }

    // Count the most probable factors
    let mut freq: HashMap<usize, usize> = HashMap::new();
    for g in pgcds {
        *freq.entry(g).or_insert(0) += 1;
    }

    // Display sorted by decreasing frequency
    let mut items: Vec<(usize, usize)> = freq.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "Tailles des clés les plus probables (de la plus probable à la moins probable) : {:?}",
        items.iter().map(|(k, _)| *k).collect::<Vec<usize>>()
    );
}

pub fn compute_gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

