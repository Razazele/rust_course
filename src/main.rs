#![allow(unused)]


use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
//Input - Output , Uso de variables mutables
/*     println!("Cual es tu nombre?");
    let mut name = String::new();
    let greeting="Encantado de Conocerte";
    io::stdin().read_line(&mut name)
    .expect("No recibi una entrada");
    println!("Hola {}! {}",name.trim_end(),greeting); */


    //SHADOWING (REDEFINING VARIABLE TYPES)
    /* const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("La edad no fue asignada un numero");
    age = age + 1;
    println!("Soy {} y quiero ${}",age,ONE_MIL); */


    //Randoms
/*     let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}",random_num) */






}
