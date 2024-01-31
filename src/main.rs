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

fn generate_random_board(led_display: &mut [[u8; 5]; 5]) {
    let mut rng = nanorand::Pcg64::new_seed(1);
    for row in 0..5 {
        for col in 0..5 {
            let b: bool = rng.generate();
            led_display[row][col] = b as u8;
        }
    }
}

fn complement(led_display: &mut [[u8; 5]; 5]) {
    for row in 0..5 {
        for col in 0..5 {
            if led_display[row][col] == 0 {
                led_display[row][col] = 1;
            }
            else { //led_display[row][col] == 1;
                led_display[row][col] = 0;
            }

        }
    }
}

fn all_lights_are_off(led_display: &mut [[u8; 5]; 5]) -> bool {
    for row in 0..5 {
        for col in 0..5 {
            if led_display[row][col] == 1 {
                return false;
            }
        }
    }
    true
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display: Display = Display::new(board.display_pins);
    let one_hundred_ms = 100; //ms
    let mut delay = timer::Timer::new(board.TIMER0);
    let mut led_display = [[0;5];5];
    let buttons = board.buttons;
    let five_hundred_ms: u32 = 500;

    generate_random_board(&mut led_display);

    loop {
        while buttons.button_a.is_low().unwrap() {
            generate_random_board(&mut led_display);
            display.show(& mut delay, led_display.clone(), one_hundred_ms);
        }

        if buttons.button_b.is_low().unwrap() {
            complement(&mut led_display);
            display.show(& mut delay, led_display.clone(), one_hundred_ms);
            delay.delay_ms(five_hundred_ms);
        }

        if all_lights_are_off(&mut led_display) == true {
            delay.delay(five_hundred_ms);
            generate_random_board(&mut led_display);
        }

        life(&mut led_display);
        display.show(& mut delay, led_display.clone(), one_hundred_ms);
        delay.delay_ms(one_hundred_ms);
    }
}
