// main.rs

#![no_main]
#![no_std]

mod life;

use life::{life, done};

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;

use microbit::{
    board::Board, 
    display::blocking::Display, 
    hal::{prelude::*, timer}
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display: Display = Display::new(board.display_pins);
    let time = 100; //ms
    let mut delay = timer::Timer::new(board.TIMER0);
    let mut image = [
        [1,0,0,0,0],
        [1,0,1,0,0],
        [1,0,1,0,1],
        [0,0,1,0,1],
        [0,0,0,0,1],
    ];
    loop {
        rprintln!("starting...");
        life(&mut image);
        display.show(& mut delay, image.clone(), time);
        delay.delay_ms(time);
    }
}
