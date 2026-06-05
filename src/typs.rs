use colored::Colorize;
use std::collections::HashMap;
use std::ops::Add;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
#[derive(Debug)]
struct Usr {
    name: String,
    age: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
}

static STATIC_VAR: i32 = 00;                // IS UNSAFE: be mutabel
static mut MUTABLE_STATIC_VAR: i32 = 700;   // IS UNSAFE: be mutabel

const CONST_VAR: i32 = 00;


pub fn rust_typs() {

    imutable_variable();
    mutable_variable();
    shadowing_variable();
    static_and_constant();
    unsafe_statement();
    signed_integers();
    unsigned_integers();
    char_bool_types();
    floats();
    smt_pointer_types();
    compound_types();
    utilitarian_data_type();
    slice_data_type();
    unsafe_data_type();
    collection_types();
    control_types();
    custom_type();
}

fn imutable_variable() {
    let var: &str = "!";
    println!(
        "{} The {},  defined by {} -> {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "IMUTABEL VARIABLE".bold().blue(),
        "'let var: T = ...'".bold().truecolor(152, 251, 152),
        var
    );
}

fn mutable_variable() {
    let mut var: &str = "!";
    var = "!!";
    println!(
        "{} A {}, which can be defined by using {} on it's statement -> {} in {:p}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "MUTABEL VARIABLE".bold().blue(),
        "'let mut var: T = ...'".bold().truecolor(152, 251, 152),
        var,
        &var
    );
}

fn shadowing_variable() {
    let var: &str = "!!!";
    println!(
        "{} Allowing {} Redeclare a existing variable {} and releasing the previous scope in memory, {} -> {} in {:p}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "SHADOWNING".bold().blue(),
        "'let var: &str'".bold().truecolor(152, 251, 152),
        "even if it's MUTABLE or IMUTABLE".bold().truecolor(152, 251, 152),
        var,
        &var
    );
}

fn utilitarian_data_type() {
    let unit: () = ();
    println!(
        "{} An {} Unit Type, E.G, {}. A zero-sized type representing the absence of a value (similar to void) -> {:?}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "UTILITARIAN DATA-TYPE".bold().blue(),
        "()".bold().truecolor(152, 251, 152),
        unit
    );
}

fn slice_data_type() {
    let slice: &str = "Hello, Rust!";
    println!(
        "{} {} = String Slice, E.G, {}. An {} reference to a UTF-8 sequence stored somewhere in memory -> \"{}\"",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "SLICE DATA-TYPE".bold().blue(),
        "&str".bold().truecolor(152, 251, 152),
        "IMMUTABLE".bold().truecolor(152, 251, 152),
        slice
    );
}


fn unsafe_data_type() {
    let mut number: i32 = 42;
    let raw_const: *const i32 = &raw const number;
    let raw_mut: *mut i32 = &raw mut number;
    unsafe {
        println!(
            "{} An {} like a Raw Pointers, E.G, {}. C-style pointers, point directly to memory addr const: {:?}, mut: {:?}) -> value: {}",
            "[AVAILABLE]".bold().truecolor(152, 251, 152),
            "UNSAFE DATA-TYPE".bold().blue(),
            "*const T / *mut T".bold().truecolor(152, 251, 152),
            raw_const,
            raw_mut,
            *raw_const
        );
    }
}


fn collection_types() {
    let vector: Vec<i32> = vec![10, 20, 30];
    println!(
        "{} A {} Vector, E.G, {}. A {} allocated on the heap -> {:?}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "COLLECTION DATA-TYPE".bold().blue(),
        "dynamically growable array".bold().truecolor(152, 251, 152),
        "Vec<T>".bold().truecolor(152, 251, 152),
        vector
    );

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("Rust", 2026);
    println!(
        "{} A {} HashMap, E.G, {}. A key-value dictionary for fast lookups -> {:?}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "COLLECTION DATA-TYPE".bold().blue(),
        "HashMap<K, V>".bold().truecolor(152, 251, 152),
        map
    );
}

fn control_types() {
    let option_val: Option<i32> = Some(42);
    println!(
        "{} A {} Option, E.G, {}. Type representing the presence (Some) or absence (None) of a value -> {:?}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "CONTROL DATA-TYPE".bold().truecolor(152, 251, 152),
        "Option<T>".bold().truecolor(152, 251, 152),
        option_val
    );

    let result_val: Result<&str, &str> = Ok("Operação bem-sucedida");
    println!(
        "{} A {} Result, E.G, {}. Type for error handling representing success (Ok) or failure (Err) -> {:?}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "CONTROL DATA-TYPE".bold().blue(),
        "Result<T, E>".bold().truecolor(152, 251, 152),
        result_val
    );
}       

