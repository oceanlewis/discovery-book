#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut delay_ms: u16 = 50;
    let v_delay_ms = Volatile::new(&mut delay_ms).read_only();

    loop {
        for current in (0..8).cycle() {
            let next = (current + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(v_delay_ms.read());

            leds[current].off().ok();
            delay.delay_ms(v_delay_ms.read());
        }
    }
}
