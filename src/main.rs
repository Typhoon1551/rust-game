use std::*;
use crate::layer::Layer;
use crate::scene::Scene;

mod surface;
mod layer;
mod scene;
mod tup_math;

const WIDTH : usize = 100;
const HEIGHT : usize = 10;

fn main() {
    let mut scene = Scene::new((WIDTH,HEIGHT),'@');
    scene.add_layer(0, '~', "terrain".to_string());
}
