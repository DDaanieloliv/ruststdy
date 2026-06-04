pub fn deref_look() {

    let mut var: i64 = 7;
    /// double-referencing var pointer!
    /// defining &var inside of fn = ref created on stack-frame
    fn dosmt(var: &mut i64) -> &i64 {
        println!("{:p}", var);
        println!("{:p}", &var);
        println!("{:p}", &&var);
        *var += 0;
        &(*var)
    }

    fn doesmt(var: &i64) -> &i64 {
        println!("{:p}", var);
        println!("{:p}", &var);
        println!("{:p}", &&var);
        &&var
    }

    let x_return = doesmt(&var);
    println!("[DOESMT] -> {}", x_return);

    let w_return = dosmt(&mut var);
    println!("[DOSMT] -> {}", w_return);

    /*
     *
     * it's wrong by ...
     *
     * let x_return = doesmt(&var);
     * let w_return = dosmt(&mut var);
     * println!("[DOESMT] -> {}", x_return);
     * println!("[DOSMT] -> {}", w_return);
     *
     * */
}
