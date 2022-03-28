pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: 0.0,
        }
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

impl Default for AveragedCollection {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut avg = AveragedCollection::new();
    avg.add(4);
    avg.add(3);
    println!("{}", avg.average());
    avg.add(5);
    println!("{}", avg.average());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        let mut avg = AveragedCollection::new();
        assert_eq!(avg.average(), 0.0);
        avg.add(5);
        avg.add(4);
        avg.add(1);
        avg.add(2);
        assert_eq!(3.0, avg.average());
    }

    #[test]
    fn add_updates_average() {
        let mut avg = AveragedCollection::new();
        avg.add(10);
        assert_eq!(10.0, avg.average)
    }
}
