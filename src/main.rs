use std::*;
use crate::layer::*;
use crate::scene::*;

mod surface;
mod layer;
mod scene;
mod tup_math;

const SIZE: (usize, usize) = (100, 10);

fn main() {
    let mut scene = Scene::new(SIZE);
    scene.insert_layer(0, ConstructLayer('~',String::from("terrain")));
    scene.insert_layer(1, ConstructLayer('@', String::from("player")));
    let mut terrain = scene.get_layer_by_name("terrain".to_owned());
    terrain.random();
    scene.get_layer_by_name("player".to_owned()).set((0,0),true);
    scene.render();
}
