#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rp2040_hal as hal;
use hal::Clock;

/// セカンドステージブートローダ。フラッシュ最初の 256B に置かれて、
/// QSPI フラッシュを XIP モードに切り替えてからユーザコードに飛ぶ。
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

/// Pico W の外付け水晶振動子は 12 MHz。
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[hal::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // GPIO22 に外付け LED (オンボード LED は CYW43 Wi-Fi チップ管理なので使えない)
    let mut led = pins.gpio22.into_push_pull_output();

    let core = cortex_m::Peripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(
        core.SYST,
        clocks.system_clock.freq().to_Hz(),
    );

    loop {
        led.set_high().unwrap();
        delay.delay_ms(500);
        led.set_low().unwrap();
        delay.delay_ms(500);
    }
}