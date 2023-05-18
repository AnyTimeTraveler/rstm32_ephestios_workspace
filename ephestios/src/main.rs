#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output};
use embassy_time::{Duration, Timer};

use {defmt as _, panic_probe as _}; // global logger

#[embassy_executor::task]
async fn blinker(mut led: Output<'static, P0_13>, interval: Duration) {
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

    let led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
    unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
}
