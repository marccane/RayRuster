#![allow(unused_variables)]
#![allow(unused_imports)]

//use cgmath::prelude::*;
//use std::{thread, time};
use std::io;

use std::fs::File;
use std::io::prelude::*;

use rand::Rng;
use std::cmp::Ordering;

fn testing(){
        // slicing a Vec
        let vec = vec![1, 2, 3];
        let int_slice = &vec[..];
        // coercing an array to a slice
        let str_slice: &[&str] = &["one", "two", "three"];
    
        /*Slices are either mutable or shared. The shared slice type is &[T], while the mutable slice type is &mut [T],
         where T represents the element type. For example, you can mutate the block of memory that a mutable slice points to:*/
    
        let x = &mut [1, 2, 3];
        let mut _y = &[2, 3, 4];
        
        x[2] = 5; //yep
        //y[2] = 1; //nope
        _y = &[3,4,5];//[x[2],x[1],x[0]];
    
        let isequal = x == &[1, 2, 5];
        println!("result {}", isequal);
    
        x[1] = 7;
        assert_eq!(x, &[1, 7, 5]);
    
        for i in _y{
            println!("{} ", i);    
        }
    
        let numbers = &[0, 1, 2];
        for n in numbers {
            println!("{} is a number!", n);
        }
    
        let mut scores = [7, 8, 9];
        for score in &mut scores[..] {
            *score += 1;
        }
        
        //aixo té tipus &[i32]
        let slice = &numbers[0..2]; //sense & dona error diguent que ha de saber el tamany en compile time...
    
        println!("magia: {:?}", (&vec, int_slice, str_slice, x, _y, numbers, scores, slice));
    
        //let multypearrtest = [1,'a',"hola"]; //nope 
        //amb {} tampoc va
        let _multypearrtest = (1,'a',"hola");
        //let multypearrtest2 = [2.0,2]; //lmao no li coles res

        let explicitTypedArr: [i32; 4] = [7,7,7,7];
        let mut arrCpy = explicitTypedArr;
        arrCpy = [1,2,3,4];
        let numbar = arrCpy[3];

        let numbar = 34; //shadowing

        const _lolconst: u32 = 25; //s'ha despecificar el tipus
        const _operacioConst: u32 = _lolconst + 10;
        const _resultat: i32 = constFn(); //nomes funcions const (constexpr???)
        
        let flotadorStrang = 123_45_6.7_8_9;
        println!("flotadorStrang: {:?}", flotadorStrang);

        let tup = (123, 4_5_6, 56.78, "zup mate", '%', [1.1,2.2,3.3]);
        let typedTup: (i32, u16, &str, [u32;2]) = (-4,5,"hola", [3,3]);
        let (a,b,c,_) = typedTup; //yeaboi
        println!("typedTup: {:?}", typedTup);

        let firstEl = tup.0;
        let secondEl = tup.1;

        let superArr: [i32; 5] = [7; 5]; // inicialitzar array de longitud 5 amb 7's
        println!("superArr: {:?}", superArr);

        let exprResult = {
            let mut x = 2.3;
            for i in 1..=20 { //20 iteracions, el simbol '=' determina que és inclusiu
                x *= 1.4;
            }
            x //no ;
        };
        println!("exprRes: {}", exprResult);

        let a = 123;
        let b = a as i16;

        let i = 10;
        println!("factorial({}) = {}", i, fact(i));

        for i in (2..9).rev(){ // 8->2
            print!("{} ", i);
        }
        println!();

        let mut vec_test = vec![1, 2, 3, 4];
        vec_test.push(5);

        for x in vec_test.iter_mut() {
            *x *= 2;
        }

        println!("{:?}",vec_test);

        let mut test: Vec<i32>;
        let mut tuple_array = vec![]; //Vec::<(i32, i32)>;
        for (idx, elem) in vec_test.iter().enumerate() {
            tuple_array.push((elem, idx, idx+1));
        }
        println!("{:?}", tuple_array);

}

fn fact(i: i32) -> i32 {
    if i == 0 { 1 } else { i * fact(i-1) }
}

const fn constFn() -> i32{ //oh shit no has de fer forward declarations de les funcions, infact no has de fer forward declarations de res, que curios eh?
    77
}

fn NevadaTestSite(){
    unsafe{

    }
}

//use cgmath::{vec3, quat, mat4};

use cgmath::Vector3;

fn testingCgmath(){
    
    //type point3 = cgmath::vector::Vector3;
    type point3 = Vector3;
    //type color = Vector3;

    let myVec = cgmath::Vector3{x:1,y:1,z:1};
    let vec2 = myVec * 2 + myVec;
    println!("{:?}", vec2);

    let v = cgmath::Vector3::new(1.0, 2.0, 3.0);
}

fn learningSample(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}

use std::io::Write; //to flush stdout

fn main() -> std::io::Result<()> {

    //testing();
    testingCgmath();

    //learningSample();

    /*let image_width: i32 = 256; 
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
        println!("\rScanlines remaining: {} ", j);
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
    file.write_all(image_ascii_data.as_bytes())?;*/
    Ok(())
}

trait Knero{
    fn lmoile(&self);
}

struct Knaci();

impl Knero for Knaci {
    fn lmoile(&self) {
        
    }
}

pub struct Arbre {
    num_fulles: i32, //coma opcional
}
