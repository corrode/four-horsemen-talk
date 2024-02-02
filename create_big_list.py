#!/usr/bin/env python3

"""
Creates a CSV file with 1000 rows of random data lik this:

```
Rusty Rooms Residences;Brussels;2023-04-12;Deluxe;132.19
Ferris Motel One;Athens;2024-10-09;Family;227.43
Borrow Checker Bed & Breakfast;Vienna;2023-09-04;Deluxe;157.99
Cargo Cabin;Oslo;2023-01-24;Double;170.17
Syntax Suites;Zurich;2023-05-05;Deluxe;188.88
```
"""

import os
import sys
import random

ROWCOUNT = 1000_000_000

hotel_names = [
    "Rusty Rooms Residences",
    "Ferris Motel One",
    "Borrow Checker Bed & Breakfast",
    "Cargo Cabin",
    "Lifetime Lodge",
    "Mutable Manor",
    "Ownership Oasis",
    "Pattern Palace",
    "Trait Towers",
    "Async Apartments",
    "Binary Bungalows",
    "Crates Cottage",
    "Data Den",
    "Enum Estate",
    "Function Flat",
    "Generics Getaway",
    "Immutable Inn",
    "Loop Lodge",
    "Match Motel",
    "Reference Resort",
    "Slice Suites",
    "Stack Stay",
    "Struct Stayhouse",
    "Tuple Townhouse",
    "Variable Villa",
    "Macro Motel",
    "Module Mansion",
    "Safe Syntax Suites",
    "Type Town",
    "Unwind Undercroft",
    "Ferris Fortress",
    "Rustic Retreat",
    "Syntax Stay",
    "Memory Motel",
    "Code Cabin",
    "Cargo Cottage",
    "Rust Retreat",
    "Safe Stay",
    "Ownership Oasis",
    "Thread Townhouse",
    "Async Apartments",
    "Borrow Bungalows",
    "Stack Stay",
    "Reference Resort",
    "Trait Towers",
    "Macro Motel",
    "Data Den",
    "Lifetime Lodge",
    "Mutable Manor",
    "Enum Estate",
    "Match Motel",
    "Vector Villa",
    "Generics Getaway",
    "Function Flat",
    "Pattern Palace",
    "Crates Cabin",
    "Rust Roadhouse",
    "Syntax Suites",
    "Binary Bungalow",
    "Module Mansion",
    "Tuple Townhouse",
    "Loop Lodge",
    "Type Town",
    "Immutable Inn",
    "Ownership Oasis",
    "Trait Towers",
    "Vector Villa",
    "Async Apartments",
    "Binary Bungalows",
    "Cargo Cottage",
    "Data Den",
    "Enum Estate",
    "Function Flat",
    "Generics Getaway",
    "Immutable Inn",
    "Loop Lodge",
    "Match Motel",
    "Reference Resort",
    "Slice Suites",
    "Stack Stay",
    "Struct Stayhouse",
    "Tuple Townhouse",
    "Variable Villa",
]

cities = [
    "Athens",
    "Berlin",
    "Brussels",
    "Budapest",
    "Copenhagen",
    "Dublin",
    "Helsinki",
    "Istanbul",
    "Lisbon",
    "London",
    "Madrid",
    "Oslo",
    "Paris",
    "Prague",
    "Rome",
    "Stockholm",
    "Vienna",
    "Warsaw",
    "Zurich",
    "Amsterdam",
    "Barcelona",
    "Florence",
    "Geneva",
    "Krakow",
    "Munich",
    "Naples",
    "Nice",
    "Porto",
    "Seville",
    "Venice",
    "New York",
    "Los Angeles",
    "Chicago",
    "Houston",
    "Phoenix",
    "Philadelphia",
    "San Antonio",
    "San Diego",
    "Dallas",
    "San Jose",
    "Austin",
    "Jacksonville",
    "Fort Worth",
    "Columbus",
    "Charlotte",
    "San Francisco",
]

room_types = [
    "Single",
    "Double",
    "Family",
    "Suite",
    "Deluxe",
    "Executive",
]

years = [2023, 2024]
months = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
days = [
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
]



def main():
    print("Creating big_list.csv")
    with open("big_list.csv", "w") as f:
        for i in range(ROWCOUNT):
            hotel_name = random.choice(hotel_names)
            city = random.choice(cities)
            date = f"{random.choice(years)}-{random.choice(months):02d}-{random.choice(days):02d}"
            room_type = random.choice(room_types)
            price = round(random.uniform(100, 300), 2)

            f.write(f"{hotel_name};{city};{date};{room_type};{price}\n")

            print(f"{i+1}/{ROWCOUNT} rows written", end="\r")


if __name__ == "__main__":
    main()
