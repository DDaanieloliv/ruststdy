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


pub fn type_overview() {
    let mut var: &str = "!";
    println!(
        "{} The variable {} , wich is {}. And it's type, {} -> {} in {:p}",
        "[IMUTABEL VARIABLE]".bold().truecolor(152, 251, 152),
        "'let var: &str = ...'".bold().truecolor(152, 251, 152),
        "IMUTABLE".bold().truecolor(152, 251, 152),
        "&str (string_slice: reference to a seq_char in stack memory)".bold().truecolor(152, 251, 152),
        var,
        &var
    );

    var = "!!";
    println!(
        "{} A variable also can be {} by using {} on it's statement -> {} in {:p}",
        "[MUTABLE VARIABLE]".bold().truecolor(152, 251, 152),
        "MUTABEL".bold().truecolor(152, 251, 152),
        "'let mut var: &str = ...'".bold().truecolor(152, 251, 152),
        var,
        &var
    );
    let var: &str = "!!!";
    println!(
        "{} Redeclare a existing variable {} and releasing the previous scope in memory, {} -> {} in {:p}",
        "[SHADOWNING]".bold().truecolor(152, 251, 152),
        "'let var: &str'".bold().truecolor(152, 251, 152),
        "even if it's MUTABLE or IMUTABLE".bold().truecolor(152, 251, 152),
        var,
        &var
    );

    print_static_and_constant();
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

    signed_integers();

    unsigned_integers();

    floats();

    println!(
        "\n{} {}. As a {} boolean type",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "bool".bold().truecolor(152, 251, 152),
        "8 bits(1 byte)".bold().truecolor(152, 251, 152)
    );

    println!(
        "{} {}. As a {} single Unicode character",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "char".bold().truecolor(152, 251, 152),
        "32 bits (4 bytes)".bold().truecolor(152, 251, 152)
    );

    let mut string: String = String::from(" ");
    string += "Alright";
    println!(
        "{} {}. As a string of Unicode character MUTABLE, stored ob Heap, {} -> {}",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "String".bold().truecolor(152, 251, 152),
        "'let [mut] str: String = String::from(\"...\")'".bold().truecolor(152, 251, 152),
        string
    );

    let tuple: (bool, i32, bool) = (true, 7, true);
    println!(
        "\n{} Tuple, E.G, {}. A fixed-size collection that can group different types -> ({}, {}, {})",
        "[COMPOUND DATA-TYPE]".bold().truecolor(152, 251, 152),
        "(bool, i32, bool)".bold().truecolor(152, 251, 152),
        tuple.0,
        tuple.1,
        tuple.2
    );

    let arr: [char; 4] = ['a', 'b', 'c', 'd'];
    println!(
        "{} Array: {}. A fixed-size collection where all elements must be of the same type, stored on the Stack -> {:?}",
        "[COMPOUND DATA-TYPE]".bold().truecolor(152, 251, 152),
        "'let arr: [char; 4] = [...]'".bold().truecolor(152, 251, 152),
        arr,
    );

    let unit: () = ();
    println!(
        "\n{} Unit Type, E.G, {}. A zero-sized type representing the absence of a value (similar to void) -> {:?}",
        "[UTILITARIAN DATA-TYPE]".bold().truecolor(152, 251, 152),
        "()".bold().truecolor(152, 251, 152),
        unit
    );

    let slice: &str = "Hello, Rust!";
    println!(
        "{} String Slice, E.G, {}. An {} reference to a UTF-8 sequence stored somewhere in memory -> \"{}\"",
        "[SLICE DATA-TYPE]".bold().truecolor(152, 251, 152),
        "&str".bold().truecolor(152, 251, 152),
        "IMMUTABLE".bold().truecolor(152, 251, 152),
        slice
    );

    let mut number: i32 = 42;
    let raw_const: *const i32 = &raw const number;
    let raw_mut: *mut i32 = &raw mut number;
    unsafe {
        println!(
            "{} Raw Pointers, E.G, {}. C-style pointers used to point directly to memory addresses const: {:?}, mut: {:?}) -> value: {}",
            "[UNSAFE DATA-TYPE]".bold().truecolor(152, 251, 152),
            "*const T / *mut T".bold().truecolor(152, 251, 152),
            raw_const,
            raw_mut,
            *raw_const
        );
    }

    let vector: Vec<i32> = vec![10, 20, 30];
    println!(
        "\n{} Vector, E.G, {}. A {} allocated on the heap -> {:?}",
        "[COLLECTION DATA-TYPE]".bold().truecolor(152, 251, 152),
        "dynamically growable array".bold().truecolor(152, 251, 152),
        "Vec<T>".bold().truecolor(152, 251, 152),
        vector
    );

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("Rust", 2026);
    println!(
        "{} HashMap, E.G, {}. A key-value dictionary for fast lookups -> {:?}",
        "[COLLECTION DATA-TYPE]".bold().truecolor(152, 251, 152),
        "HashMap<K, V>".bold().truecolor(152, 251, 152),
        map
    );

    let option_val: Option<i32> = Some(42);
    println!(
        "\n{} Option, E.G, {}. Type representing the presence (Some) or absence (None) of a value -> {:?}",
        "[CONTROL DATA-TYPE]".bold().truecolor(152, 251, 152),
        "Option<T>".bold().truecolor(152, 251, 152),
        option_val
    );

    let result_val: Result<&str, &str> = Ok("Operação bem-sucedida");
    println!(
        "{} Result, E.G, {}. Type for error handling representing success (Ok) or failure (Err) -> {:?}",
        "[CONTROL DATA-TYPE]".bold().truecolor(152, 251, 152),
        "Result<T, E>".bold().truecolor(152, 251, 152),
        result_val
    );

    let usr_struct = Usr { name: String::from("Alex"), age: 30 };
    println!(
        "\n{} Struct, E.G, {}. A custom structure to group named fields -> {:?}",
        "[CUSTOM DATA-TYPE]".bold().truecolor(152, 251, 152),
        "struct".bold().truecolor(152, 251, 152),
        usr_struct
    );

    let enum_status = Status::Active;
    println!(
        "\n{} Enum, E.G, {}. An algebraic data type representing one of multiple variants -> {:?}",
        "[CUSTOM DATA-TYPE]".bold().truecolor(152, 251, 152),
        "enum".bold().truecolor(152, 251, 152),
        enum_status
    );

    let boxed_num = Box::new(100);
    println!(
        "\n{} Box, E.G, {}. Smart pointer for explicit heap allocation -> Value: {}",
        "[SMART-POINTER DATA-TYPE]".bold().truecolor(152, 251, 152),
        "Box<T>".bold().truecolor(152, 251, 152),
        boxed_num
    );

    let rc_data = Rc::new(500);
    let rc_clone = Rc::clone(&rc_data);
    println!(
        "{} Rc, E.G, {}. Reference counting pointer for single-threaded multiple ownership -> Owners: {}, Value: {}",
        "[SMART-POINTER DATA-TYPE]".bold().truecolor(152, 251, 152),
        "Rc<T>".bold().truecolor(152, 251, 152),
        Rc::strong_count(&rc_data),
        rc_clone
    );

    let arc_data = Arc::new(888);
    println!(
        "{} Arc, E.G, {}. Atomically reference counted pointer for thread-safe multiple ownership -> Value: {}",
        "[SMART-POINTER DATA-TYPE]".bold().truecolor(152, 251, 152),
        "Arc<T>".bold().truecolor(152, 251, 152),
        arc_data.add(88)
    );

    let mutex_data = Mutex::new(10);
    println!(
        "{} Mutex, E.G, {}. Mutual exclusion lock for safe data mutation between threads -> Inside Lock: {:?}",
        "[SMART-POINTER DATA-TYPE]".bold().truecolor(152, 251, 152),
        "Mutex<T>".bold().truecolor(152, 251, 152),
        mutex_data.lock().unwrap()
    );
}



