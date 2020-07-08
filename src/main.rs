#![allow(unused_variables)]
#![allow(unused_imports)]

//use cgmath::prelude::*;
//use std::{thread, time};
use std::io;

use std::fs::File;
use std::io::prelude::*;

use std::io::Write; //to flush stdout

//use cgmath::{vec3, quat, mat4};

mod testing;

use cgmath::Vector3;
type Point3 = Vector3<f32>;
type Color = Vector3<f32>;
type Vec3 = Vector3<f32>;

pub struct Ray{
    pub origin: Point3,
    pub dir: Vec3,
    pub test: Vector3<f32>,
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

fn write_color(image_ascii_data: String, pixel_color: Color) {    
}

fn ray_color(r: Ray) -> Color {
    //let unit_direction: Vec3 = r.dir.normalize();
    //let test: Vector3<f64> = Vector3::new(1.0,1.0,1.0);
    //let test: Vector3<f64> = Vector3::new(1.0,1.0,1.0);
    //let t2 = test.normalize();
    //let boxed = Box::new(Vector3::new(1.0,1.0,1.0)); //: std::boxed::Box<cgmath::vector::Vector3<f64>>
    //let t3 = boxed.normalize();

    //let t4 = Vector3::new(0.0, 0.0, 0.0).normalize();

    Color::new(1.0,1.0,1.0)
}

fn main() -> std::io::Result<()> {

    testing::testing();
    //testing_cgmath();

    //learningSample();

    let image_width: i32 = 256; 
    let image_height = 256;

    let mut file = File::create("image.ppm")?;
    let mut image_ascii_data: String = "".to_owned();
    //let borrowed: &str = "hola";
    //let b2: &str = &image_width.to_string();
    let header = format!("P3\n{} {}\n255\n",image_width, image_height);
    image_ascii_data.push_str(&header);
    //imageASCIIData +=  image_width.to_string();

    let type_identifier: f64 = 1.2;

    //for j in image_height-1..=0 {
    for j in (0..image_height).rev() {
        //println!("{}", j);
        //println!("\rScanlines remaining: {} ", j); //commented out while the \r thing doesn't work
        std::io::stdout().flush().expect("error flushing stdout");
        for i in 0..image_width{
            let r = i as f64 / (image_width-1) as f64;
            let g = j as f64 / (image_width-1) as f64;
            let b = 0.25;
            
            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            image_ascii_data.push_str(&format!("{} {} {}\n",ir,ig,ib));
        }
    }

    //let wtfType = b"Hello, world!";
    file.write_all(image_ascii_data.as_bytes())?;
    Ok(())
}
