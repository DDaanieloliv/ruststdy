mod type_view;
mod deref_look;

use ::colored::Colorize;
use deref_look::deref_look;
use type_view::type_overview;
use std::{
    mem::MaybeUninit,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

const CLOCK_REALTIME_COARSE: i32 = 5;

unsafe extern "C" {
    fn clock_gettime(clock_id: i32, tp: *mut Timespec) -> i32;
}

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

fn main() {
    type_overview();
    deref_look();

    unix_epoch();
    coarse_clock();
    libc_coarse_clock();

}


fn coarse_clock() {
    let init = Instant::now();

    let unix_epoch = unsafe {
        let mut t = MaybeUninit::<Timespec>::uninit();
        clock_gettime(CLOCK_REALTIME_COARSE, t.as_mut_ptr());
        t.assume_init().tv_sec
    };

    let secs = (unix_epoch % 86400) as u32;
    let h = (secs / 3600) as i32;
    let m = (secs % 3600) / 60;
    let utc_hour = (h + 24 - 3) % 24;
    println!(
        "coarse clock_time -> {:02}:{:02} in {:?}",
        utc_hour,
        m,
        init.elapsed()
    );
}

fn libc_coarse_clock() {
    let init = Instant::now();

    let mut t = MaybeUninit::<libc::timespec>::uninit();
    let unix_epoch = unsafe {
        libc::clock_gettime(libc::CLOCK_REALTIME_COARSE, t.as_mut_ptr());
        t.assume_init().tv_sec
    };
    let secs = (unix_epoch % 86400) as u32;
    let h = (secs / 3600) as i32;
    let m = (secs % 3600) / 60;
    let utc_hour = (h + 24 - 3) % 24;
    println!(
        "libc clock_time -> {}",
        format!("{:02}:{:02} in {:?}", utc_hour, m, init.elapsed()).blue()
    );
}

fn unix_epoch() {
    let init = Instant::now();

    let unix_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let secs = (unix_epoch % 86400) as u32;
    let h = (secs / 3600) as i32;
    let m = (secs % 3600) / 60;
    let utc_hour = (h + 24 - 3) % 24;
    println!(
        "timeSystem clock_time -> {:02}:{:02} in {:?}",
        utc_hour,
        m,
        init.elapsed()
    );
}
