mod camera;
mod clio;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod scenes;
mod sphere;
mod threaded;
mod vec3;

use scenes::SceneSettings;

fn main() {
    // get scene, cam, and settings
    let scene_settings: SceneSettings = scenes::defocus_blur_scene();
    // render
    threaded::render(scene_settings);
}
