use rand::Rng;
use rand_distr::{Distribution, Normal};

use super::dandelion::Dandelion;
pub struct Land {
    pub day: u32, //change the season based off this
    pub humidity: f32,
    pub wind_speed: f32,
    pub normal: Normal<f32>,
    pub small_chance_normal_dist: Normal<f32>,
    pub rng: rand::rngs::ThreadRng,
    pub base_aridness: f32,
    pub one_in_this_num: usize,
    pub dandelions_per_meter: [[u32; 200]; 200],
}
impl Land {
    pub fn new(wind_speed: f32, humidity: f32, aridness_constant: f32) -> Self {
        Land {
            day: 0,
            wind_speed,
            humidity,
            normal: Normal::new(2.0, 10. / 3.).unwrap(), //mean, standard deviation
            rng: rand::thread_rng(),
            base_aridness: aridness_constant,
            small_chance_normal_dist: Normal::new(50.0, 150. / 3.).unwrap(), //mean, standard deviation
            one_in_this_num: 200, //0.05% chance
            dandelions_per_meter: [[0; 200]; 200]
        }
    }
    pub fn tick(&mut self, land: &mut Vec<Dandelion>) -> Vec<Dandelion> {
        let mut new_dandelions = vec![];
        let am_to_spawn = self.calclulate_am_to_spawn_value(land.len());
        self.calculate_normal_dist();
        for dandelion in land {
            let mut dandelions = dandelion.tick(self, am_to_spawn);
            new_dandelions.append(&mut dandelions);
        }
        self.day += 1;
        new_dandelions
    }
    fn calculate_aridness(&self, num_dandelions: usize, temp: f32) -> f32 {
        let top = self.base_aridness * num_dandelions as f32;
        let aridness = top/(temp*self.humidity);
        aridness
    }
    fn calculate_normal_dist(&mut self) {
        self.normal = Normal::new(5.0, (50. * self.wind_speed * self.humidity) / 3.).unwrap()
    }
    fn calclulate_am_to_spawn_value(&self, num_dandelions: usize) -> usize {
        let temp = self.temperature();
        let top = 25. * self.humidity * self.wind_speed * temp;
        let bottom = top / (3. * self.calculate_aridness(num_dandelions, temp));
        (bottom + 0.5) as usize //round it
    }
    fn temperature(&self) -> f32 {
        let x = self.day as f32;
        let x_squared = -0.00137 * x*x;
        let temp = (x_squared + 0.577*x + 13.6)/59.;
        temp
    }
    pub fn seed_normal(&mut self) -> f32 {
        let rand_chance_to_go_far = self.rng.gen_range(0..self.one_in_this_num); // this is the small chance that it goes further than 100 m
        let dist = if rand_chance_to_go_far == 1 {
            println!("SMALL CHANCE TO GO FAR");
            self.small_chance_normal_dist.sample(&mut self.rng)
        } else {
            self.normal.sample(&mut self.rng)
        };
        dist
    }
}
