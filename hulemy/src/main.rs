#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_semihosting::hio;
use core::fmt::Write;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Sludge is cute!").unwrap();

    loop {}
}
