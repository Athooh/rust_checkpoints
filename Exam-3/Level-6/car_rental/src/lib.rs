use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Car {
    pub color: String,
    pub plate: String,
}

#[derive(Debug)]
pub struct RentalBusiness {
    pub car: RefCell<Car>,
}

impl RentalBusiness {
    // Borrow the car immutably
    pub fn rent_car(&self) -> Ref<Car> {
        self.car.borrow()
    }

    // Take ownership of the car and replace it with an empty one
    pub fn sell_car(&self) -> Car {
        self.car.replace(Car::default())
    }

    // Borrow the car mutably
    pub fn repair_car(&self) -> RefMut<Car> {
        self.car.borrow_mut()
    }

    // Replace the car with a new one
    pub fn change_car(&self, new_car: Car) {
        self.car.replace(new_car);
    }
}