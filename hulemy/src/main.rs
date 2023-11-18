#![no_std]
#![no_main]

use panic_halt as _;

use core::fmt::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    let peripherals = boxxo::Peripherals::take().unwrap();

    writeln!(stdout, "Sludge is awesome!").unwrap();

    peripherals
        .SYSCTL
        .rcgcgpio
        .write(|w| w.sysctl_rcgcgpio_r5().set_bit());

    peripherals
        .GPIOF
        .dir
        .modify(|r, w| unsafe { w.bits(r.bits() | 1u32 << 1) });

    peripherals
        .GPIOF
        .pur
        .modify(|r, w| unsafe { w.bits(r.bits() | 1u32 << 1) });

    peripherals
        .GPIOF
        .den
        .modify(|r, w| unsafe { w.bits(r.bits() | 1u32 << 1) });

    peripherals
        .GPIOF
        .data
        .modify(|r, w| unsafe { w.bits(r.bits() | 1u32 << 1) });

    writeln!(stdout, "Sludge is cute!").unwrap();

    loop {}
}
