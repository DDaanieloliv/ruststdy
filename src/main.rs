use colored::Colorize;

const CONST_VAR: i32 = 00;
static STATIC_VAR: i32 = 00; // IS UNSAFE: be mutabel
static mut MUTABLE_STATIC_VAR: i32 = 700; // IS UNSAFE: be mutabel

fn main() {
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
        MUTABLE_STATIC_VAR = 7; // SUSCEPTIBLE: data-race
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
    let min_i8: i8 = -128;
    let max_i8: i8 = 127;
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i8".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "8 bits(1 byte)".bold().truecolor(152, 251, 152),
        max_i8,
        min_i8
    );


    let min_i16: i16 = -32768;
    let max_i16: i16 = 32767;
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i16".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "16 bits(2 byte)".bold().truecolor(152, 251, 152),
        min_i16,
        max_i16
    );

    let min_i32: i32 = -2147483648;
    let max_i32: i32 =  2147483647;
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i32".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "32 bits(4 byte)".bold().truecolor(152, 251, 152),
        min_i32,
        max_i32
    );


    let min_i64: i64 = -9223372036854775808;
    let max_i64: i64 =  9223372036854775807;
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i64".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "64 bits(8 byte)".bold().truecolor(152, 251, 152),
        min_i64,
        max_i64
    );


    let min_i128: &str = "-1.70 * 10^38";
    let max_i128: &str =  "1.70 * 10^38 ~";
    println!("{} {}. As a {}, {} with interval of min and max -> {} and {}", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "i128".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152),
        "128 bits(16 byte)".bold().truecolor(152, 251, 152),
        min_i128,
        max_i128
    );


    println!("{} {}. As a {}, Igual ao i32 em sistemas 32-bits ou i64 em sistemas 64-bits", 
        "[DATA-TYPE]".bold().truecolor(152, 251, 152),
        "isize".bold().truecolor(152, 251, 152),
        "SIGNED integer".bold().truecolor(152, 251, 152)
    );
}

fn unsigned_integers() {
}
