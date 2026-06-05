use colored::Colorize;

pub fn rust_borrow() {
    ownership();
}


fn ownership() {
    println!("{} The {}. Every value has exactly {} at a time, and {} it's variable {}",
        "[BORROW-CHECKER RULE]".bold().red(),
        "Ownership Rule".red(),
        "ONE OWNER".red(),
        "automatically destryed/dropped when".red(),
        "goes out of scope".red()
    );
}
