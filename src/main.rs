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

/// Uses the PCG64 random number generator to produce random boolean values.
/// These values are cast into u8 and assigned to the cells in the LED display.
/// It uses the hardware's random number generator as a seed.
/// 
/// # Arguments
///
/// * `led_display` - A mutable reference to a 5x5 array representing the LED display.
/// * `rng` - A mutable reference to the hardware's random number generator

fn generate_random_board(led_display: &mut [[u8; 5]; 5], rng: &mut HWRng) {
    let mut rand = Pcg64::new_seed(rng.random_u64() as u128);

    for (_, row) in led_display.iter_mut().enumerate().take(5) {
        for (_, cell) in row.iter_mut().enumerate().take(5)  {
            let b: bool = rand.generate();
            *cell = b as u8;
        }
    }
}

/// Flips the binary values in the 5x5 LED display. If a cell
/// contains a `0`, it is changed to `1`, and vice versa.
///
/// # Arguments
///
/// * `led_display` - A mutable reference to a 5x5 array representing the LED display.

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

/// Main function, gets the board peripherals, generates a random board.
/// If button A is pressed, the LED lights are randomized
/// If button B is pressed, the LED lights show the complement of what was previously displayed.
/// If there are no LED lights on the screen, a random LED pattern is generated.
/// Otherwise, the game of Life plays on the LED lights.
#[entry]
fn main() -> ! {
    rtt_init_print!();
    let one_hundred_ms = 100; //ms
    let five_hundred_ms: u32 = 500;

    let board = Board::take().unwrap();
    let mut rng = HWRng::new(board.RNG);
    let mut display: Display = Display::new(board.display_pins);

    let mut delay = timer::Timer::new(board.TIMER0);
    let mut led_display = [[0; 5]; 5];
    let buttons = board.buttons;

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
