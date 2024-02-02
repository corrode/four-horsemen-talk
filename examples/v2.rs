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

trait HotelReader {
    fn read<'a>(&mut self, read: &'a mut dyn Read) -> Box<dyn Iterator<Item = Result<Hotel>> + 'a>;
}

struct CsvHotelReader {
    reader_builder: csv::ReaderBuilder,
}

impl CsvHotelReader {
    fn new() -> Self {
        CsvHotelReader {
            reader_builder: csv::ReaderBuilder::new(),
        }
    }
}

impl HotelReader for CsvHotelReader {
    fn read<'a>(&mut self, read: &'a mut dyn Read) -> Box<dyn Iterator<Item = Result<Hotel>> + 'a> {
        let reader = self
            .reader_builder
            .has_headers(false)
            .delimiter(b';')
            .from_reader(read);
        Box::new(
            reader
                .into_deserialize::<Hotel>()
                .map(|res| res.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)),
        )
    }
}

fn main() -> Result<()> {
    let mut file = std::fs::File::open("hotels.csv")?;
    let mut hotels = CsvHotelReader::new();

    let mut cities: HashMap<String, Price> = HashMap::new();

    for hotel in hotels.read(&mut file) {
        let hotel: Hotel = match hotel {
            Ok(hotel) => hotel,
            Err(err) => {
                eprintln!("Error reading hotel: {}", err);
                continue;
            }
        };

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
