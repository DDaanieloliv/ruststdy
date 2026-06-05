pub fn deref_look() {

    let mut var: i64 = 7;
    /// defining &var inside of fn = ref created on stack-frame
    fn dosmt(var: &mut i64) -> &i64 {
        println!("[DEREF_COERSION PARAMETER-TYPE] ´&&i64´ TO ´&i64´ (NOT ALLOWED)");
        println!("[OBJECT-REFERENCE] {:p}", var);
        println!("       ↓");
        println!("[LOCAL-STACK-REF] {:p}", &var);
        println!("       ↓");
        println!("[LOCAL-STACK-REF] {:p}", &&var);
        println!("       ↓");
        *var += 0;
        &&(*var) // DEREF COERSION BY RETURN TYPE
    }

    fn doesmt(var: &i64) -> &i64 {
        println!("[DEREF_COERSION PARAMETER-TYPE] ´&&i64´ TO ´&i64´");
        println!("[OBJECT-REFERENCE] &i64 {:p}", var);
        println!("       ↓");
        println!("[LOCAL-STACK-REF] {:p}", &var);
        println!("       ↓");
        println!("[LOCAL-STACK-REF] {:p}", &&var);
        println!("       ↓");
        &&var // DEREF COERSION BY RETURN TYPE
    }

    let x_return = doesmt(&&var); // DEREF COERSION BY PARAMETER
    println!("[DEREF_COERSION RETURN-TYPE] ´&&i64´ TO ´&i64´ -> {:p}", x_return);

    // DEREF COERSION BY PARAMETER ´&&mut var´ -> ´&mut var´ NOT ALLOWED 
    // SINCE A MUTABLE REFERENCE MEANS EXCLUSIBILITY OF ITS REFERENCE TO 
    // OBJECT AND BY ´&&mut var´ THE COMPILER NOT GARANTEES THAT EXCLUSIVE REFERENCE
    let w_return = dosmt(&mut var);
    println!("[DEREF_COERSION RETURN-TYPE] ´&&i64´ TO ´&i64´ -> {:p}", w_return);

    /*
     *
     * it's wrong by you cant be able to have a mutable reference to an object
     * while mantain a reference to a imutable reference of it's object, being
     * allow to have N Imutable References to an object or 1 Mutable Reference
     *
     * let x_return = doesmt(&var);
     * let w_return = dosmt(&mut var);
     * println!("[DOESMT] -> {}", x_return);
     * println!("[DOSMT] -> {}", w_return);
     *
     * */
}
