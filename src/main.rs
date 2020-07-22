#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io;
use std::fs::File;
//use std::io::prelude::*;
use std::io::Write; //to flush stdout
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;

//use cgmath::prelude::*;
use cgmath::prelude::InnerSpace;
//use cgmath::Vector3;

use pixel_canvas::{Canvas, Color};

use rayruster::raytracing::{Ray2, Vec3, Color2, Point32, Intersectable, IntersectableList, Camera};
use rayruster::figures::*;
use rayruster::{settings, utils::*};
/*
fn random_double() -> f32 {
    distr.sample(&mut rng)
}*/

fn write_color(image_ascii_data: &mut String, pixel_color: &Color2, samples_per_pixel: i32) {

    let scale = 1.0 / samples_per_pixel as f32;

    let ir = (256.0 * clamp(pixel_color.x * scale, 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(pixel_color.y * scale, 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(pixel_color.z * scale, 0.0, 0.999)) as i32;

    image_ascii_data.push_str(&format!("{} {} {}\n", ir, ig, ib));
}

fn ray_color(r: Ray2, world: &dyn Intersectable) -> Color2 {

    let opt_hitrec = world.intersect(&r, 0.0, INFINITY);

    match opt_hitrec {
        Some(hit_rec) => {        
            //let n = (hit_rec.p - Vec3::new(0.0, 0.0, -1.0)).normalize();
            let n = hit_rec.normal;
            0.5 * Color2::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
        },
        None => {
            let unit_direction: Vec3 = r.dir.normalize(); //convertir a vector unitari
            let t = 0.5 * (unit_direction.y + 1.0); //mappejar l'intèrval [-1,1] a [0,1]
            (1.0 - t) * Color2::new(1.0, 1.0, 1.0) + t * Color2::new(0.5, 0.7, 1.0)
        }
    }
}

fn process_cli_parameters() -> (settings::DisplayMode, i8, i16, i16) {
    let display_mode: settings::DisplayMode = match std::env::args().nth(1) {
        Some(r) => match r.as_str() {
            "-w" => settings::DisplayMode::WINDOW,
            "-f" => settings::DisplayMode::FILE,
            _ => settings::DisplayMode::FILE,

        },
        None => settings::DisplayMode::FILE,
    };
    
    let raytracing_depth: i8 = match std::env::args().nth(2) {
        Some(r) => match r.parse::<i8>() {
            Ok(depth) => if depth < 1 { settings::DEFAULT_RAYTRACING_DEPTH } else { depth },
            Err(e) => settings::DEFAULT_RAYTRACING_DEPTH,
        },
        None => settings::DEFAULT_RAYTRACING_DEPTH,
    };

    let width: i16 = match std::env::args().nth(3) {   
        Some(r) => match r.parse::<i16>() {
            Ok(width) => if width < 1 { settings::DEFAULT_WIDTH } else { width },
            Err(e) => settings::DEFAULT_WIDTH,
        },
        None => settings::DEFAULT_WIDTH,
    };

    let height: i16 = match std::env::args().nth(4) {
        Some(r) => match r.parse::<i16>() {
            Ok(height) => if height < 1 { settings::DEFAULT_HEIGHT } else { height },
            Err(e) => settings::DEFAULT_HEIGHT,
        },
        None => settings::DEFAULT_HEIGHT,
    };

    (display_mode, raytracing_depth, width, height)
}

fn get_settings_from_run_parameters() -> settings::Settings {
    let mut settings = settings::Settings::new();
    
    let (display_mode, ray_depth, width, height) = process_cli_parameters();
    
    settings.display_mode = display_mode;
    settings.ray_depth = ray_depth;
    settings.width = width;
    settings.height = height;
    
    settings
}

fn main() -> std::io::Result<()> {

    // Settings retreive from run parameters.
    let settings = get_settings_from_run_parameters();
    let mut raytraced_color_buffer: Vec<Color2> = vec!();

    //output
    let mut file = File::create("image.ppm")?;
    let mut image_ascii_data: String = "".to_owned();
    let header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    image_ascii_data.push_str(&header);

    //image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    //world
    let mut world = IntersectableList{objects: Vec::<Box<dyn Intersectable>>::new()};
    world.add(Box::new(Sphere{center: Point32::new(0.0,0.0,-1.0), radius: 0.5}));
    world.add(Box::new(Sphere{center: Point32::new(0.0,-100.5,-1.0), radius: 100.0}));

    //camera
    let camera = Camera::new();

    //misc
    let mut rng = rand::thread_rng();
    
    let distr = Uniform::from(0.0..1.0);

    for j in (0..IMAGE_HEIGHT).rev() {
        //println!("{}", j);
        //println!("\rScanlines remaining: {} ", j); //commented out while the \r thing doesn't work
        std::io::stdout().flush().expect("error flushing stdout");
        for i in 0..IMAGE_WIDTH {

            let mut pixel_color = Color2::new(0.0,0.0,0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
                let r = camera.get_ray(u,v);
                pixel_color += ray_color(r, &world);
            }

            //let pixel_color = ray_color(r, &world);
            write_color(&mut image_ascii_data, &pixel_color, SAMPLES_PER_PIXEL);
            raytraced_color_buffer.push(pixel_color);
        }
    }

    file.write_all(image_ascii_data.as_bytes())?;
    
    match settings.display_mode { 
        settings::DisplayMode::WINDOW => {
            let canvas = Canvas::new(IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize)
            .title("RayRuster 0.7");

            canvas.render(move |mouse, image| {
                let width = image.width() as usize;
                for (y, row) in image.chunks_mut(width).enumerate() {
                    for (x, pixel) in row.iter_mut().enumerate() {
                        let buffer = &raytraced_color_buffer;
                        match buffer.get(IMAGE_WIDTH as usize * y + x) {
                            Some(color) => {
                                *pixel = pixel_canvas::Color {
                                    r: (color.x*256.0) as u8,
                                    g: (color.y*256.0) as u8,
                                    b: (color.z*256.0) as u8,
                                }
                            }
                            _ => (),
                        };
                    }
                }
            });
        }
        _ => (),
    }
    

    Ok(())
}