use csv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
struct Hotel {
    name: String,
    city: String,
    date: String,
    roomtype: String,
    price: f32,
}

fn main() -> Result<()> {
    let mut hotels = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path("hotels.csv")?;

    // City name and min/mean/max price
    let mut cities: HashMap<String, (f32, f32, f32)> = HashMap::new();

    for hotel in hotels.deserialize() {
        let hotel: Hotel = match hotel {
            Ok(hotel) => hotel,
            Err(err) => {
                eprintln!("Error reading hotel: {}", err);
                continue;
            }
        };

        let price: f32 = hotel.price;

        // Update price for city
        let city_data = cities
            .entry(hotel.city.to_string())
            .or_insert((price, price, price));
        let (min, mean, max) = city_data;
        if price < *min {
            *min = price;
        }
        if price > *max {
            *max = price;
        }
        *mean = (*mean + price) / 2.0;
    }

    println!("City min/mean/max prices:");
    for (city, (min, mean, max)) in cities {
        println!("{}: {}€/{}€/{}€", city, min, mean, max);
    }
    Ok(())
}
