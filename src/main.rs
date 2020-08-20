#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io;
use std::fs::File;
use std::io::Write; //to flush stdout
//use std::io::prelude::*;

use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

//use cgmath::prelude::*;
use cgmath::prelude::InnerSpace;

use pixel_canvas::{Canvas, Color};

use rayruster::raytracing::{Ray2, Vec3, Color2, Point32, Intersectable, IntersectableList, Camera};
use rayruster::figures::*;
use rayruster::{materials::*, settings, utils::*};

use rayon::prelude::*;

fn write_color(image_ascii_data: &mut String, pixel_color: &Color2) {

    //Convert color to byte 
    let ir = (256.0 * pixel_color.x) as u8;
    let ig = (256.0 * pixel_color.y) as u8;
    let ib = (256.0 * pixel_color.z) as u8;

    image_ascii_data.push_str(&format!("{} {} {}\n", ir, ig, ib));
}

fn ray_color(r: Ray2, world: &dyn Intersectable, depth: i32) -> Color2 {
    
    if depth < 0 { return Color2::new(0.0,0.0,0.0); }

    let opt_hitrec = world.intersect(&r, 0.001, INFINITY);

    match opt_hitrec {
        Some(hit_rec) => {                    
            /*let target = hit_rec.p + hit_rec.normal + random_unit_vector();
            0.5 * ray_color(Ray2{origin: hit_rec.p, dir: target - hit_rec.p}, world, depth-1)*/
            let scatter_res = hit_rec.material.scatter(r, hit_rec);
            match scatter_res {
                Some((attenuation, scattered)) => {
                    let aten = attenuation;
                    let bounce = ray_color(scattered, world, depth-1);
                    Color2::new(aten.x*bounce.x, aten.y*bounce.y, aten.z*bounce.z)
                },
                None => Color2::new(0.0,0.0,0.0)
            } 
        },
        None => {
            let unit_direction: Vec3 = r.dir.normalize(); //convertir a vector unitari
            let t = 0.5 * (unit_direction.y + 1.0); //mappejar l'intèrval [-1,1] a [0,1]
            (1.0 - t) * Color2::new(1.0, 1.0, 1.0) + t * Color2::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() -> std::io::Result<()> {

    // Settings retreive from run parameters.
    let settings = get_settings_from_run_parameters();

    //image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_DEPTH: i32 = 10;
    
    //world
    let mat_ground = Lambertian{albedo: Color2::new(0.8, 0.8, 0.0)};
    let mat_right = Metal::new(Color2::new(0.8, 0.6, 0.2));
    let mat_left = Dielectric::new(5.0);
    let mat_center = Dielectric::new(1.5);
    
    let mut world = IntersectableList{objects: Vec::<Box<dyn Intersectable>>::new()};
    world.add(Box::new(Sphere{center: Point32::new(0.0,-100.5,-1.0), radius: 100.0, material: &mat_ground}));
    world.add(Box::new(Sphere{center: Point32::new(0.0,0.0,-1.0), radius: 0.5, material: &mat_center}));
    world.add(Box::new(Sphere{center: Point32::new(-1.0,0.0,-1.0), radius: 0.5, material: &mat_left}));
    world.add(Box::new(Sphere{center: Point32::new(1.0,0.0,-1.0), radius: 0.5, material: &mat_right}));

    //camera
    let camera = Camera::new();

    //misc
    let mut pixel_col_vec = vec![(0, None); NUM_PIXELS];
    const NUM_PIXELS: usize = (IMAGE_WIDTH * IMAGE_HEIGHT) as usize;
    
    for i in 0..NUM_PIXELS {
        pixel_col_vec[i] = (i, None);
    }
    
    pixel_col_vec.par_iter_mut().for_each(|t| {
        let (pixelidx, _) = *t;
        let i = pixelidx % IMAGE_WIDTH;
        let j = pixelidx / IMAGE_WIDTH;
        let mut rng = rand::thread_rng();
        let scale = 1.0 / SAMPLES_PER_PIXEL as f32;
        
        let mut acc_pixel_color = Color2::new(0.0,0.0,0.0);
        for s in 0..SAMPLES_PER_PIXEL {
            let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
            let r = camera.get_ray(u,v);
            acc_pixel_color += ray_color(r, &world, MAX_DEPTH);
        }

        //Get the average colour of all the samples and gamma correct (p^(1/2))
        let pixel_color = Color2::new(
            clamp((acc_pixel_color.x * scale).sqrt(), 0.0, 0.999),
            clamp((acc_pixel_color.y * scale).sqrt(), 0.0, 0.999),
            clamp((acc_pixel_color.z * scale).sqrt(), 0.0, 0.999),
        );
        
        *t = (0, Some(pixel_color));
    });
    
    match settings.display_mode { 
        settings::DisplayMode::WINDOW => {
            let canvas = Canvas::new(IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize)
            .title("RayRuster 0.7");

            canvas.render(move |mouse, image| {
                let width = image.width() as usize;
                let scale = 1.0 / SAMPLES_PER_PIXEL as f32;
                for (y, row) in image.chunks_mut(width).enumerate() {
                    for (x, pixel) in row.iter_mut().enumerate() {
                        let (_, opt_pix_col) = pixel_col_vec[IMAGE_WIDTH as usize * y + x];
                        let color = opt_pix_col.unwrap();

                        *pixel = pixel_canvas::Color {
                            r: (256.0 * color.x) as u8,
                            g: (256.0 * color.y) as u8,
                            b: (256.0 * color.z) as u8,
                        }
                    }
                }
            });
        }
        settings::DisplayMode::FILE => {
            let mut file = File::create("image.ppm")?;
            let mut image_ascii_data: String = "".to_owned();
            let header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
            image_ascii_data.push_str(&header);

            for i in (0..IMAGE_HEIGHT).rev() {
                for j in 0..IMAGE_WIDTH {
                    let (_, opt_pix_col) = pixel_col_vec[i*IMAGE_WIDTH + j];
                    write_color(&mut image_ascii_data, &opt_pix_col.unwrap());
                }
            }

            file.write_all(image_ascii_data.as_bytes())?;
        },
    }

    Ok(())
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
