use crate::land::*;
use rand::prelude::*;

const NUM_SEEDS_LEFT: u32 = 250;
#[derive(Clone, Copy, Debug)]
pub struct Dandelion {
    pub age: u32,
    pub x: f32,
    pub y: f32,
    pub num_seeds_left: u32,
    pub is_bloomed: bool,
    pub am_times_bloomed: u32,
}
impl Dandelion {
    pub fn new(x: f32, y: f32) -> Self {
        Dandelion {
            x,
            y,
            age: 0,
            num_seeds_left: NUM_SEEDS_LEFT,
            is_bloomed: false,
            am_times_bloomed: 0,
        }
    }
    pub fn tick(&mut self, land: &mut Land, aridness: f64) -> Vec<Dandelion> {
        self.generate_normal_dist();
        let mut dandelions = vec![];
        if self.is_bloomed {
            for _ in 0..((9.*aridness)+0.5) as usize {
                //because it spreads 9 per day
                let dandelion = self.spread_seed(land);
                dandelions.push(dandelion);
                self.num_seeds_left -= 1;
                if self.num_seeds_left == 0 {
                    self.num_seeds_left = NUM_SEEDS_LEFT;
                    self.is_bloomed = false;
                }
            }
        }
        self.age += 1;
        if self.age >= 90+(28*self.am_times_bloomed) {
            self.is_bloomed = true;
            self.am_times_bloomed += 1;
        }
        dandelions
    }
    fn generate_normal_dist(&mut self) {
        //this is where you generate the normal function each day based off the wind and stuff
    }
    fn spread_seed(&mut self, land: &mut Land) -> Dandelion {
        let dist = land.seed_normal();
        let dir = land.rng.gen_range(0..360);
        let (x, y) = calculate_triangle_sides(dist, dir as f32);
        Dandelion::new(x, y)
    }
}
fn calculate_triangle_sides(hypotenuse: f32, degrees: f32) -> (f32, f32) {
    let radians = degrees.to_radians();
    let side_a = hypotenuse * radians.sin();
    let side_b = hypotenuse * radians.cos();
    (side_a, side_b)
}