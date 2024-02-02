use std::{collections::HashMap, fs};

fn main() {
    let hotels = fs::read_to_string("hotels.csv").unwrap();

    // City name and min/mean/max price
    let mut cities: HashMap<String, (f32, f32, f32)> = HashMap::new();

    for hotel in hotels.lines() {
        let hotel_data: Vec<&str> = hotel.split(";").collect();
        if hotel_data.len() != 5 {
            continue;
        }
        let city = hotel_data[1];
        let price = hotel_data[4];

        let price: f32 = match price.parse() {
            Ok(price) => price,
            Err(_) => {
                eprintln!("Invalid price: {}", price);
                continue;
            }
        };

        // Update price for city
        let city_data = cities
            .entry(city.to_string())
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
}
