#![no_std]
#![no_main]

use core::{cell::Cell, panic::PanicInfo};
use cortex_m::{
    interrupt::{free, Mutex},
    peripheral::NVIC,
};
use cortex_m_rt::entry; // The runtime
use defmt_rtt as _;
use stm32_hal2::{
    self,
    clocks::Clocks,
    gpio::{self, Edge, Pin, PinMode, Port},
    pac::{self, interrupt},
};
static FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
#[entry]

fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();
    let mut button = Pin::new(Port::C, 13, PinMode::Input);
    button.enable_interrupt(Edge::Falling);
    unsafe {
        NVIC::unmask(pac::Interrupt::EXTI15_10);
    }
    let mut led = Pin::new(Port::A, 5, PinMode::Output);
    led.set_low();
    loop {
        if free(|cs| FLAG.borrow(cs).get() == true) {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
#[interrupt]
fn EXTI15_10() {
    gpio::clear_exti_interrupt(0);
    if free(|cs| FLAG.borrow(cs).get() == false) {
        free(|cs| FLAG.borrow(cs).set(true));
    } else {
        free(|cs| FLAG.borrow(cs).set(false));
    }
}
