//35
fn main () {
let numbers: [i32; 5]= [1,2,3,4,5];
    println!("all the numbers are:  {:?}", numbers);
let fruits: [&str; 3] = ["apple", "banaa", "orange"];
    println!("fruits array 1st: {}", fruits[0]);
        println!("fruits array 2nd: {}", fruits[1]);
            println!("fruits array 3ed: {}", fruits[2]);

let human: (String, u32, bool) = ("Alice".to_string(), 32, true);
    println!("this is alice: {:?}", human);
let my_mix_tuple = ("Him", 22, true,[1,2,3]);
    println!("this is Him: {:?}", my_mix_tuple);
//(string, u32, bool,)

}



/*
    let x: i32 = -42;
let y: u64 = 100;
    println!("signed integer!: {}", x);
    println!("unsigned integer!: {}", y);
//==================================================
let pi: f64 = 3.14;
    println!("value of pi {}", pi);
//==================================================
let is_snowing: bool = true;
    println!("is it snowing ? {}", is_snowing);
//==================================================
let letter: char = 'a';
print!("this is Letter: {}", letter);

*/


/*
    println!("Hello, Rust from cargo!");
*/

/* 
//im using a 60% keyboard rn so i need to copy therse i dont know how to type them 
//  {    }
*/