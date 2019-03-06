pub struct AveragedCollection {
    list: Vec<i32>,
    average: Option<f64>,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection { list: vec![], average: None }
    }

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
            },
            None => None,
        }
    }

    pub fn average(&self) -> Option<f64> {
        self.average
    }

    fn update_average(&mut self) {
        self.average = {
            if self.list.is_empty() {
                None
            } else {
                let total: i64 = self.list.iter().map(|&x| x as i64).sum();
                Some(total as f64 / self.list.len() as f64)
            }
        };  
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_works() {
        let mut collection = AveragedCollection::new();
        assert_eq!(collection.average(), None);
        collection.add(1);
        assert_eq!(collection.average(), Some(1.0));
        collection.add(3);
        assert_eq!(collection.average(), Some(2.0));
        collection.add(2);
        assert_eq!(collection.average(), Some(2.0));
        let popped = collection.remove();
        assert_eq!(popped, Some(2));
        assert_eq!(collection.average(), Some(2.0));
        let popped = collection.remove();
        assert_eq!(popped, Some(3));
        assert_eq!(collection.average(), Some(1.0));
        let popped = collection.remove();
        assert_eq!(popped, Some(1));
        assert_eq!(collection.average(), None);
        let popped = collection.remove();
        assert_eq!(popped, None);
    }
}
