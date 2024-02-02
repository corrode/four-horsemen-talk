use ahash::AHashMap;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::io::Result;

#[derive(Debug, Default)]
struct Price {
    min: f32,
    max: f32,
    sum: f64,
    count: usize,
}

impl Price {
    fn add(&mut self, price: f32) {
        self.min = self.min.min(price);
        self.max = self.max.max(price);
        self.count += 1;
        self.sum += price as f64;
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{min:.2}/{mean:.2}/{max:.2}",
            min = self.min,
            mean = self.sum / self.count as f64,
            max = self.max
        )
    }
}

fn main() -> Result<()> {
    let file = File::open("big_list.csv")?;
    let mmap = unsafe { Mmap::map(&file)? };

    let results: AHashMap<Vec<u8>, Price> = mmap
        .par_split(|&b| b == b'\n')
        .filter_map(|line| {
            let parts: Vec<&[u8]> = line.split(|&b| b == b';').collect();
            if parts.len() >= 5 {
                if let Ok(price) = std::str::from_utf8(parts[4]).unwrap_or("0").parse::<f32>() {
                    let city = parts[1].to_vec();
                    return Some((city, price));
                }
            }
            None
        })
        .fold(
            || AHashMap::new(),
            |mut map, (city, price)| {
                map.entry(city).or_insert_with(Price::default).add(price);
                map
            },
        )
        .reduce(
            || AHashMap::new(),
            |mut map1, map2| {
                for (city, price_struct) in map2 {
                    map1.entry(city)
                        .and_modify(|e| e.add(price_struct.min)) // Assuming you want to add the minimum price of `price_struct` to the existing price
                        .or_insert(price_struct);
                }
                map1
            },
        );

    println!("City min/mean/max prices:");
    for (city, price) in results {
        println!("{:?}: {}", String::from_utf8_lossy(&city), price);
    }

    Ok(())
}
