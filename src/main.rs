struct ParkingLot {
    capacity: usize,
    occupied_spaces: usize,
}

impl ParkingLot {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            occupied_spaces: 0,
        }
    }

    fn park_car(&mut self) -> bool {
        if self.occupied_spaces < self.capacity {
            self.occupied_spaces += 1;
            true
        } else {
            println!("Parking lot is full.");
            false
        }
    }

    fn exit_car(&mut self) -> bool {
        if self.occupied_spaces > 0 {
            self.occupied_spaces -= 1;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut parking_lot = ParkingLot::new(10);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), true);
    assert_eq!(parking_lot.park_car(), false); // Parking lot is full
    assert_eq!(parking_lot.exit_car(), true);
    assert_eq!(parking_lot.exit_car(), true);
    assert_eq!(parking_lot.exit_car(), true);
    assert_eq!(parking_lot.exit_car(), true);
    assert_eq!(parking_lot.exit_car(), true);
    assert_eq!(parking_lot.exit_car(), true);
    assert_eq!(parking_lot.exit_car(), true);
    assert_eq!(parking_lot.exit_car(), true);
    assert_eq!(parking_lot.exit_car(), true);
    assert_eq!(parking_lot.exit_car(), false); // Parking lot is empty
}
