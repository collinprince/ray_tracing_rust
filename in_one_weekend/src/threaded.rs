use crate::clio::get_thread_parameters;
use crate::color::write_color;
use crate::ray::Ray;
use crate::rtweekend::*;
use crate::scenes::{ray_color, ImageSettings, SceneSettings};
use crate::vec3::*;

use std::io::{self, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;

pub struct ThreadParameters {
    pub num_threads: usize,
    pub lines_per_thread: usize,
}

pub struct ThreadInput {
    pub thread_idx: i32,
    pub line_idx: i32,
    pub lines_per_thread: usize,
    pub scene_settings: Arc<SceneSettings>,
    pub tx: Sender<Vec<Color>>,
}

fn thread_work(thread_input: ThreadInput) {
    let ThreadInput {
        thread_idx: _,
        line_idx,
        lines_per_thread,
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
    let mut v: Vec<Color> = vec![];
    for j in (line_idx..(line_idx + lines_per_thread as i32)).rev() {
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
    }
    tx.send(v).unwrap();
}

pub fn multi_threaded(scene_settings: SceneSettings, thread_params: ThreadParameters) {
    let ThreadParameters {
        num_threads,
        lines_per_thread,
    } = thread_params;
    let ImageSettings {
        aspect_ratio: _,
        image_width: _,
        image_height,
        samples_per_pixel,
        max_depth: _,
    } = scene_settings.image_settings;

    let scene_settings: Arc<SceneSettings> = Arc::new(scene_settings);
    // cycle_size: the amount of lines that will computed in a cycle
    // if all theads are able to compute lines_per_thread number of lines
    let cycle_size: usize = num_threads * lines_per_thread;
    let image_height: usize = image_height as usize;
    for i in (0..(image_height - (image_height % cycle_size) + 1))
        .rev()
        .step_by(cycle_size)
    {
        eprintln!("running line {i}");
        // get how many threads should be run in this cycle,
        // accounting for first cycle where there may be less lines than
        // cycle_size
        let num_threads = (((i + cycle_size).min(image_height) - i) as f64
            / lines_per_thread as f64)
            .ceil() as i32;

        // create vector of channels (1 per thread)
        let (senders, receivers): (Vec<Sender<Vec<Color>>>, Vec<Receiver<Vec<Color>>>) = (0
            ..num_threads)
            .into_iter()
            .map(|_| mpsc::channel())
            .unzip();

        for (thread_idx, sender) in senders.into_iter().enumerate() {
            let start_of_thread_work = i + thread_idx * lines_per_thread;
            let end_of_thead_work = (start_of_thread_work + lines_per_thread).min(image_height);
            let lines_per_thread = end_of_thead_work - start_of_thread_work;
            let thread_input = ThreadInput {
                thread_idx: thread_idx as i32,
                line_idx: (i + (thread_idx * lines_per_thread)) as i32,
                scene_settings: Arc::clone(&scene_settings),
                tx: sender,
                lines_per_thread,
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

pub fn single_threaded(scene_settings: SceneSettings) {
    let SceneSettings {
        world,
        cam,
        image_settings,
    } = scene_settings;
    let ImageSettings {
        image_height,
        image_width,
        samples_per_pixel,
        max_depth,
        ..
    } = image_settings;

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = ((i as f64) + random_f64()) / ((image_width - 1) as f64);
                let v: f64 = ((j as f64) + random_f64()) / ((image_height - 1) as f64);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
}

// handle getting commandline multithreading parameters,
// then calling associated single or multithreaded behavior
pub fn render(scene_settings: SceneSettings) {
    // print image header
    print!(
        "P3\n{} {}\n255\n",
        scene_settings.image_settings.image_width, scene_settings.image_settings.image_height
    );

    // get parameters for multi threading if given
    let thread_params: ThreadParameters = get_thread_parameters();

    if thread_params.num_threads == 1 {
        single_threaded(scene_settings);
    } else {
        multi_threaded(scene_settings, thread_params);
    }

    eprintln!("\nDone.");
}
