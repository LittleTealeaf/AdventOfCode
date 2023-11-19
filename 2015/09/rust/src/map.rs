pub struct Map {
    cities: Vec<String>,
    distances: Vec<([String; 2], i32)>,
}

impl Map {
    pub fn get_cities(&self) -> &Vec<String> {
        &self.cities
    }

    pub fn get_distance(&self, from: String, to: String) -> Option<i32> {
        self.distances
            .iter()
            .find_map(|dist| (dist.0.contains(&from) && dist.0.contains(&to)).then_some(dist.1))
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut cities = Vec::new();
        let mut distances = Vec::new();

        for line in value.lines() {
            let mut tokens = line.split(" ");
            let from = tokens.next().unwrap().to_string();
            let _ = tokens.next();
            let to = tokens.next().unwrap().to_string();
            let _ = tokens.next();
            let distance = tokens.next().unwrap().parse::<i32>().unwrap();

            if !cities.contains(&from) {
                cities.push(from.clone());
            }

            if !cities.contains(&to) {
                cities.push(to.clone());
            }

            distances.push(([from, to], distance));
        }

        Self { cities, distances }
    }
}
