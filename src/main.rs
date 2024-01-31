// main.rs

#![no_main]
#![no_std]

mod life;

use life::{done, life};

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use nanorand::{pcg64::Pcg64, Rng};

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, timer, Rng as HWRng},
};

fn generate_random_board(led_display: &mut [[u8; 5]; 5], rng: &mut HWRng) {
    let mut rand = Pcg64::new_seed(rng.random_u64() as u128);

    for (_, row) in led_display.iter_mut().enumerate().take(5) {
        for (_, cell) in row.iter_mut().enumerate().take(5)  {
            let b: bool = rand.generate();
            *cell = b as u8;
        }
    }
}

fn complement(led_display: &mut [[u8; 5]; 5]) {
    for (_, row) in led_display.iter_mut().enumerate().take(5) {
        for (_, cell) in row.iter_mut().enumerate().take(5)  {
            if *cell == 0 {
                *cell = 1;
            } else { //*cell == 1;
                *cell = 0;
            }
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut rng = HWRng::new(board.RNG);
    let mut display: Display = Display::new(board.display_pins);
    let one_hundred_ms = 100; //ms
    let mut delay = timer::Timer::new(board.TIMER0);
    let mut led_display = [[0; 5]; 5];
    let buttons = board.buttons;
    let five_hundred_ms: u32 = 500;

    generate_random_board(&mut led_display, &mut rng);

    loop {
        while buttons.button_a.is_low().unwrap() {
            generate_random_board(&mut led_display, &mut rng);
            display.show(&mut delay, led_display, one_hundred_ms);
        }

        if buttons.button_b.is_low().unwrap() {
            complement(&mut led_display);
            display.show(&mut delay, led_display, one_hundred_ms);
            delay.delay_ms(five_hundred_ms);
        }

        if done(&led_display) {
            delay.delay(five_hundred_ms);
            generate_random_board(&mut led_display, &mut rng);
        }

        life(&mut led_display);
        display.show(&mut delay, led_display, one_hundred_ms);
        delay.delay_ms(one_hundred_ms);
    }
}
