use std::{collections::HashMap, fs};

fn main() {
    let hotels = fs::read_to_string("hotels.csv").unwrap();

    // City name and min/mean/max price
    let mut cities: HashMap<String, (f32, f32, f32, usize)> = HashMap::new();

    for hotel in hotels.lines() {
        let hotel_data: Vec<&str> = hotel.split(";").collect();
        let _name = hotel_data[0];
        let city = hotel_data[1];
        let _date = hotel_data[2];
        let _roomtype = hotel_data[3];
        let price = hotel_data[4];

        let price: f32 = price.parse().unwrap();

        // Update price for city.
        if let Some(city_data) = cities.get_mut(&city.to_string()) {
            // Update min
            if price < city_data.0 {
                city_data.0 = price;
            }
            // Update max
            if price > city_data.2 {
                city_data.2 = price;
            }
            // Update mean
            city_data.1 = (city_data.1 + price) / 2.0;
        } else {
            cities.insert(city.to_string(), (price, price, price, 1));
        }
    }

    println!("City min/mean/max prices:");
    for (city, (min, mean, max, _)) in cities {
        println!("{}: {}€/{}€/{}€", city, min, mean, max);
    }
}
