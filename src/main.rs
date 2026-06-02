mod type_view;

use type_view::type_overview;

fn main() {
    let var: i64 = 7;
    /// double-referencing var pointer!
    fn dosmt(var: &i64) -> &i64 {
        println!("{:p}", var);
        println!("{:p}", &var);
        println!("{:p}", &&var);
        &&var
    }

    dosmt(&var);
    type_overview();
}
