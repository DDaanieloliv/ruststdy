mod type_view;

use std::time::{SystemTime, UNIX_EPOCH};

use type_view::type_overview;

fn main() {
    type_overview();

    let var: i64 = 7;
    /// double-referencing var pointer!
    fn dosmt(var: &i64) -> &i64 {
        println!("{:p}", var);
        println!("{:p}", &var);
        println!("{:p}", &&var);
        &&var
    }

    dosmt(&var);

    let unx_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let secs = (unx_epoch % 86400) as u32;
    let h = (secs / 3600) as i32;
    let m =  (secs % 3600) / 60;
    let utc_hour = (h + 24 - 3) % 24;
    println!("clock_time -> {:02}:{:02}", utc_hour, m);
}
