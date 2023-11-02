use glam::{Vec3, Vec2};
use tile_based_game::{prelude::*, assets::AssetServer, primitives::rect};
use rand_distr::Normal;

use crate::dandelion::{land::Land, dandelion::Dandelion};
const NUM_DAYS: u32 = 365;
mod dandelion {
    pub mod dandelion;
    pub mod land;
}
fn main() {
    pollster::block_on(run());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0)
    );
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(false, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //add models
    //custom mesh
    let mut asset_server = state.world.get_resource_mut::<AssetServer>().unwrap();
    let dandelion_idx = asset_server.compile_material("texture_atlas.png").await;
    let mut land = Land {
        day: 0,
        wind_speed: 0.0,
        temperature: 0.0,
        humidity: 0.0,
        rainfall: 0.0,
        soil_moisture: 0.0,
        max_num_dandelions: 1000,
        normal: Normal::new(2.0,10./3.).unwrap(), //mean, standard deviation
        rng: rand::thread_rng(),
        base_aridness: 0.3,
        
    };
    let mut dandelions = vec![];
    dandelions.push(Dandelion::new(50.,0.));
    for day in 0..NUM_DAYS {
    println!("Day {}", day);
    let mut new_dandelions = land.tick(&mut dandelions);
    dandelions.append(&mut new_dandelions);
    println!("Dandelions: {}", dandelions.len());
    }
    let mut dandelion_total = 0;
    for dandelion in dandelions {
    if dandelion.x > 0. && dandelion.x <= 100. && dandelion.y > 0. && dandelion.y <= 100. {
        dandelion_total += 1;
    }
    }
    println!("Number of total dandelions: {}", dandelion_total);
    let p1 = Vec2::new(-0.5,-0.5);
    let p2 = Vec2::new(0.5,0.5);
    let (vertices, indices) = rect(p1,p2);
    let instances = dandelions.map;
    asset_server.build_mesh(vertices, indices, instances, dandelion_idx, false);
    run_event_loop(state, event_loop);
}