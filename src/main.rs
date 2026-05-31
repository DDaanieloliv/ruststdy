const CONST_VAR: i32 = 00;                      
static STATIC_VAR: i32 = 00;                    // IS UNSAFE: be mutabel
static mut MUTABLE_STATIC_VAR: i32 = 700;       // IS UNSAFE: be mutabel

fn main() {
    let mut var: &str = "!";
    println!("[IMUTABEL VARIABLE]  A 'let var: &str' variable, wich is IMUTABLE, of type &str (string_slice: reference to a seq_char in stack memory) -> {} in {:p}", var, &var);
    var = "!!";
    println!("[MUTABLE VARIABLE]  Also can be MUTABEL by using 'let mut var: &str ...' on it's statement -> {} in {:p}", var, &var);
    let var: &str = "!!!";
    println!("[SHADOWNING]  Being able to redeclare that variable 'let var: &str' and releasing the previous scope in memory, even if it's MUTABLE or IMUTABLE -> {} in {:p}", var, &var);

    print_static_and_constant();
    unsafe {
        MUTABLE_STATIC_VAR = 7;                // SUSCEPTIBLE: data-race
        println!("[AVAILABLE]  The 'unsafe' keyword to declare the existence of contracts the compiler can't check, like the approach of mutate a static var -> {}", std::ptr::read(&raw const MUTABLE_STATIC_VAR));
    }
}

fn print_static_and_constant() {
    println!("[AVAILABLE]  CONTANTS 'const', of global scope, declared in UPPER_SNAKE_CASE and IMUTABLE -> {}", CONST_VAR);
    println!("[AVAILABLE]  STATIC 'static', of global scope, declared in UPPER_SNAKE_CASE and availabel to be MUTABLE by 'mut' (how ever susceptible to data-race) -> {}", STATIC_VAR);
}
