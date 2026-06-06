use colored::Colorize;

pub fn rust_borrow() {
    ownership();
    move_ownership_semantics();
    borrowing();
    lifetime();
    aliasing_xor_mutability();
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

fn move_ownership_semantics() {
    println!("{} The {}. {}, transferring back the ownership",
        "[BORROW-CHECKER WARN]".bold().red(),
        "Ownership MOVE semantics".yellow(),
        "Passing a value to an func transfers the Ownership and the same when it returns".yellow(),
    );
}

fn borrowing() {
    println!("{} {}. To {} is created a {}, by using {} or {}, binding it to a func and borrowing it without transferer ownership",
        "[BORROW-CHECKER WARN]".bold().red(),
        "Borrowing".yellow(),
        "use a value without transferer the Ownership".yellow(),
        "reference".yellow(),
        "´&T´".yellow(),
        "´&mut T´ to modify it's value".yellow(),
    );
}

fn lifetime() {
    println!("{} The {}. A reference never can live more than the value pointed",
        "[BORROW-CHECKER RULE]".bold().red(),
        "Lifetime Rule".red(),
    );
}

fn aliasing_xor_mutability() {
    println!("{} The {}. May have {} {} {} at a time, {}",
        "[BORROW-CHECKER RULE]".bold().red(),
        "Aliasing or Mutability rule".red(),
        "N° Imutable References(´&T´)".red(),
        "or".yellow(),
        "One Mutable Reference(´&mut T´)".red(),
        "and no one Imutable ref can coexists with it".yellow(),
    );
}
