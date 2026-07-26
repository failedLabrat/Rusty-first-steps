//35
//test
fn main () {
let numbers: [i32; 5]= [1,2,3,4,5];
    println!("all the numbers are:  {:?}", numbers);
let fruits: [&str; 3] = ["apple", "banaa", "orange"];
    println!("fruits array 1st: {}", fruits[0]);
        println!("fruits array 2nd: {}", fruits[1]);
            println!("fruits array 3ed: {}", fruits[2]);

let human: (String, u32, bool) = ("Alice".to_string(), 32, true);
    println!("this is alice: {:?}", human);
let tuple: (&str, u32, bool) = ("Him", 22, true);
        println!("Name: {}", tuple.0);
            println!("Age: {}", tuple.1);
                println!("Is active: {}", tuple.2);
                
let (name, age, is_active) = tuple;
println!("{} is {} years old, and {}", name, age,is_active);

//claude excercise 
let student1: (&str, u32, char) = ("alex", 19, 'B');
let student2: (&str, u32, char) = ("Leon", 20, 'C');
let student3: (&str, u32, char) = ("Hassan", 21, 'A');

let (student1_name, student1_age, student1_grade) = student1;
let (student2_name, student2_age, student2_grade) = student2;
let (student3_name, student3_age, student3_grade) = student3;


    println!("the 3 Test Rauslts are as following: \n {} scored at {} years old, scored a solid {}.\n for the Next Test Resault we got {}, age {} scoring a {}.\n for the last score its {} at {} years old scoring an impressive {}", 
    student1_name,student1_age,student1_grade,student2_name,student2_age,student2_grade,student3_name,student3_age,student3_grade)

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