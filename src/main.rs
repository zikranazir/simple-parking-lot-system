use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MAX_PARKING_SLOT: usize = 5;

fn main() {
    let parking_count = Arc::new(Mutex::new(0));
    let is_parking_full = Arc::new(Mutex::new(false));

    for i in 0..10 {
        let parking_count = parking_count.clone();
        let is_parking_full = is_parking_full.clone();

        thread::spawn(move || {
            let parked = park_car(&parking_count, &is_parking_full);
            println!("Car {} {}", i + 1, parked);
            thread::sleep(Duration::from_millis(100));
            if parked {
                leave_car(&parking_count, &is_parking_full);
            }
        });
    }

    loop {
        let count = *parking_count.lock().unwrap();
        let full = *is_parking_full.lock().unwrap();
        println!("Parking Count: {}", count);
        if full {
            println!("Parking Lot is Full!");
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn park_car(parking_count: &Arc<Mutex<usize>>, is_parking_full: &Arc<Mutex<bool>>) -> bool {
    let mut parked = false;
    let mut count = parking_count.lock().unwrap();
    let mut full = is_parking_full.lock().unwrap();

    if *count < MAX_PARKING_SLOT {
        *count += 1;
        println!("Parked a car. Parking Count: {}", *count);
        if *count == MAX_PARKING_SLOT {
            *full = true;
            println!("Parking Lot is Full!");
        }
        parked = true;
    }
    parked
}

fn leave_car(parking_count: &Arc<Mutex<usize>>, is_parking_full: &Arc<Mutex<bool>>) {
    let mut count = parking_count.lock().unwrap();
    let mut full = is_parking_full.lock().unwrap();

    if *count > 0 {
        *count -= 1;
        println!("A car left. Parking Count: {}", *count);
        if *full && *count < MAX_PARKING_SLOT {
            *full = false;
            println!("Parking Lot is not Full!");
        }
    }
}