fn custom_type() {
    let usr_struct = Usr { name: String::from("Alex"), age: 30 };
    println!(
        "{} A {} Struct, E.G, {}. A custom structure to group named fields -> {:?}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "CUSTOM DATA-TYPE".bold().blue(),
        "struct".bold().truecolor(152, 251, 152),
        usr_struct
    );

    let enum_status = Status::Active;
    println!(
        "{} A {} Enum, E.G, {}. An algebraic data type representing one of multiple variants -> {:?}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "CUSTOM DATA-TYPE".bold().blue(),
        "enum".bold().truecolor(152, 251, 152),
        enum_status
    );
}

fn compound_types() {
    let tuple: (bool, i32, bool) = (true, 7, true);
    println!(
        "{} A {} Tuple, E.G, {}. A fixed-size collection that can group different types -> ({}, {}, {})",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "COMPOUND DATA-TYPE".bold().blue(),
        "(bool, i32, bool)".bold().truecolor(152, 251, 152),
        tuple.0,
        tuple.1,
        tuple.2
    );

    let arr: [char; 4] = ['a', 'b', 'c', 'd'];
    println!(
        "{} {} Array: {}. A fixed-size collection where all elements must be of the same type, stored on the Stack -> {:?}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "COMPOUND DATA-TYPE".bold().blue(),
        "'let arr: [char; 4] = [...]'".bold().truecolor(152, 251, 152),
        arr,
    );
}


fn char_bool_types() {
    println!(
        "{} {} {}. As a {} boolean type",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "BOOLEAN DATA-TYPE".bold().blue(),
        "bool".bold().truecolor(152, 251, 152),
        "8 bits(1 byte)".bold().truecolor(152, 251, 152)
    );

    println!(
        "{} {} {}. As a {} single Unicode character",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "CHARACTER DATA-TYPE".bold().blue(),
        "char".bold().truecolor(152, 251, 152),
        "32 bits (4 bytes)".bold().truecolor(152, 251, 152)
    );

    let mut string: String = String::from(" ");
    string += "Alright";
    println!(
        "{} {} {}. As a string of Unicode character MUTABLE, stored ob Heap, {} -> {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "CHANGEABLE STRING DATA-TYPE".bold().blue(),
        "String".bold().truecolor(152, 251, 152),
        "'let [mut] str: String = String::from(\"...\")'".bold().truecolor(152, 251, 152),
        string
    );
}


fn smt_pointer_types() {
    let boxed_num = Box::new(100);
    println!(
        "{} A {} Box, E.G, {}. Smart pointer for explicit heap allocation -> Value: {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "SMART-POINTER DATA-TYPE".bold().blue(),
        "Box<T>".bold().truecolor(152, 251, 152),
        boxed_num
    );

    let rc_data = Rc::new(500);
    let rc_clone = Rc::clone(&rc_data);
    println!(
        "{} A {} Rc, E.G, {}. Reference counting pointer for single-threaded multiple ownership -> Owners: {}, Value: {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "SMART-POINTER DATA-TYPE".bold().blue(),
        "Rc<T>".bold().truecolor(152, 251, 152),
        Rc::strong_count(&rc_data),
        rc_clone
    );

    let arc_data = Arc::new(888);
    println!(
        "{} A {} Arc, E.G, {}. Atomically reference counted pointer for thread-safe multiple ownership -> Value: {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "SMART-POINTER DATA-TYPE".bold().blue(),
        "Arc<T>".bold().truecolor(152, 251, 152),
        arc_data.add(88)
    );

    let mutex_data = Mutex::new(10);
    println!(
        "{} A {} Mutex, E.G, {}. Mutual exclusion lock for safe data mutation between threads -> Inside Lock: {:?}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "SMART-POINTER DATA-TYPE".bold().blue(),
        "Mutex<T>".bold().truecolor(152, 251, 152),
        mutex_data.lock().unwrap()
    );
}



