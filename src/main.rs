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

fn write_color(image_ascii_data: &mut String, pixel_color: &Color2, samples_per_pixel: i32) {

    let scale = 1.0 / samples_per_pixel as f32;

    //Get the average colour of all the samples, gamma correct (p^(1/2)) and convert color to byte 
    let ir = (256.0 * clamp((pixel_color.x * scale).sqrt(), 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp((pixel_color.y * scale).sqrt(), 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp((pixel_color.z * scale).sqrt(), 0.0, 0.999)) as u8;

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

use rayon::prelude::*;

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
    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_DEPTH: i32 = 10;
    
    //world
    let mat_ground = Lambertian{albedo: Color2::new(0.8, 0.8, 0.0)};
    let mat_center = Lambertian{albedo: Color2::new(0.7, 0.3, 0.3)};
    let mat_left = Metal::new(Color2::new(0.8, 0.8, 0.8));
    let mat_right = Metal::new(Color2::new(0.8, 0.6, 0.2));
    
    let mut world = IntersectableList{objects: Vec::<Box<dyn Intersectable>>::new()};
    world.add(Box::new(Sphere{center: Point32::new(0.0,-100.5,-1.0), radius: 100.0, material: &mat_ground}));
    world.add(Box::new(Sphere{center: Point32::new(0.0,0.0,-1.0), radius: 0.5, material: &mat_center}));
    world.add(Box::new(Sphere{center: Point32::new(-1.0,0.0,-1.0), radius: 0.5, material: &mat_left}));
    world.add(Box::new(Sphere{center: Point32::new(1.0,0.0,-1.0), radius: 0.5, material: &mat_right}));

    //camera
    let camera = Camera::new();

    //misc
    let mut rng = rand::thread_rng();
    const NUM_PIXELS: usize = (IMAGE_WIDTH * IMAGE_HEIGHT) as usize;
    let mut pixel_col_vec = vec![(0, None); NUM_PIXELS]; //i32, Option::<Color2>
    
    for i in 0..NUM_PIXELS {
        pixel_col_vec[i] = (i, None);
    }
    pixel_col_vec[0] = (0, Some(Color2::new(0.0,0.0,0.0)));
    
    pixel_col_vec.par_iter_mut().for_each(|t| {
        let (pixelidx, _) = *t;
        let i = pixelidx as i32 % IMAGE_WIDTH;
        let j = pixelidx as i32/ IMAGE_WIDTH;
        let mut rng = rand::thread_rng();
        
        let mut pixel_color = Color2::new(0.0,0.0,0.0);
        for s in 0..SAMPLES_PER_PIXEL {
            let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
            let r = camera.get_ray(u,v);
            pixel_color += ray_color(r, &world, MAX_DEPTH);
        }
        
        *t = (0, Some(pixel_color));
        
        //let mut rng = thread_rng();
        //*p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect()
        
    });

    /*for j in (0..IMAGE_HEIGHT).rev() {
        //println!("{}", j);
        //println!("\rScanlines remaining: {} ", j); //commented out while the \r thing doesn't work
        std::io::stdout().flush().expect("error flushing stdout");
        for i in 0..IMAGE_WIDTH {

            let mut pixel_color = Color2::new(0.0,0.0,0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
                let r = camera.get_ray(u,v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }

            write_color(&mut image_ascii_data, &pixel_color, SAMPLES_PER_PIXEL);
            raytraced_color_buffer.push(pixel_color);
        }
    }*/
    
    for tpl in pixel_col_vec {
        let (_, opt_pix_col) = tpl;
        write_color(&mut image_ascii_data, &opt_pix_col.unwrap(), SAMPLES_PER_PIXEL);
    }

    file.write_all(image_ascii_data.as_bytes())?;
    
    match settings.display_mode { 
        settings::DisplayMode::WINDOW => {
            let canvas = Canvas::new(IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize)
            .title("RayRuster 0.7");

            canvas.render(move |mouse, image| {
                let width = image.width() as usize;
                let scale = 1.0 / SAMPLES_PER_PIXEL as f32;
                for (y, row) in image.chunks_mut(width).enumerate() {
                    for (x, pixel) in row.iter_mut().enumerate() {
                        let buffer = &raytraced_color_buffer;
                        match buffer.get(IMAGE_WIDTH as usize * (IMAGE_HEIGHT as usize - y) + x) {
                            Some(color) => {
                                *pixel = pixel_canvas::Color {
                                    r: (255.0 * color.x * scale) as u8,
                                    g: (255.0 * color.y * scale) as u8,
                                    b: (255.0 * color.z * scale) as u8,
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
