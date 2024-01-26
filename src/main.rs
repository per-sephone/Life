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

use nanorand::{pcg64::Pcg64, Rng, SeedableRng};

fn generate_random_board(board: &mut [[u8; 5]; 5]) {
    let mut rng = nanorand::Pcg64::new_seed(1);

    for row in 0..5 {
        for col in 0..5 {
            let b: bool = rng.generate();
            board[row][col] = b as u8;
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display: Display = Display::new(board.display_pins);
    let time = 100; //ms
    let mut delay = timer::Timer::new(board.TIMER0);
    let mut image = [[0;5];5];

    generate_random_board(&mut image);

    loop {
        rprintln!("starting...");
        life(&mut image);
        display.show(& mut delay, image.clone(), time);
        delay.delay_ms(time);
    }
}
