const CONST_VAR: i32 = 00;                      // NOT ALLOWED: be mutabel
static STATIC_VAR: i32 = 00;                    // IS UNSAFE: be mutabel
static mut MUTABLE_STATIC_VAR: i32 = 700;       // IS UNSAFE: be mutabel

fn main() {
    let mut var:&str = "!";
    println!("Hello, world!{}", var);
    var = "!!";
    println!("Hello, world!{}", var);

    print_static_and_constant();
    unsafe {
        MUTABLE_STATIC_VAR = 00;
        println!("NEW OUTPUT: i32 -> {}", MUTABLE_STATIC_VAR + 7);
    }
}

fn print_static_and_constant() {
    println!("CONTANT VAR: i32 -> {}", CONST_VAR);
    println!("STATIC VAR: i32 -> {}", STATIC_VAR);
}
