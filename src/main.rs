#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io;
use std::fs::File;
//use std::io::prelude::*;
use std::io::Write; //to flush stdout

//use cgmath::prelude::*;
use cgmath::prelude::InnerSpace;
use cgmath::Vector3;

use rayruster::raytracing::{Ray2, Vec3, Color2, Point32, Intersectable};
use rayruster::figures::*;
use rayruster::settings;

fn write_color(image_ascii_data: &mut String, pixel_color: Color2) {
    let ir = (255.999 * pixel_color.x) as i32;
    let ig = (255.999 * pixel_color.y) as i32;
    let ib = (255.999 * pixel_color.z) as i32;
    image_ascii_data.push_str(&format!("{} {} {}\n", ir, ig, ib));
}

fn ray_color(r: Ray2) -> Color2 {
    let sphere = Sphere::new(Point32::new(0.0, 0.0, -1.0), 0.5);
    let sphere_intersect = sphere.intersect(&r, 0.0, 100.0);

    match sphere_intersect {
        Some(hit_rec) => {        
            let n = (r.at(hit_rec.t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
            0.5 * Color2::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
        },
        None => {
            let unit_direction: Vec3 = r.dir.normalize();
            let t = 0.5 * (unit_direction.y + 1.0); //mappejar l'intèrval [-1,1] a [0,1]
            (1.0 - t) * Color2::new(1.0, 1.0, 1.0) + t * Color2::new(0.5, 0.7, 1.0)
        }
    }
}

fn process_cli_parameters() -> i8 {
    match std::env::args().nth(1).expect("no raytracing depth given").parse::<i8>() {
        Ok(depth) => if depth < 1 { 1 } else { depth },
        Err(e) => 1,
    }
}

fn main() -> std::io::Result<()> {

    let mut settings = settings::Settings::new();
    let ray_depth = process_cli_parameters();
    settings.ray_depth = ray_depth;

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image_width = 384;
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;

    let mut file = File::create("image.ppm")?;
    let mut image_ascii_data: String = "".to_owned();

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    image_ascii_data.push_str(&header);

    let viewport_height: f32 = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point32::new(0.0, 0.0, 0.0);
    let horitzontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horitzontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //for j in image_height-1..=0 {
    for j in (0..image_height).rev() {
        //println!("{}", j);
        //println!("\rScanlines remaining: {} ", j); //commented out while the \r thing doesn't work
        std::io::stdout().flush().expect("error flushing stdout");
        for i in 0..image_width {

            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = Ray2 {
                origin: origin,
                dir: lower_left_corner + u * horitzontal + v * vertical - origin,
            };

            let pixel_color = ray_color(r);
            write_color(&mut image_ascii_data, pixel_color);
        }
    }

    file.write_all(image_ascii_data.as_bytes())?;

    //henryTesting();

    Ok(())
}
/*
use figures::Sphere;

fn henryTesting() {
    let mut sphere = Sphere::new();
}*/
