#![allow(unused_variables)]
#![allow(unused_imports)]

//use std::{thread, time};
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write; //to flush stdout

use cgmath::prelude::*;
//use cgmath::{vec3, quat, mat4};
//use cgmath::structure::InnerSpace; //nope
//use cgmath::BaseFloat;
//use cgmath::prelude::InnerSpace;
use cgmath::Vector3;

mod testing;

type Point3 = Vector3<f32>;
type Color = Vector3<f32>;
type Vec3 = Vector3<f32>;

pub struct Ray{
    pub origin: Point3,
    pub dir: Vec3,
    //pub test: Vector3<f32>,
}

impl Ray {
    fn at(&self, t: f32) -> Point3 {
        //let stub = self.dir.normalize();
        self.origin + t*self.dir
    }
}


fn testing_cgmath(){
    
    let my_vec = cgmath::Vector3{x:1,y:1,z:1};
    let vec2 = my_vec * 2 + my_vec;
    println!("{:?}", vec2);

    let v = cgmath::Vector3::new(1.0, 2.0, 3.0);

    let pointerini = Point3::new(1.0, 1.0, 1.0);
    let colorini = cgmath::Vector3{x:1.0, y:1.1, z:1.2};

    //let ray_test = Ray { origin: Point3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 1.0, 1.0) };

    //let test = Vec3::new();

}

fn write_color(image_ascii_data: &mut String, pixel_color: Color) {
    let ir = (255.999 * pixel_color.x) as i32;
    let ig = (255.999 * pixel_color.y) as i32;
    let ib = (255.999 * pixel_color.z) as i32;
    image_ascii_data.push_str(&format!("{} {} {}\n",ir,ig,ib));
}

fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn ray_color(r: Ray) -> Color {
    if hit_sphere(&Point3::new(0.0,0.0,-1.0), 0.5, &r) {
        return Color::new(1.0,0.0,0.0);
    }
    let unit_direction: Vec3 = r.dir.normalize();
    let t = 0.5*(unit_direction.y + 1.0); //mappejar l'intèrval [-1,1] a [0,1]
    (1.0-t)*Color::new(1.0,1.0,1.0) + t*Color::new(0.5,0.7,1.0)

    //let test: Vector3<f64> = Vector3::new(1.0,1.0,1.0);
    //let t2 = test.normalize();
    //let t4 = Vector3::new(0.0, 0.0, 0.0).normalize();
}

fn main() -> std::io::Result<()> {

    testing::testing();
    //testing_cgmath();

    //learningSample();

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image_width = 384; 
    let image_height: i32 = (image_width as f32 / ASPECT_RATIO) as i32;

    let mut file = File::create("image.ppm")?;
    let mut image_ascii_data: String = "".to_owned();
    //let borrowed: &str = "hola";
    //let b2: &str = &image_width.to_string();
    let header = format!("P3\n{} {}\n255\n",image_width, image_height);
    image_ascii_data.push_str(&header);
    //imageASCIIData +=  image_width.to_string();

    let viewport_height: f32 = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horitzontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horitzontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,focal_length);
    
    //let type_identifier: f64 = 1.2;

    //for j in image_height-1..=0 {
    for j in (0..image_height).rev() {
        //println!("{}", j);
        //println!("\rScanlines remaining: {} ", j); //commented out while the \r thing doesn't work
        std::io::stdout().flush().expect("error flushing stdout");
        for i in 0..image_width {
            /*let r = i as f64 / (image_width-1) as f64;
            let g = j as f64 / (image_width-1) as f64;
            let b = 0.25;
            
            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;
            
            let pixel_color = Vec3::new(i as f32 / ((image_width-1) as f32), j as f32 / ((image_height-1) as f32), 0.25);
            */

            let u = i as f32 / (image_width-1) as f32;
            let v = j as f32 / (image_height-1) as f32;
            let r = Ray{ origin: origin, dir: lower_left_corner + u*horitzontal + v*vertical - origin };

            let pixel_color = ray_color(r);
            write_color(&mut image_ascii_data, pixel_color);
        }
    }

    //let wtfType = b"Hello, world!";
    file.write_all(image_ascii_data.as_bytes())?;
    Ok(())
}
