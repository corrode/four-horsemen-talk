use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Hotel<'a> {
    name: &'a str,
    city: &'a str,
    date: &'a str,
    roomtype: &'a str,
    price: f32,
}

#[derive(Debug)]
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

impl Default for Price {
    fn default() -> Self {
        Price {
            min: f32::MAX,
            max: f32::MIN,
            sum: 0.0,
            count: 0,
        }
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
    let file = File::open("hotels.csv")?;
    let reader = BufReader::new(file);

    // City name and min/mean/max price
    let mut cities: HashMap<String, Price> = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        // Parse line
        let hotel_data: Vec<&str> = line.split(';').collect();
        let hotel = Hotel {
            name: hotel_data[0],
            city: hotel_data[1],
            date: hotel_data[2],
            roomtype: hotel_data[3],
            price: hotel_data[4].parse()?,
        };

        // Update price for city
        let price = cities
            .entry(hotel.city.to_string())
            .or_insert_with(Price::default);

        price.add(hotel.price);
    }

    println!("City min/mean/max prices:");
    for (city, price) in cities {
        println!("{city}: {price}");
    }
    Ok(())
}
