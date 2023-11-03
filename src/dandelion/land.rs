use rand::Rng;
use rand_distr::{Distribution, Normal};

use super::dandelion::Dandelion;
pub struct Land {
    pub day: u32, //change the season based off this
    pub temperature: f32,
    pub humidity: f32,
    pub wind_speed: f32,
    pub normal: Normal<f32>,
    pub small_chance_normal_dist: Normal<f32>,
    pub rng: rand::rngs::ThreadRng,
    pub base_aridness: f64,
    pub one_in_this_num: usize,
}
impl Land {
    pub fn new(wind_speed: f32, temperature: f32, humidity: f32, aridness_constant: f64) -> Self {
        Land {
            day: 0,
            wind_speed,
            temperature,
            humidity,
            normal: Normal::new(2.0, 10. / 3.).unwrap(), //mean, standard deviation
            rng: rand::thread_rng(),
            base_aridness: aridness_constant,
            small_chance_normal_dist: Normal::new(2.0, 10. / 3.).unwrap(), //mean, standard deviation
            one_in_this_num: 2000, //0.05% chance
        }
    }
    pub fn tick(&mut self, land: &mut Vec<Dandelion>) -> Vec<Dandelion> {
        let mut new_dandelions = vec![];
        let am_to_spawn = self.calclulate_am_to_spawn_value(land.len());
        for dandelion in land {
            let mut dandelions = dandelion.tick(self, self.calclulate_am_to_spawn_value(am_to_spawn));
            new_dandelions.append(&mut dandelions);
        }
        self.day += 1;
        new_dandelions
    }
    fn calclulate_am_to_spawn_value(&self, num_dandelions: usize) -> usize {
        (((self.base_aridness + num_dandelions as f64) / self.humidity as f64) + 0.5) as usize
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
