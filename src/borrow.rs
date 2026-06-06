use colored::Colorize;

pub fn rust_borrow() {
    ownership();
    borrowing();
    lifetime();
    move_ownership_semantics();
}


fn ownership() {
    println!("{} The {}. Every value has exactly {} at a time, and {} it's variable {}",
        "[BORROW-CHECKER WARN]".bold().red(),
        "Ownership Rule".yellow(),
        "ONE OWNER".yellow(),
        "automatically destryed/dropped when".yellow(),
        "goes out of scope".yellow()
    );
}

fn borrowing() {
    println!("{} {}. To use a value without transferer the Ownership is created a reference, by using ´&T´, binding it to a func an borrowing it without transferer ownership",
        "[BORROW-CHECKER WARN]".bold().red(),
        "Borrowing".yellow(),
    );
}

fn lifetime() {
    println!("{} The {}. A reference never can live more than the value pointed",
        "[BORROW-CHECKER RULE]".bold().red(),
        "Lifetime Rule".red(),
    );
}

fn move_ownership_semantics() {
    println!("{} The {}. {}, transferring back the ownership",
        "[BORROW-CHECKER WARN]".bold().red(),
        "Ownership MOVE semantics".yellow(),
        "Passing a value to an func transfers the Ownership and the same when it returns".yellow(),
    );
}