fn print_static_and_constant() {
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
    println!(" ");
    println!(
        "{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i8".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "8 bits(1 byte)".bold().truecolor(152, 251, 152),
        i8::MIN,
        i8::MAX
    );
    println!(
        "{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i16".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "16 bits(2 byte)".bold().truecolor(152, 251, 152),
        i16::MIN,
        i16::MAX
    );
    println!(
        "{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i32".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "32 bits(4 byte)".bold().truecolor(152, 251, 152),
        i32::MIN,
        i32::MAX
    );
    println!(
        "{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i64".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "64 bits(8 byte)".bold().truecolor(152, 251, 152),
        i64::MIN,
        i64::MAX
    );
    println!(
        "{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i128".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "128 bits(16 byte)".bold().truecolor(152, 251, 152),
        i128::MIN,
        i128::MIN
    );
    println!(
        "{} {}. As a {}, Igual ao i32 em sistemas 32-bits ou i64 em sistemas 64-bits", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "isize".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152)
    );
}


fn unsigned_integers() {
    println!("\n{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "u8".bold().truecolor(152, 251, 152),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "8 bits(1 byte)".bold().truecolor(152, 251, 152),
        u8::MIN,
        u8::MAX
    );
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "u16".bold().truecolor(152, 251, 152),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "16 bits(2 byte)".bold().truecolor(152, 251, 152),
        u16::MIN,
        u16::MAX
    );
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "u32".bold().truecolor(152, 251, 152),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "32 bits(4 byte)".bold().truecolor(152, 251, 152),
        u32::MIN,
        u32::MAX
    );
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "u64".bold().truecolor(152, 251, 152),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "64 bits(8 byte)".bold().truecolor(152, 251, 152),
        u64::MIN,
        u64::MAX
    );
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "u128".bold().truecolor(152, 251, 152),
        "UNSIGNED integer".bold().truecolor(152, 251, 152),
        "128 bits(16 byte)".bold().truecolor(152, 251, 152),
        u128::MIN,
        u128::MAX
    );
    println!(
        "{} {}. As a {}, Igual ao u32 em sistemas 32-bits ou u64 em sistemas 64-bits", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "usize".bold().truecolor(152, 251, 152),
        "UNSIGNED integer".bold().truecolor(152, 251, 152)
    );
}

fn floats()  { 
    println!("\n{} {}. As a {}, {} 32 bits",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "f32".bold().truecolor(152, 251, 152),
        "decimal type with simple precision (IEEE-754)".bold().truecolor(152, 251, 152),
        "inherentemente signed".bold().truecolor(152, 251, 152)
    );


    println!("{} {}. As a {} (better for math precision), {} 64 bits, however also not recomended to monetary operations",
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "f64".bold().truecolor(152, 251, 152),
        "decimal type with double precision".bold().truecolor(152, 251, 152),
        "inherentemente signed".bold().truecolor(152, 251, 152)
    );
}
