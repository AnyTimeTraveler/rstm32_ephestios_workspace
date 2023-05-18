#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::peripherals::PA0;

#[embassy_executor::task]
async fn blinker(mut led: Output<'static, PA0>, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let led = Output::new(p.PA0, Level::Low, Speed::Low);
    unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
}
