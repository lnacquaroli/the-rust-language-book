use core::str;

// We leave the list and average fields private so there is no way for external code
// to add or remove items to or from the list field directly; otherwise, the average
// field might become out of sync when the list changes. The average method returns
// the value in the average field, allowing external code to read the average but
// not modify it.
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// The public methods add, remove, and average are the only ways to access or modify
// data in an instance of AveragedCollection. When an item is added to list using
// the add method or removed using the remove method, the implementations of each
// call the private update_average method that handles updating the average field as
// well.
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
