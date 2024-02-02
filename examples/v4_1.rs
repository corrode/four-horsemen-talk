use csv;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
struct Hotel {
    name: String,
    city: String,
    date: String,
    roomtype: String,
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

fn parse_cities(rdr: impl Read) -> Result<HashMap<String, Price>> {
    let mut hotels = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_reader(rdr);

    // City name and min/mean/max price
    let mut cities: HashMap<String, Price> = HashMap::new();

    for hotel in hotels.deserialize() {
        let hotel: Hotel = match hotel {
            Ok(hotel) => hotel,
            Err(err) => {
                eprintln!("Error reading hotel: {}", err);
                continue;
            }
        };

        // Update price for city
        let price = cities
            .entry(hotel.city.to_string())
            .or_insert_with(Price::default);

        price.add(hotel.price);
    }

    Ok(cities)
}

fn main() -> Result<()> {
    let rdr = std::fs::File::open("hotels.csv")?;
    let cities = parse_cities(rdr);

    println!("City min/mean/max prices:");
    for (city, price) in cities? {
        println!("{:?}: {}", city, price);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cities() {
        let input = b"Hotel;City;Date;RoomType;Price\n\
            Hilton;Berlin;2019-01-01;Single;100.00\n\
            Hilton;Berlin;2019-01-01;Double;200.00\n\
            Hilton;Berlin;2019-01-01;Suite;300.00\n\
            Hilton;Berlin;2019-01-02;Single;100.00\n\
       ";

        let cities = parse_cities(&input[..]).unwrap();
        assert_eq!(cities.len(), 1);
        assert_eq!(cities.get("Berlin").unwrap().min, 100.0);
        assert_eq!(cities.get("Berlin").unwrap().max, 300.0);
        assert_eq!(cities.get("Berlin").unwrap().sum, 700.0);
        assert_eq!(cities.get("Berlin").unwrap().count, 4);
    }
}
