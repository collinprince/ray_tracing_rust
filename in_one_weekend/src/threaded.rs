use crate::color::write_color;
use crate::ray::Ray;
use crate::rtweekend::*;
use crate::scenes::{self, ray_color, ImageSettings, SceneSettings};
use crate::vec3::*;

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;

pub struct ThreadInput {
    pub thread_idx: i32,
    pub line_idx: i32,
    pub scene_settings: Arc<SceneSettings>,
    pub tx: Sender<Vec<Color>>,
}

fn thread_work(thread_input: ThreadInput) {
    let ThreadInput {
        thread_idx,
        line_idx,
        scene_settings,
        tx,
    } = thread_input;
    let SceneSettings {
        world,
        cam,
        image_settings,
    } = &*scene_settings;
    let ImageSettings {
        aspect_ratio: _,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    } = image_settings;
    let j: i32 = line_idx;
    let mut v: Vec<Color> = vec![];
    for i in 0..(*image_width) {
        let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..*samples_per_pixel {
            let u: f64 = ((i as f64) + random_f64()) / ((*image_width - 1) as f64);
            let v: f64 = ((j as f64) + random_f64()) / ((*image_height - 1) as f64);
            let r: Ray = cam.get_ray(u, v);
            pixel_color += ray_color(&r, &world, *max_depth);
        }
        v.push(pixel_color);
    }
    tx.send(v).unwrap();
}

pub fn multi_threaded() {
    let scene_settings: SceneSettings = scenes::defocus_blur_scene();
    let ImageSettings {
        aspect_ratio,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    } = scene_settings.image_settings;

    let scene_settings: Arc<SceneSettings> = Arc::new(scene_settings);

    let num_threads: usize = 4;

    for i in (0..image_height).rev().step_by(num_threads) {
        eprintln!("running line {i}");
        // get how many threads should be run in this cycle,
        // accounting for last cycle where there me may be less lines than
        // number_threads
        let num_threads = (i + num_threads as i32).min(image_height) - i;

        // create vector of channels (1 per thread)
        let (senders, receivers): (Vec<Sender<Vec<Color>>>, Vec<Receiver<Vec<Color>>>) = (0
            ..num_threads)
            .into_iter()
            .map(|_| mpsc::channel())
            .unzip();

        for (thread_idx, sender) in senders.into_iter().enumerate() {
            let thread_input = ThreadInput {
                thread_idx: thread_idx as i32,
                line_idx: i + thread_idx as i32,
                scene_settings: Arc::clone(&scene_settings),
                tx: sender,
            };
            thread::spawn(|| {
                thread_work(thread_input);
            });
        }

        for x in receivers.into_iter().rev() {
            let colors = x.recv().unwrap();
            for color in colors {
                write_color(color, samples_per_pixel);
            }
        }
    }
}
