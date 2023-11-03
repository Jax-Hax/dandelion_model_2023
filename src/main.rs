use glam::{Vec2, Vec3};
use tile_based_game::{assets::AssetServer, prelude::*, primitives::rect};

use crate::dandelion::{dandelion::Dandelion, land::Land};
const NUM_DAYS: u32 = 96;
const ZOOM_OUT: f32 = 20.;
mod dandelion {
    pub mod dandelion;
    pub mod land;
}
fn main() {
    pollster::block_on(run());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0));
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(false, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //add models
    //custom mesh
    let mut asset_server = state.world.get_resource_mut::<AssetServer>().unwrap();
    let dandelion_idx = asset_server.compile_material("cube-diffuse.jpg").await;
    let mut land = Land::new(0.0,0.0,0.0,0.3);
    let mut dandelions = vec![];
    dandelions.push(Dandelion::new(50., 0.));
    for day in 0..NUM_DAYS {
        println!("Day {}", day);
        let mut new_dandelions = land.tick(&mut dandelions);
        dandelions.append(&mut new_dandelions);
        println!("Dandelions: {}", dandelions.len());
    }
    let mut dandelion_total = 0;
    for dandelion in &dandelions {
        if dandelion.x > 0. && dandelion.x <= 100. && dandelion.y > 0. && dandelion.y <= 100. {
            dandelion_total += 1;
        }
    }
    println!("Number of total dandelions: {}", dandelion_total);
    let p1 = Vec2::new(-0.01, -0.01);
    let p2 = Vec2::new(0.01, 0.01);
    let (vertices, indices) = rect(p1, p2);
    let mut instances2: Vec<Instance> = dandelions
        .iter_mut()
        .map(|dandelion| Instance {
            position: Vec3::new(dandelion.x / ZOOM_OUT, dandelion.y / ZOOM_OUT, 0.),
            enabled: (dandelion.x > 0. && dandelion.x <= 100. && dandelion.y > 0. && dandelion.y <= 100.),
            ..Default::default()
        })
        .collect();
    asset_server.build_mesh(
        vertices,
        indices,
        instances2.iter_mut().map(|instance| instance).collect(),
        dandelion_idx,
        false,
    );


    //100x100 final grid
    let p1 = Vec2::new(0., 0.);
    let p2 = Vec2::new(100. / ZOOM_OUT,100. / ZOOM_OUT);
    let (vertices, indices) = rect(p1, p2);
    asset_server.build_mesh(
        vertices,
        indices,
        vec![&mut Instance {position: Vec3::new(0.,0.,0.), ..Default::default()}],
        dandelion_idx,
        false,
    );
    run_event_loop(state, event_loop);
}
