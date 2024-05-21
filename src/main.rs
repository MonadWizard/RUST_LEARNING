// import module
// mod fibonacci{
//     pub mod fibonacci;
// }

// import path
// use crate::fibonacci::fibonacci as fibonacci;

// use crate::fibonacci::fibonacci;


// mod fibonacci;

use crate::mod_example::call_fibonacci::call_fibonacci;




pub mod fibonacci;
pub mod mod_example;

fn main() {
    println!("Hello, world!");

    let n = 20; 

        //  call fibonacci module 
    // println!("fib({n}) = {}", fibonacci::fibonacci::fib(n));
    
    // println!("fib({n}) = {}",  fibonacci::fib(n));


    println!("fib({n}) = {}",  call_fibonacci(n));
    
    
}



