use csv;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

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
    fn read(&mut self) -> Result<Box<dyn Iterator<Item = Result<Hotel>> + '_>>;
}

struct CsvHotelReader {
    reader: csv::Reader<File>,
}

impl CsvHotelReader {
    fn new(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_reader(file);
        Ok(CsvHotelReader { reader })
    }
}

impl HotelReader for CsvHotelReader {
    fn read(&mut self) -> Result<Box<dyn Iterator<Item = Result<Hotel>> + '_>> {
        let iter = self
            .reader
            .deserialize::<Hotel>()
            .map(|res| res.map_err(|e| Box::new(e) as Box<dyn std::error::Error>));
        Ok(Box::new(iter))
    }
}

fn main() -> Result<()> {
    let mut hotels = CsvHotelReader::new("hotels.csv")?;

    let mut cities: HashMap<String, Price> = HashMap::new();

    for hotel in hotels.read()? {
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
