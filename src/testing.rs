use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn testing() {
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
    _y = &[3, 4, 5]; //[x[2],x[1],x[0]];

    let isequal = x == &[1, 2, 5];
    println!("result {}", isequal);

    x[1] = 7;
    assert_eq!(x, &[1, 7, 5]);

    for i in _y {
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

    println!(
        "magia: {:?}",
        (&vec, int_slice, str_slice, x, _y, numbers, scores, slice)
    );

    //let multypearrtest = [1,'a',"hola"]; //nope
    //amb {} tampoc va
    let _multypearrtest = (1, 'a', "hola");
    //let multypearrtest2 = [2.0,2]; //lmao no li coles res

    let explicit_typed_arr: [i32; 4] = [7, 7, 7, 7];
    let mut arr_cpy = explicit_typed_arr;
    arr_cpy = [1, 2, 3, 4];
    let numbar = arr_cpy[3];

    let numbar = 34; //shadowing

    const _LOLCONST: u32 = const_fn() as u32; //s'ha despecificar el tipus
    const _OPERACIO_CONST: u32 = _LOLCONST + 10;
    const _RESULTAT: i32 = const_fn(); //nomes funcions const (constexpr???)

    let flotador_strang = 123_45_6.7_8_9;
    println!("flotadorStrang: {:?}", flotador_strang);

    let tup = (123, 4_5_6, 56.78, "zup mate", '%', [1.1, 2.2, 3.3]);
    let typed_tup: (i32, u16, &str, [u32; 2]) = (-4, 5, "hola", [3, 3]);
    let (a, b, c, _) = typed_tup; //yeaboi
    println!("typedTup: {:?}", typed_tup);

    let first_el = tup.0;
    let second_el = tup.1;

    let super_arr: [i32; 5] = [7; 5]; // inicialitzar array de longitud 5 amb 7's
    println!("superArr: {:?}", super_arr);

    let expr_result = {
        let mut x = 2.3;
        for i in 1..=20 {
            //20 iteracions, el simbol '=' determina que és inclusiu
            x *= 1.4;
        }
        x //no ;
    };
    println!("exprRes: {}", expr_result);

    let a = 123;
    let b = a as i16;

    let i = 10;
    println!("factorial({}) = {}", i, fact(i));

    for i in (2..9).rev() {
        // 8->2
        print!("{} ", i);
    }
    println!();

    let mut vec_test = vec![1, 2, 3, 4];
    vec_test.push(5);

    for x in vec_test.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", vec_test);

    let mut test: Vec<i32>;
    let mut tuple_array = vec![]; //Vec::<(i32, i32)>;
    for (idx, elem) in vec_test.iter().enumerate() {
        tuple_array.push((elem, idx, idx + 1));
    }
    println!("{:?}", tuple_array);

    let x = 12;
    println!("fact({}) :{}", x, fact(x));

    //let wtfType = b"Hello, world!";

    nevada_test_site();

    if false {
        learning_sample();
        testing_cgmath();
    }

    let stfu_compiler = Arbre{ num_fulles: 123, };
}

fn fact(i: i32) -> i32 {
    if i == 0 {
        1
    } else {
        i * fact(i - 1)
    }
}

const fn const_fn() -> i32 {
    //oh shit no has de fer forward declarations de les funcions, infact no has de fer forward declarations de res, que curios eh?
    77
}

fn nevada_test_site() {
    unsafe {}
}

fn learning_sample() {
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

use cgmath::{Point3};

fn testing_cgmath() {
    let my_vec = cgmath::Vector3 { x: 1, y: 1, z: 1 };
    let vec2 = my_vec * 2 + my_vec;
    println!("{:?}", vec2);

    let v = cgmath::Vector3::new(1.0, 2.0, 3.0);

    let pointerini = Point3::new(1.0, 1.0, 1.0);
    let colorini = cgmath::Vector3 {
        x: 1.0,
        y: 1.1,
        z: 1.2,
    };

    //let ray_test = Ray { origin: Point3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 1.0, 1.0) };

    //let test = Vec3::new();
}

trait Knero {
    fn lmoile(&self);
}

struct Knaci();

impl Knero for Knaci {
    fn lmoile(&self) {}
}

pub struct Arbre {
    num_fulles: i32, //coma opcional
}