fn static_and_constant() {
    println!(
        "{} CONTANTS {}, of {}, declared in UPPER_SNAKE_CASE and {} -> {} in {:p}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "'const'".bold().truecolor(152, 251, 152),
        "global scope".bold().truecolor(152, 251, 152),
        "IMUTABLE".bold().truecolor(152, 251, 152),
        CONST_VAR, 
        &CONST_VAR
    );
    println!(
        "{} STATIC {}, of {}, and availabel to be {}({}) -> {} in {:p}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "'static'".bold().truecolor(152, 251, 152),
        "global scope".bold().truecolor(152, 251, 152),
        "MUTABLE by 'mut'".bold().truecolor(152, 251, 152),
        "how ever susceptible to data-race".bold().red(),
        STATIC_VAR, 
        &STATIC_VAR
    );
}


fn signed_integers() {
    println!(
        "{} A {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "i8 DATA-TYPE".bold().blue(),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "8 bits(1 byte)".bold().truecolor(152, 251, 152),
        i8::MIN,
        i8::MAX
    );
    println!(
        "{} A {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "i16 DATA-TYPE".bold().blue(),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "16 bits(2 byte)".bold().truecolor(152, 251, 152),
        i16::MIN,
        i16::MAX
    );
    println!(
        "{} A {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "i32 DATA-TYPE".bold().blue(),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "32 bits(4 byte)".bold().truecolor(152, 251, 152),
        i32::MIN,
        i32::MAX
    );
    println!(
        "{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "i64 DATA-TYPE".bold().blue(),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "64 bits(8 byte)".bold().truecolor(152, 251, 152),
        i64::MIN,
        i64::MAX
    );
    println!(
        "{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "i128 DATA-TYPE".bold().blue(),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "128 bits(16 byte)".bold().truecolor(152, 251, 152),
        i128::MIN,
        i128::MIN
    );
    println!(
        "{} {}. As a {}, Igual ao i32 em sistemas 32-bits ou i64 em sistemas 64-bits", 
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "isize DATA-TYPE".bold().blue(),
        "SIGNED integer".bold().truecolor(152, 251, 152)
    );
}


fn unsigned_integers() {
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "u8 DATA-TYPE".bold().blue(),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "8 bits(1 byte)".bold().truecolor(152, 251, 152),
        u8::MIN,
        u8::MAX
    );
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "u16 DATA-TYPE".bold().blue(),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "16 bits(2 byte)".bold().truecolor(152, 251, 152),
        u16::MIN,
        u16::MAX
    );
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "u32 DATA-TYPE".bold().blue(),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "32 bits(4 byte)".bold().truecolor(152, 251, 152),
        u32::MIN,
        u32::MAX
    );
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "u64 DATA-TYPE".bold().blue(),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "64 bits(8 byte)".bold().truecolor(152, 251, 152),
        u64::MIN,
        u64::MAX
    );
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "u128 DATA-TYPE".bold().blue(),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "128 bits(16 byte)".bold().truecolor(152, 251, 152),
        u128::MIN,
        u128::MAX
    );
    println!(
        "{} {}. As a {}, Igual ao u32 em sistemas 32-bits ou u64 em sistemas 64-bits", 
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "usize DATA-TYPE".bold().blue(),
        "UNSIGNED integer".bold().truecolor(152, 251, 152)
    );
}

fn floats()  { 
    println!("{} {}. As a {}, {} 32 bits",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "f32 DATA-TYPE".bold().blue(),
        "decimal type with simple precision (IEEE-754)".bold().truecolor(152, 251, 152),
        "inherentemente signed".bold().truecolor(152, 251, 152)
    );


    println!("{} {}. As a {} (better for math precision), {} 64 bits, however also not recomended to monetary operations",
        "[AVAILABLE]".bold().truecolor(152, 251, 152),
        "f64 DATA-TYPE".bold().blue(),
        "decimal type with double precision".bold().truecolor(152, 251, 152),
        "inherentemente signed".bold().truecolor(152, 251, 152)
    );
}

fn unsafe_statement() {
    unsafe {
        MUTABLE_STATIC_VAR = 7;
        println!(
            "{} {} keyword to declare the {} the approach of a {} -> {}",
            "[AVAILABLE]".bold().truecolor(152, 251, 152),
            "'unsafe'".bold().red(),
            "existence of contracts the compiler can't check, like".bold().red(),
            "mutate static var".bold().red(),
            std::ptr::read(&raw const MUTABLE_STATIC_VAR)
        );
    }
}

