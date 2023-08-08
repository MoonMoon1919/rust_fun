pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_is_updated_when_items_added() {
        let mut ac = AveragedCollection{ list: vec![], average: 0.0 };
        ac.add(2);
        ac.add(4);
        ac.add(6);

        assert_eq!(ac.average(), 4.0)
    }

    #[test]
    fn average_is_updated_when_items_removed() {
        let mut ac = AveragedCollection{ list: vec![], average: 0.0 };

        ac.add(2);
        ac.add(4);
        ac.add(6);

        assert_eq!(ac.average(), 4.0);

        ac.remove();

        assert_eq!(ac.average(), 3.0);
    }
}
