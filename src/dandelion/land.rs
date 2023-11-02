use crate::dandelion::*;
use rand_distr::{Normal,Distribution};
pub struct Land {
  pub day: u32, //change the season based off this
  pub temperature: f32,
  pub humidity: f32,
  pub rainfall: f32,
  pub soil_moisture: f32,
  pub wind_speed: f32,
  pub max_num_dandelions: u32, //at a certain point no more dandelions can grow
  pub normal: Normal<f32>,
  pub rng: rand::rngs::ThreadRng,
  pub base_aridness: f64,
}
impl Land {
  pub fn tick(&mut self, land: &mut Vec<Dandelion>) -> Vec<Dandelion>{
    let mut new_dandelions = vec![];
    let aridness = self.base_aridness * land.len() as f64;
    for dandelion in land {
      let mut dandelions = dandelion.tick(self, aridness);
      new_dandelions.append(&mut dandelions);
    }
    self.day += 1;
    new_dandelions
  }
  pub fn seed_normal(&mut self) -> f32 {
    self.normal.sample(&mut self.rng)
  }
}