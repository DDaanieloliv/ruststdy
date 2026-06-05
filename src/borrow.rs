use colored::Colorize;

pub fn rust_borrow() {
    ownership();
    lifetime();
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

fn lifetime() {
    println!("{} The {}. A reference never can live more than the value pointed",
        "[BORROW-CHECKER RULE]".bold().red(),
        "Lifetime Rule".red(),
    );
    /*
     * let r;
     *
     * {
     *  let x = 10;
     *  r = &x;
     * } <- r dropped here !
     * 
     * println!("{r}");
     *
     * */
}
