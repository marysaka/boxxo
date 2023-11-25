#![no_std]

pub use boxxo as pac;

pub mod init_state {
    pub struct Enabled<T = ()>(pub T);

    pub struct Disabled;
}

#[allow(non_snake_case)]
pub struct Peripherals {
    // TODO: everything
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        Some(Self::new(pac::Peripherals::take()?))
    }

    pub unsafe fn steal() -> Self {
        Self::new(pac::Peripherals::steal())
    }

    fn new(_p: pac::Peripherals) -> Self {
        Self {}
    }
}
