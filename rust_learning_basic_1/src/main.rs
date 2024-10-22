mod basic {
    pub mod datatype;
    pub mod variable;
    pub mod condition;
    pub mod loops;
}

fn main() {
    println!("\n Rust শেখার জন্য প্রস্তুত? \n");

    println!("\n Rust basic data types:");
    basic::datatype::run();

    println!("\n Rust basic variables:");
    basic::variable::run();
    
    println!("\n Rust basic condition:");
    basic::condition::run();

    println!("\nRust basic loops:"); basic::loops::run();

}
