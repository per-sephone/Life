#![no_main]
#![no_std]

mod life;

use cortex_m_rt::entry;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        prelude::*,
        pac::TIMER1,
        timer::Timer,
    },
};
use nanorand::{pcg64::Pcg64, Rng, SeedableRng};
use panic_halt as _;



#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _board = Board::take().unwrap();
    let mut counter = 0u64;
    loop {
        rprintln!("{}", counter);
        counter += 1;
    }
}
