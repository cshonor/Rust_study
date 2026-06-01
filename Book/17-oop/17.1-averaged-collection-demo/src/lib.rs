//! `AveragedCollection`：用私有字段 + 公有方法演示封装（本书示例 17-1、17-2 思路）。

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
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
        if self.list.is_empty() {
            self.average = 0.0;
            return;
        }
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_tracks_list() {
        let mut c = AveragedCollection::new();
        c.add(10);
        c.add(20);
        c.add(30);
        assert!((c.average() - 20.0).abs() < f64::EPSILON);
        c.remove();
        assert!((c.average() - 15.0).abs() < f64::EPSILON);
    }
}
