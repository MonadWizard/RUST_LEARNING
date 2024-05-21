// pub fn fib(n: u32) -> u32 {
//     if n < 2 {
//         // The base case.
//         todo!("Implement this")
//     } else {
//         // The recursive case.
//         todo!("Implement this")
//     }
// }

pub fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}