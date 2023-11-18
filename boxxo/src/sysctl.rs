#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Identification 0"]
    pub did0: DID0,
    #[doc = "0x04 - Device Identification 1"]
    pub did1: DID1,
    #[doc = "0x08 - Device Capabilities 0"]
    pub dc0: DC0,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Device Capabilities 1"]
    pub dc1: DC1,
    #[doc = "0x14 - Device Capabilities 2"]
    pub dc2: DC2,
    #[doc = "0x18 - Device Capabilities 3"]
    pub dc3: DC3,
    #[doc = "0x1c - Device Capabilities 4"]
    pub dc4: DC4,
    #[doc = "0x20 - Device Capabilities 5"]
    pub dc5: DC5,
    #[doc = "0x24 - Device Capabilities 6"]
    pub dc6: DC6,
    #[doc = "0x28 - Device Capabilities 7"]
    pub dc7: DC7,
    #[doc = "0x2c - Device Capabilities 8 ADC Channels"]
    pub dc8: DC8,
    #[doc = "0x30 - Brown-Out Reset Control"]
    pub pborctl: PBORCTL,
    _reserved12: [u8; 0x0c],
    #[doc = "0x40 - Software Reset Control 0"]
    pub srcr0: SRCR0,
    #[doc = "0x44 - Software Reset Control 1"]
    pub srcr1: SRCR1,
    #[doc = "0x48 - Software Reset Control 2"]
    pub srcr2: SRCR2,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x54 - Interrupt Mask Control"]
    pub imc: IMC,
    #[doc = "0x58 - Masked Interrupt Status and Clear"]
    pub misc: MISC,
    #[doc = "0x5c - Reset Cause"]
    pub resc: RESC,
    #[doc = "0x60 - Run-Mode Clock Configuration"]
    pub rcc: RCC,
    _reserved20: [u8; 0x08],
    #[doc = "0x6c - GPIO High-Performance Bus Control"]
    pub gpiohbctl: GPIOHBCTL,
    #[doc = "0x70 - Run-Mode Clock Configuration 2"]
    pub rcc2: RCC2,
    _reserved22: [u8; 0x08],
    #[doc = "0x7c - Main Oscillator Control"]
    pub moscctl: MOSCCTL,
    _reserved23: [u8; 0x80],
    #[doc = "0x100 - Run Mode Clock Gating Control Register 0"]
    pub rcgc0: RCGC0,
    #[doc = "0x104 - Run Mode Clock Gating Control Register 1"]
    pub rcgc1: RCGC1,
    #[doc = "0x108 - Run Mode Clock Gating Control Register 2"]
    pub rcgc2: RCGC2,
    _reserved26: [u8; 0x04],
    #[doc = "0x110 - Sleep Mode Clock Gating Control Register 0"]
    pub scgc0: SCGC0,
    #[doc = "0x114 - Sleep Mode Clock Gating Control Register 1"]
    pub scgc1: SCGC1,
    #[doc = "0x118 - Sleep Mode Clock Gating Control Register 2"]
    pub scgc2: SCGC2,
    _reserved29: [u8; 0x04],
    #[doc = "0x120 - Deep Sleep Mode Clock Gating Control Register 0"]
    pub dcgc0: DCGC0,
    #[doc = "0x124 - Deep-Sleep Mode Clock Gating Control Register 1"]
    pub dcgc1: DCGC1,
    #[doc = "0x128 - Deep Sleep Mode Clock Gating Control Register 2"]
    pub dcgc2: DCGC2,
    _reserved32: [u8; 0x18],
    #[doc = "0x144 - Deep Sleep Clock Configuration"]
    pub dslpclkcfg: DSLPCLKCFG,
    _reserved33: [u8; 0x04],
    #[doc = "0x14c - System Properties"]
    pub sysprop: SYSPROP,
    #[doc = "0x150 - Precision Internal Oscillator Calibration"]
    pub piosccal: PIOSCCAL,
    #[doc = "0x154 - Precision Internal Oscillator Statistics"]
    pub pioscstat: PIOSCSTAT,
    _reserved36: [u8; 0x08],
    #[doc = "0x160 - PLL Frequency 0"]
    pub pllfreq0: PLLFREQ0,
    #[doc = "0x164 - PLL Frequency 1"]
    pub pllfreq1: PLLFREQ1,
    #[doc = "0x168 - PLL Status"]
    pub pllstat: PLLSTAT,
    _reserved39: [u8; 0x24],
    #[doc = "0x190 - Device Capabilities 9 ADC Digital Comparators"]
    pub dc9: DC9,
    _reserved40: [u8; 0x0c],
    #[doc = "0x1a0 - Non-Volatile Memory Information"]
    pub nvmstat: NVMSTAT,
    _reserved41: [u8; 0x015c],
    #[doc = "0x300 - Watchdog Timer Peripheral Present"]
    pub ppwd: PPWD,
    #[doc = "0x304 - Timer Peripheral Present"]
    pub pptimer: PPTIMER,
    #[doc = "0x308 - General-Purpose Input/Output Peripheral Present"]
    pub ppgpio: PPGPIO,
    #[doc = "0x30c - Micro Direct Memory Access Peripheral Present"]
    pub ppdma: PPDMA,
    _reserved45: [u8; 0x04],
    #[doc = "0x314 - Hibernation Peripheral Present"]
    pub pphib: PPHIB,
    #[doc = "0x318 - Universal Asynchronous Receiver/Transmitter Peripheral Present"]
    pub ppuart: PPUART,
    #[doc = "0x31c - Synchronous Serial Interface Peripheral Present"]
    pub ppssi: PPSSI,
    #[doc = "0x320 - Inter-Integrated Circuit Peripheral Present"]
    pub ppi2c: PPI2C,
    _reserved49: [u8; 0x04],
    #[doc = "0x328 - Universal Serial Bus Peripheral Present"]
    pub ppusb: PPUSB,
    _reserved50: [u8; 0x08],
    #[doc = "0x334 - Controller Area Network Peripheral Present"]
    pub ppcan: PPCAN,
    #[doc = "0x338 - Analog-to-Digital Converter Peripheral Present"]
    pub ppadc: PPADC,
    #[doc = "0x33c - Analog Comparator Peripheral Present"]
    pub ppacmp: PPACMP,
    #[doc = "0x340 - Pulse Width Modulator Peripheral Present"]
    pub pppwm: PPPWM,
    #[doc = "0x344 - Quadrature Encoder Interface Peripheral Present"]
    pub ppqei: PPQEI,
    _reserved55: [u8; 0x10],
    #[doc = "0x358 - EEPROM Peripheral Present"]
    pub ppeeprom: PPEEPROM,
    #[doc = "0x35c - Wide Timer Peripheral Present"]
    pub ppwtimer: PPWTIMER,
    _reserved57: [u8; 0x01a0],
    #[doc = "0x500 - Watchdog Timer Software Reset"]
    pub srwd: SRWD,
    #[doc = "0x504 - Timer Software Reset"]
    pub srtimer: SRTIMER,
    #[doc = "0x508 - General-Purpose Input/Output Software Reset"]
    pub srgpio: SRGPIO,
    #[doc = "0x50c - Micro Direct Memory Access Software Reset"]
    pub srdma: SRDMA,
    _reserved61: [u8; 0x04],
    #[doc = "0x514 - Hibernation Software Reset"]
    pub srhib: SRHIB,
    #[doc = "0x518 - Universal Asynchronous Receiver/Transmitter Software Reset"]
    pub sruart: SRUART,
    #[doc = "0x51c - Synchronous Serial Interface Software Reset"]
    pub srssi: SRSSI,
    #[doc = "0x520 - Inter-Integrated Circuit Software Reset"]
    pub sri2c: SRI2C,
    _reserved65: [u8; 0x04],
    #[doc = "0x528 - Universal Serial Bus Software Reset"]
    pub srusb: SRUSB,
    _reserved66: [u8; 0x08],
    #[doc = "0x534 - Controller Area Network Software Reset"]
    pub srcan: SRCAN,
    #[doc = "0x538 - Analog-to-Digital Converter Software Reset"]
    pub sradc: SRADC,
    #[doc = "0x53c - Analog Comparator Software Reset"]
    pub sracmp: SRACMP,
    #[doc = "0x540 - Pulse Width Modulator Software Reset"]
    pub srpwm: SRPWM,
    #[doc = "0x544 - Quadrature Encoder Interface Software Reset"]
    pub srqei: SRQEI,
    _reserved71: [u8; 0x10],
    #[doc = "0x558 - EEPROM Software Reset"]
    pub sreeprom: SREEPROM,
    #[doc = "0x55c - Wide Timer Software Reset"]
    pub srwtimer: SRWTIMER,
    _reserved73: [u8; 0xa0],
    #[doc = "0x600 - Watchdog Timer Run Mode Clock Gating Control"]
    pub rcgcwd: RCGCWD,
    #[doc = "0x604 - Timer Run Mode Clock Gating Control"]
    pub rcgctimer: RCGCTIMER,
    #[doc = "0x608 - General-Purpose Input/Output Run Mode Clock Gating Control"]
    pub rcgcgpio: RCGCGPIO,
    #[doc = "0x60c - Micro Direct Memory Access Run Mode Clock Gating Control"]
    pub rcgcdma: RCGCDMA,
    _reserved77: [u8; 0x04],
    #[doc = "0x614 - Hibernation Run Mode Clock Gating Control"]
    pub rcgchib: RCGCHIB,
    #[doc = "0x618 - Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
    pub rcgcuart: RCGCUART,
    #[doc = "0x61c - Synchronous Serial Interface Run Mode Clock Gating Control"]
    pub rcgcssi: RCGCSSI,
    #[doc = "0x620 - Inter-Integrated Circuit Run Mode Clock Gating Control"]
    pub rcgci2c: RCGCI2C,
    _reserved81: [u8; 0x04],
    #[doc = "0x628 - Universal Serial Bus Run Mode Clock Gating Control"]
    pub rcgcusb: RCGCUSB,
    _reserved82: [u8; 0x08],
    #[doc = "0x634 - Controller Area Network Run Mode Clock Gating Control"]
    pub rcgccan: RCGCCAN,
    #[doc = "0x638 - Analog-to-Digital Converter Run Mode Clock Gating Control"]
    pub rcgcadc: RCGCADC,
    #[doc = "0x63c - Analog Comparator Run Mode Clock Gating Control"]
    pub rcgcacmp: RCGCACMP,
    #[doc = "0x640 - Pulse Width Modulator Run Mode Clock Gating Control"]
    pub rcgcpwm: RCGCPWM,
    #[doc = "0x644 - Quadrature Encoder Interface Run Mode Clock Gating Control"]
    pub rcgcqei: RCGCQEI,
    _reserved87: [u8; 0x10],
    #[doc = "0x658 - EEPROM Run Mode Clock Gating Control"]
    pub rcgceeprom: RCGCEEPROM,
    #[doc = "0x65c - Wide Timer Run Mode Clock Gating Control"]
    pub rcgcwtimer: RCGCWTIMER,
    _reserved89: [u8; 0xa0],
    #[doc = "0x700 - Watchdog Timer Sleep Mode Clock Gating Control"]
    pub scgcwd: SCGCWD,
    #[doc = "0x704 - Timer Sleep Mode Clock Gating Control"]
    pub scgctimer: SCGCTIMER,
    #[doc = "0x708 - General-Purpose Input/Output Sleep Mode Clock Gating Control"]
    pub scgcgpio: SCGCGPIO,
    #[doc = "0x70c - Micro Direct Memory Access Sleep Mode Clock Gating Control"]
    pub scgcdma: SCGCDMA,
    _reserved93: [u8; 0x04],
    #[doc = "0x714 - Hibernation Sleep Mode Clock Gating Control"]
    pub scgchib: SCGCHIB,
    #[doc = "0x718 - Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
    pub scgcuart: SCGCUART,
    #[doc = "0x71c - Synchronous Serial Interface Sleep Mode Clock Gating Control"]
    pub scgcssi: SCGCSSI,
    #[doc = "0x720 - Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
    pub scgci2c: SCGCI2C,
    _reserved97: [u8; 0x04],
    #[doc = "0x728 - Universal Serial Bus Sleep Mode Clock Gating Control"]
    pub scgcusb: SCGCUSB,
    _reserved98: [u8; 0x08],
    #[doc = "0x734 - Controller Area Network Sleep Mode Clock Gating Control"]
    pub scgccan: SCGCCAN,
    #[doc = "0x738 - Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
    pub scgcadc: SCGCADC,
    #[doc = "0x73c - Analog Comparator Sleep Mode Clock Gating Control"]
    pub scgcacmp: SCGCACMP,
    #[doc = "0x740 - Pulse Width Modulator Sleep Mode Clock Gating Control"]
    pub scgcpwm: SCGCPWM,
    #[doc = "0x744 - Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
    pub scgcqei: SCGCQEI,
    _reserved103: [u8; 0x10],
    #[doc = "0x758 - EEPROM Sleep Mode Clock Gating Control"]
    pub scgceeprom: SCGCEEPROM,
    #[doc = "0x75c - Wide Timer Sleep Mode Clock Gating Control"]
    pub scgcwtimer: SCGCWTIMER,
    _reserved105: [u8; 0xa0],
    #[doc = "0x800 - Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgcwd: DCGCWD,
    #[doc = "0x804 - Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgctimer: DCGCTIMER,
    #[doc = "0x808 - General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
    pub dcgcgpio: DCGCGPIO,
    #[doc = "0x80c - Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
    pub dcgcdma: DCGCDMA,
    _reserved109: [u8; 0x04],
    #[doc = "0x814 - Hibernation Deep-Sleep Mode Clock Gating Control"]
    pub dcgchib: DCGCHIB,
    #[doc = "0x818 - Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
    pub dcgcuart: DCGCUART,
    #[doc = "0x81c - Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
    pub dcgcssi: DCGCSSI,
    #[doc = "0x820 - Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
    pub dcgci2c: DCGCI2C,
    _reserved113: [u8; 0x04],
    #[doc = "0x828 - Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
    pub dcgcusb: DCGCUSB,
    _reserved114: [u8; 0x08],
    #[doc = "0x834 - Controller Area Network Deep-Sleep Mode Clock Gating Control"]
    pub dcgccan: DCGCCAN,
    #[doc = "0x838 - Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
    pub dcgcadc: DCGCADC,
    #[doc = "0x83c - Analog Comparator Deep-Sleep Mode Clock Gating Control"]
    pub dcgcacmp: DCGCACMP,
    #[doc = "0x840 - Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
    pub dcgcpwm: DCGCPWM,
    #[doc = "0x844 - Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
    pub dcgcqei: DCGCQEI,
    _reserved119: [u8; 0x10],
    #[doc = "0x858 - EEPROM Deep-Sleep Mode Clock Gating Control"]
    pub dcgceeprom: DCGCEEPROM,
    #[doc = "0x85c - Wide Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgcwtimer: DCGCWTIMER,
    _reserved121: [u8; 0x01a0],
    #[doc = "0xa00 - Watchdog Timer Peripheral Ready"]
    pub prwd: PRWD,
    #[doc = "0xa04 - Timer Peripheral Ready"]
    pub prtimer: PRTIMER,
    #[doc = "0xa08 - General-Purpose Input/Output Peripheral Ready"]
    pub prgpio: PRGPIO,
    #[doc = "0xa0c - Micro Direct Memory Access Peripheral Ready"]
    pub prdma: PRDMA,
    _reserved125: [u8; 0x04],
    #[doc = "0xa14 - Hibernation Peripheral Ready"]
    pub prhib: PRHIB,
    #[doc = "0xa18 - Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
    pub pruart: PRUART,
    #[doc = "0xa1c - Synchronous Serial Interface Peripheral Ready"]
    pub prssi: PRSSI,
    #[doc = "0xa20 - Inter-Integrated Circuit Peripheral Ready"]
    pub pri2c: PRI2C,
    _reserved129: [u8; 0x04],
    #[doc = "0xa28 - Universal Serial Bus Peripheral Ready"]
    pub prusb: PRUSB,
    _reserved130: [u8; 0x08],
    #[doc = "0xa34 - Controller Area Network Peripheral Ready"]
    pub prcan: PRCAN,
    #[doc = "0xa38 - Analog-to-Digital Converter Peripheral Ready"]
    pub pradc: PRADC,
    #[doc = "0xa3c - Analog Comparator Peripheral Ready"]
    pub pracmp: PRACMP,
    #[doc = "0xa40 - Pulse Width Modulator Peripheral Ready"]
    pub prpwm: PRPWM,
    #[doc = "0xa44 - Quadrature Encoder Interface Peripheral Ready"]
    pub prqei: PRQEI,
    _reserved135: [u8; 0x10],
    #[doc = "0xa58 - EEPROM Peripheral Ready"]
    pub preeprom: PREEPROM,
    #[doc = "0xa5c - Wide Timer Peripheral Ready"]
    pub prwtimer: PRWTIMER,
}
#[doc = "DID0 (rw) register accessor: Device Identification 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`did0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`did0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@did0`]
module"]
pub type DID0 = crate::Reg<did0::DID0_SPEC>;
#[doc = "Device Identification 0"]
pub mod did0;
#[doc = "DID1 (rw) register accessor: Device Identification 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`did1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`did1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@did1`]
module"]
pub type DID1 = crate::Reg<did1::DID1_SPEC>;
#[doc = "Device Identification 1"]
pub mod did1;
#[doc = "DC0 (rw) register accessor: Device Capabilities 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc0`]
module"]
pub type DC0 = crate::Reg<dc0::DC0_SPEC>;
#[doc = "Device Capabilities 0"]
pub mod dc0;
#[doc = "DC1 (rw) register accessor: Device Capabilities 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc1`]
module"]
pub type DC1 = crate::Reg<dc1::DC1_SPEC>;
#[doc = "Device Capabilities 1"]
pub mod dc1;
#[doc = "DC2 (rw) register accessor: Device Capabilities 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc2`]
module"]
pub type DC2 = crate::Reg<dc2::DC2_SPEC>;
#[doc = "Device Capabilities 2"]
pub mod dc2;
#[doc = "DC3 (rw) register accessor: Device Capabilities 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc3`]
module"]
pub type DC3 = crate::Reg<dc3::DC3_SPEC>;
#[doc = "Device Capabilities 3"]
pub mod dc3;
#[doc = "DC4 (rw) register accessor: Device Capabilities 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc4`]
module"]
pub type DC4 = crate::Reg<dc4::DC4_SPEC>;
#[doc = "Device Capabilities 4"]
pub mod dc4;
#[doc = "DC5 (rw) register accessor: Device Capabilities 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc5`]
module"]
pub type DC5 = crate::Reg<dc5::DC5_SPEC>;
#[doc = "Device Capabilities 5"]
pub mod dc5;
#[doc = "DC6 (rw) register accessor: Device Capabilities 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc6`]
module"]
pub type DC6 = crate::Reg<dc6::DC6_SPEC>;
#[doc = "Device Capabilities 6"]
pub mod dc6;
#[doc = "DC7 (rw) register accessor: Device Capabilities 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc7`]
module"]
pub type DC7 = crate::Reg<dc7::DC7_SPEC>;
#[doc = "Device Capabilities 7"]
pub mod dc7;
#[doc = "DC8 (rw) register accessor: Device Capabilities 8 ADC Channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc8`]
module"]
pub type DC8 = crate::Reg<dc8::DC8_SPEC>;
#[doc = "Device Capabilities 8 ADC Channels"]
pub mod dc8;
#[doc = "PBORCTL (rw) register accessor: Brown-Out Reset Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pborctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pborctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pborctl`]
module"]
pub type PBORCTL = crate::Reg<pborctl::PBORCTL_SPEC>;
#[doc = "Brown-Out Reset Control"]
pub mod pborctl;
#[doc = "SRCR0 (rw) register accessor: Software Reset Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcr0`]
module"]
pub type SRCR0 = crate::Reg<srcr0::SRCR0_SPEC>;
#[doc = "Software Reset Control 0"]
pub mod srcr0;
#[doc = "SRCR1 (rw) register accessor: Software Reset Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcr1`]
module"]
pub type SRCR1 = crate::Reg<srcr1::SRCR1_SPEC>;
#[doc = "Software Reset Control 1"]
pub mod srcr1;
#[doc = "SRCR2 (rw) register accessor: Software Reset Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcr2`]
module"]
pub type SRCR2 = crate::Reg<srcr2::SRCR2_SPEC>;
#[doc = "Software Reset Control 2"]
pub mod srcr2;
#[doc = "RIS (rw) register accessor: Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "IMC (rw) register accessor: Interrupt Mask Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imc`]
module"]
pub type IMC = crate::Reg<imc::IMC_SPEC>;
#[doc = "Interrupt Mask Control"]
pub mod imc;
#[doc = "MISC (rw) register accessor: Masked Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`]
module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "Masked Interrupt Status and Clear"]
pub mod misc;
#[doc = "RESC (rw) register accessor: Reset Cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resc`]
module"]
pub type RESC = crate::Reg<resc::RESC_SPEC>;
#[doc = "Reset Cause"]
pub mod resc;
#[doc = "RCC (rw) register accessor: Run-Mode Clock Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc`]
module"]
pub type RCC = crate::Reg<rcc::RCC_SPEC>;
#[doc = "Run-Mode Clock Configuration"]
pub mod rcc;
#[doc = "GPIOHBCTL (rw) register accessor: GPIO High-Performance Bus Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiohbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiohbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiohbctl`]
module"]
pub type GPIOHBCTL = crate::Reg<gpiohbctl::GPIOHBCTL_SPEC>;
#[doc = "GPIO High-Performance Bus Control"]
pub mod gpiohbctl;
#[doc = "RCC2 (rw) register accessor: Run-Mode Clock Configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc2`]
module"]
pub type RCC2 = crate::Reg<rcc2::RCC2_SPEC>;
#[doc = "Run-Mode Clock Configuration 2"]
pub mod rcc2;
#[doc = "MOSCCTL (rw) register accessor: Main Oscillator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moscctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moscctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moscctl`]
module"]
pub type MOSCCTL = crate::Reg<moscctl::MOSCCTL_SPEC>;
#[doc = "Main Oscillator Control"]
pub mod moscctl;
#[doc = "RCGC0 (rw) register accessor: Run Mode Clock Gating Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgc0`]
module"]
pub type RCGC0 = crate::Reg<rcgc0::RCGC0_SPEC>;
#[doc = "Run Mode Clock Gating Control Register 0"]
pub mod rcgc0;
#[doc = "RCGC1 (rw) register accessor: Run Mode Clock Gating Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgc1`]
module"]
pub type RCGC1 = crate::Reg<rcgc1::RCGC1_SPEC>;
#[doc = "Run Mode Clock Gating Control Register 1"]
pub mod rcgc1;
#[doc = "RCGC2 (rw) register accessor: Run Mode Clock Gating Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgc2`]
module"]
pub type RCGC2 = crate::Reg<rcgc2::RCGC2_SPEC>;
#[doc = "Run Mode Clock Gating Control Register 2"]
pub mod rcgc2;
#[doc = "SCGC0 (rw) register accessor: Sleep Mode Clock Gating Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc0`]
module"]
pub type SCGC0 = crate::Reg<scgc0::SCGC0_SPEC>;
#[doc = "Sleep Mode Clock Gating Control Register 0"]
pub mod scgc0;
#[doc = "SCGC1 (rw) register accessor: Sleep Mode Clock Gating Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc1`]
module"]
pub type SCGC1 = crate::Reg<scgc1::SCGC1_SPEC>;
#[doc = "Sleep Mode Clock Gating Control Register 1"]
pub mod scgc1;
#[doc = "SCGC2 (rw) register accessor: Sleep Mode Clock Gating Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc2`]
module"]
pub type SCGC2 = crate::Reg<scgc2::SCGC2_SPEC>;
#[doc = "Sleep Mode Clock Gating Control Register 2"]
pub mod scgc2;
#[doc = "DCGC0 (rw) register accessor: Deep Sleep Mode Clock Gating Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgc0`]
module"]
pub type DCGC0 = crate::Reg<dcgc0::DCGC0_SPEC>;
#[doc = "Deep Sleep Mode Clock Gating Control Register 0"]
pub mod dcgc0;
#[doc = "DCGC1 (rw) register accessor: Deep-Sleep Mode Clock Gating Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgc1`]
module"]
pub type DCGC1 = crate::Reg<dcgc1::DCGC1_SPEC>;
#[doc = "Deep-Sleep Mode Clock Gating Control Register 1"]
pub mod dcgc1;
#[doc = "DCGC2 (rw) register accessor: Deep Sleep Mode Clock Gating Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgc2`]
module"]
pub type DCGC2 = crate::Reg<dcgc2::DCGC2_SPEC>;
#[doc = "Deep Sleep Mode Clock Gating Control Register 2"]
pub mod dcgc2;
#[doc = "DSLPCLKCFG (rw) register accessor: Deep Sleep Clock Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dslpclkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dslpclkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dslpclkcfg`]
module"]
pub type DSLPCLKCFG = crate::Reg<dslpclkcfg::DSLPCLKCFG_SPEC>;
#[doc = "Deep Sleep Clock Configuration"]
pub mod dslpclkcfg;
#[doc = "SYSPROP (rw) register accessor: System Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysprop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysprop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysprop`]
module"]
pub type SYSPROP = crate::Reg<sysprop::SYSPROP_SPEC>;
#[doc = "System Properties"]
pub mod sysprop;
#[doc = "PIOSCCAL (rw) register accessor: Precision Internal Oscillator Calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`piosccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`piosccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@piosccal`]
module"]
pub type PIOSCCAL = crate::Reg<piosccal::PIOSCCAL_SPEC>;
#[doc = "Precision Internal Oscillator Calibration"]
pub mod piosccal;
#[doc = "PIOSCSTAT (rw) register accessor: Precision Internal Oscillator Statistics\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pioscstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pioscstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pioscstat`]
module"]
pub type PIOSCSTAT = crate::Reg<pioscstat::PIOSCSTAT_SPEC>;
#[doc = "Precision Internal Oscillator Statistics"]
pub mod pioscstat;
#[doc = "PLLFREQ0 (rw) register accessor: PLL Frequency 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllfreq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllfreq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllfreq0`]
module"]
pub type PLLFREQ0 = crate::Reg<pllfreq0::PLLFREQ0_SPEC>;
#[doc = "PLL Frequency 0"]
pub mod pllfreq0;
#[doc = "PLLFREQ1 (rw) register accessor: PLL Frequency 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllfreq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllfreq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllfreq1`]
module"]
pub type PLLFREQ1 = crate::Reg<pllfreq1::PLLFREQ1_SPEC>;
#[doc = "PLL Frequency 1"]
pub mod pllfreq1;
#[doc = "PLLSTAT (rw) register accessor: PLL Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstat`]
module"]
pub type PLLSTAT = crate::Reg<pllstat::PLLSTAT_SPEC>;
#[doc = "PLL Status"]
pub mod pllstat;
#[doc = "DC9 (rw) register accessor: Device Capabilities 9 ADC Digital Comparators\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc9`]
module"]
pub type DC9 = crate::Reg<dc9::DC9_SPEC>;
#[doc = "Device Capabilities 9 ADC Digital Comparators"]
pub mod dc9;
#[doc = "NVMSTAT (rw) register accessor: Non-Volatile Memory Information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvmstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvmstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvmstat`]
module"]
pub type NVMSTAT = crate::Reg<nvmstat::NVMSTAT_SPEC>;
#[doc = "Non-Volatile Memory Information"]
pub mod nvmstat;
#[doc = "PPWD (rw) register accessor: Watchdog Timer Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppwd`]
module"]
pub type PPWD = crate::Reg<ppwd::PPWD_SPEC>;
#[doc = "Watchdog Timer Peripheral Present"]
pub mod ppwd;
#[doc = "PPTIMER (rw) register accessor: Timer Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pptimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pptimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pptimer`]
module"]
pub type PPTIMER = crate::Reg<pptimer::PPTIMER_SPEC>;
#[doc = "Timer Peripheral Present"]
pub mod pptimer;
#[doc = "PPGPIO (rw) register accessor: General-Purpose Input/Output Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppgpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppgpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppgpio`]
module"]
pub type PPGPIO = crate::Reg<ppgpio::PPGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Peripheral Present"]
pub mod ppgpio;
#[doc = "PPDMA (rw) register accessor: Micro Direct Memory Access Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppdma`]
module"]
pub type PPDMA = crate::Reg<ppdma::PPDMA_SPEC>;
#[doc = "Micro Direct Memory Access Peripheral Present"]
pub mod ppdma;
#[doc = "PPHIB (rw) register accessor: Hibernation Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pphib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pphib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pphib`]
module"]
pub type PPHIB = crate::Reg<pphib::PPHIB_SPEC>;
#[doc = "Hibernation Peripheral Present"]
pub mod pphib;
#[doc = "PPUART (rw) register accessor: Universal Asynchronous Receiver/Transmitter Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppuart`]
module"]
pub type PPUART = crate::Reg<ppuart::PPUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Present"]
pub mod ppuart;
#[doc = "PPSSI (rw) register accessor: Synchronous Serial Interface Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppssi`]
module"]
pub type PPSSI = crate::Reg<ppssi::PPSSI_SPEC>;
#[doc = "Synchronous Serial Interface Peripheral Present"]
pub mod ppssi;
#[doc = "PPI2C (rw) register accessor: Inter-Integrated Circuit Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppi2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppi2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppi2c`]
module"]
pub type PPI2C = crate::Reg<ppi2c::PPI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Peripheral Present"]
pub mod ppi2c;
#[doc = "PPUSB (rw) register accessor: Universal Serial Bus Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppusb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppusb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppusb`]
module"]
pub type PPUSB = crate::Reg<ppusb::PPUSB_SPEC>;
#[doc = "Universal Serial Bus Peripheral Present"]
pub mod ppusb;
#[doc = "PPCAN (rw) register accessor: Controller Area Network Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppcan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppcan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppcan`]
module"]
pub type PPCAN = crate::Reg<ppcan::PPCAN_SPEC>;
#[doc = "Controller Area Network Peripheral Present"]
pub mod ppcan;
#[doc = "PPADC (rw) register accessor: Analog-to-Digital Converter Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppadc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppadc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppadc`]
module"]
pub type PPADC = crate::Reg<ppadc::PPADC_SPEC>;
#[doc = "Analog-to-Digital Converter Peripheral Present"]
pub mod ppadc;
#[doc = "PPACMP (rw) register accessor: Analog Comparator Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppacmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppacmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppacmp`]
module"]
pub type PPACMP = crate::Reg<ppacmp::PPACMP_SPEC>;
#[doc = "Analog Comparator Peripheral Present"]
pub mod ppacmp;
#[doc = "PPPWM (rw) register accessor: Pulse Width Modulator Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pppwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pppwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pppwm`]
module"]
pub type PPPWM = crate::Reg<pppwm::PPPWM_SPEC>;
#[doc = "Pulse Width Modulator Peripheral Present"]
pub mod pppwm;
#[doc = "PPQEI (rw) register accessor: Quadrature Encoder Interface Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppqei::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppqei::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppqei`]
module"]
pub type PPQEI = crate::Reg<ppqei::PPQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Peripheral Present"]
pub mod ppqei;
#[doc = "PPEEPROM (rw) register accessor: EEPROM Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppeeprom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppeeprom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppeeprom`]
module"]
pub type PPEEPROM = crate::Reg<ppeeprom::PPEEPROM_SPEC>;
#[doc = "EEPROM Peripheral Present"]
pub mod ppeeprom;
#[doc = "PPWTIMER (rw) register accessor: Wide Timer Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppwtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppwtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppwtimer`]
module"]
pub type PPWTIMER = crate::Reg<ppwtimer::PPWTIMER_SPEC>;
#[doc = "Wide Timer Peripheral Present"]
pub mod ppwtimer;
#[doc = "SRWD (rw) register accessor: Watchdog Timer Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srwd`]
module"]
pub type SRWD = crate::Reg<srwd::SRWD_SPEC>;
#[doc = "Watchdog Timer Software Reset"]
pub mod srwd;
#[doc = "SRTIMER (rw) register accessor: Timer Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srtimer`]
module"]
pub type SRTIMER = crate::Reg<srtimer::SRTIMER_SPEC>;
#[doc = "Timer Software Reset"]
pub mod srtimer;
#[doc = "SRGPIO (rw) register accessor: General-Purpose Input/Output Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srgpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srgpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srgpio`]
module"]
pub type SRGPIO = crate::Reg<srgpio::SRGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Software Reset"]
pub mod srgpio;
#[doc = "SRDMA (rw) register accessor: Micro Direct Memory Access Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srdma`]
module"]
pub type SRDMA = crate::Reg<srdma::SRDMA_SPEC>;
#[doc = "Micro Direct Memory Access Software Reset"]
pub mod srdma;
#[doc = "SRHIB (rw) register accessor: Hibernation Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srhib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srhib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srhib`]
module"]
pub type SRHIB = crate::Reg<srhib::SRHIB_SPEC>;
#[doc = "Hibernation Software Reset"]
pub mod srhib;
#[doc = "SRUART (rw) register accessor: Universal Asynchronous Receiver/Transmitter Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sruart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sruart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sruart`]
module"]
pub type SRUART = crate::Reg<sruart::SRUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Software Reset"]
pub mod sruart;
#[doc = "SRSSI (rw) register accessor: Synchronous Serial Interface Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srssi`]
module"]
pub type SRSSI = crate::Reg<srssi::SRSSI_SPEC>;
#[doc = "Synchronous Serial Interface Software Reset"]
pub mod srssi;
#[doc = "SRI2C (rw) register accessor: Inter-Integrated Circuit Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sri2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sri2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sri2c`]
module"]
pub type SRI2C = crate::Reg<sri2c::SRI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Software Reset"]
pub mod sri2c;
#[doc = "SRUSB (rw) register accessor: Universal Serial Bus Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srusb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srusb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srusb`]
module"]
pub type SRUSB = crate::Reg<srusb::SRUSB_SPEC>;
#[doc = "Universal Serial Bus Software Reset"]
pub mod srusb;
#[doc = "SRCAN (rw) register accessor: Controller Area Network Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcan`]
module"]
pub type SRCAN = crate::Reg<srcan::SRCAN_SPEC>;
#[doc = "Controller Area Network Software Reset"]
pub mod srcan;
#[doc = "SRADC (rw) register accessor: Analog-to-Digital Converter Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sradc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sradc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sradc`]
module"]
pub type SRADC = crate::Reg<sradc::SRADC_SPEC>;
#[doc = "Analog-to-Digital Converter Software Reset"]
pub mod sradc;
#[doc = "SRACMP (rw) register accessor: Analog Comparator Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sracmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sracmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sracmp`]
module"]
pub type SRACMP = crate::Reg<sracmp::SRACMP_SPEC>;
#[doc = "Analog Comparator Software Reset"]
pub mod sracmp;
#[doc = "SRPWM (rw) register accessor: Pulse Width Modulator Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srpwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srpwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srpwm`]
module"]
pub type SRPWM = crate::Reg<srpwm::SRPWM_SPEC>;
#[doc = "Pulse Width Modulator Software Reset"]
pub mod srpwm;
#[doc = "SRQEI (rw) register accessor: Quadrature Encoder Interface Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srqei::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srqei::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqei`]
module"]
pub type SRQEI = crate::Reg<srqei::SRQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Software Reset"]
pub mod srqei;
#[doc = "SREEPROM (rw) register accessor: EEPROM Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sreeprom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sreeprom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sreeprom`]
module"]
pub type SREEPROM = crate::Reg<sreeprom::SREEPROM_SPEC>;
#[doc = "EEPROM Software Reset"]
pub mod sreeprom;
#[doc = "SRWTIMER (rw) register accessor: Wide Timer Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srwtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srwtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srwtimer`]
module"]
pub type SRWTIMER = crate::Reg<srwtimer::SRWTIMER_SPEC>;
#[doc = "Wide Timer Software Reset"]
pub mod srwtimer;
#[doc = "RCGCWD (rw) register accessor: Watchdog Timer Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcwd`]
module"]
pub type RCGCWD = crate::Reg<rcgcwd::RCGCWD_SPEC>;
#[doc = "Watchdog Timer Run Mode Clock Gating Control"]
pub mod rcgcwd;
#[doc = "RCGCTIMER (rw) register accessor: Timer Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgctimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgctimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgctimer`]
module"]
pub type RCGCTIMER = crate::Reg<rcgctimer::RCGCTIMER_SPEC>;
#[doc = "Timer Run Mode Clock Gating Control"]
pub mod rcgctimer;
#[doc = "RCGCGPIO (rw) register accessor: General-Purpose Input/Output Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcgpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcgpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcgpio`]
module"]
pub type RCGCGPIO = crate::Reg<rcgcgpio::RCGCGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Run Mode Clock Gating Control"]
pub mod rcgcgpio;
#[doc = "RCGCDMA (rw) register accessor: Micro Direct Memory Access Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcdma`]
module"]
pub type RCGCDMA = crate::Reg<rcgcdma::RCGCDMA_SPEC>;
#[doc = "Micro Direct Memory Access Run Mode Clock Gating Control"]
pub mod rcgcdma;
#[doc = "RCGCHIB (rw) register accessor: Hibernation Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgchib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgchib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgchib`]
module"]
pub type RCGCHIB = crate::Reg<rcgchib::RCGCHIB_SPEC>;
#[doc = "Hibernation Run Mode Clock Gating Control"]
pub mod rcgchib;
#[doc = "RCGCUART (rw) register accessor: Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcuart`]
module"]
pub type RCGCUART = crate::Reg<rcgcuart::RCGCUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
pub mod rcgcuart;
#[doc = "RCGCSSI (rw) register accessor: Synchronous Serial Interface Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcssi`]
module"]
pub type RCGCSSI = crate::Reg<rcgcssi::RCGCSSI_SPEC>;
#[doc = "Synchronous Serial Interface Run Mode Clock Gating Control"]
pub mod rcgcssi;
#[doc = "RCGCI2C (rw) register accessor: Inter-Integrated Circuit Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgci2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgci2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgci2c`]
module"]
pub type RCGCI2C = crate::Reg<rcgci2c::RCGCI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Run Mode Clock Gating Control"]
pub mod rcgci2c;
#[doc = "RCGCUSB (rw) register accessor: Universal Serial Bus Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcusb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcusb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcusb`]
module"]
pub type RCGCUSB = crate::Reg<rcgcusb::RCGCUSB_SPEC>;
#[doc = "Universal Serial Bus Run Mode Clock Gating Control"]
pub mod rcgcusb;
#[doc = "RCGCCAN (rw) register accessor: Controller Area Network Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgccan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgccan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgccan`]
module"]
pub type RCGCCAN = crate::Reg<rcgccan::RCGCCAN_SPEC>;
#[doc = "Controller Area Network Run Mode Clock Gating Control"]
pub mod rcgccan;
#[doc = "RCGCADC (rw) register accessor: Analog-to-Digital Converter Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcadc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcadc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcadc`]
module"]
pub type RCGCADC = crate::Reg<rcgcadc::RCGCADC_SPEC>;
#[doc = "Analog-to-Digital Converter Run Mode Clock Gating Control"]
pub mod rcgcadc;
#[doc = "RCGCACMP (rw) register accessor: Analog Comparator Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcacmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcacmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcacmp`]
module"]
pub type RCGCACMP = crate::Reg<rcgcacmp::RCGCACMP_SPEC>;
#[doc = "Analog Comparator Run Mode Clock Gating Control"]
pub mod rcgcacmp;
#[doc = "RCGCPWM (rw) register accessor: Pulse Width Modulator Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcpwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcpwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcpwm`]
module"]
pub type RCGCPWM = crate::Reg<rcgcpwm::RCGCPWM_SPEC>;
#[doc = "Pulse Width Modulator Run Mode Clock Gating Control"]
pub mod rcgcpwm;
#[doc = "RCGCQEI (rw) register accessor: Quadrature Encoder Interface Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcqei::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcqei::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcqei`]
module"]
pub type RCGCQEI = crate::Reg<rcgcqei::RCGCQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Run Mode Clock Gating Control"]
pub mod rcgcqei;
#[doc = "RCGCEEPROM (rw) register accessor: EEPROM Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgceeprom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgceeprom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgceeprom`]
module"]
pub type RCGCEEPROM = crate::Reg<rcgceeprom::RCGCEEPROM_SPEC>;
#[doc = "EEPROM Run Mode Clock Gating Control"]
pub mod rcgceeprom;
#[doc = "RCGCWTIMER (rw) register accessor: Wide Timer Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcwtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcwtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcwtimer`]
module"]
pub type RCGCWTIMER = crate::Reg<rcgcwtimer::RCGCWTIMER_SPEC>;
#[doc = "Wide Timer Run Mode Clock Gating Control"]
pub mod rcgcwtimer;
#[doc = "SCGCWD (rw) register accessor: Watchdog Timer Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcwd`]
module"]
pub type SCGCWD = crate::Reg<scgcwd::SCGCWD_SPEC>;
#[doc = "Watchdog Timer Sleep Mode Clock Gating Control"]
pub mod scgcwd;
#[doc = "SCGCTIMER (rw) register accessor: Timer Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgctimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgctimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgctimer`]
module"]
pub type SCGCTIMER = crate::Reg<scgctimer::SCGCTIMER_SPEC>;
#[doc = "Timer Sleep Mode Clock Gating Control"]
pub mod scgctimer;
#[doc = "SCGCGPIO (rw) register accessor: General-Purpose Input/Output Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcgpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcgpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcgpio`]
module"]
pub type SCGCGPIO = crate::Reg<scgcgpio::SCGCGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Sleep Mode Clock Gating Control"]
pub mod scgcgpio;
#[doc = "SCGCDMA (rw) register accessor: Micro Direct Memory Access Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcdma`]
module"]
pub type SCGCDMA = crate::Reg<scgcdma::SCGCDMA_SPEC>;
#[doc = "Micro Direct Memory Access Sleep Mode Clock Gating Control"]
pub mod scgcdma;
#[doc = "SCGCHIB (rw) register accessor: Hibernation Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgchib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgchib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgchib`]
module"]
pub type SCGCHIB = crate::Reg<scgchib::SCGCHIB_SPEC>;
#[doc = "Hibernation Sleep Mode Clock Gating Control"]
pub mod scgchib;
#[doc = "SCGCUART (rw) register accessor: Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcuart`]
module"]
pub type SCGCUART = crate::Reg<scgcuart::SCGCUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
pub mod scgcuart;
#[doc = "SCGCSSI (rw) register accessor: Synchronous Serial Interface Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcssi`]
module"]
pub type SCGCSSI = crate::Reg<scgcssi::SCGCSSI_SPEC>;
#[doc = "Synchronous Serial Interface Sleep Mode Clock Gating Control"]
pub mod scgcssi;
#[doc = "SCGCI2C (rw) register accessor: Inter-Integrated Circuit Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgci2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgci2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgci2c`]
module"]
pub type SCGCI2C = crate::Reg<scgci2c::SCGCI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
pub mod scgci2c;
#[doc = "SCGCUSB (rw) register accessor: Universal Serial Bus Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcusb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcusb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcusb`]
module"]
pub type SCGCUSB = crate::Reg<scgcusb::SCGCUSB_SPEC>;
#[doc = "Universal Serial Bus Sleep Mode Clock Gating Control"]
pub mod scgcusb;
#[doc = "SCGCCAN (rw) register accessor: Controller Area Network Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgccan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgccan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgccan`]
module"]
pub type SCGCCAN = crate::Reg<scgccan::SCGCCAN_SPEC>;
#[doc = "Controller Area Network Sleep Mode Clock Gating Control"]
pub mod scgccan;
#[doc = "SCGCADC (rw) register accessor: Analog-to-Digital Converter Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcadc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcadc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcadc`]
module"]
pub type SCGCADC = crate::Reg<scgcadc::SCGCADC_SPEC>;
#[doc = "Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
pub mod scgcadc;
#[doc = "SCGCACMP (rw) register accessor: Analog Comparator Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcacmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcacmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcacmp`]
module"]
pub type SCGCACMP = crate::Reg<scgcacmp::SCGCACMP_SPEC>;
#[doc = "Analog Comparator Sleep Mode Clock Gating Control"]
pub mod scgcacmp;
#[doc = "SCGCPWM (rw) register accessor: Pulse Width Modulator Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcpwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcpwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcpwm`]
module"]
pub type SCGCPWM = crate::Reg<scgcpwm::SCGCPWM_SPEC>;
#[doc = "Pulse Width Modulator Sleep Mode Clock Gating Control"]
pub mod scgcpwm;
#[doc = "SCGCQEI (rw) register accessor: Quadrature Encoder Interface Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcqei::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcqei::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcqei`]
module"]
pub type SCGCQEI = crate::Reg<scgcqei::SCGCQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
pub mod scgcqei;
#[doc = "SCGCEEPROM (rw) register accessor: EEPROM Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgceeprom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgceeprom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgceeprom`]
module"]
pub type SCGCEEPROM = crate::Reg<scgceeprom::SCGCEEPROM_SPEC>;
#[doc = "EEPROM Sleep Mode Clock Gating Control"]
pub mod scgceeprom;
#[doc = "SCGCWTIMER (rw) register accessor: Wide Timer Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcwtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcwtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcwtimer`]
module"]
pub type SCGCWTIMER = crate::Reg<scgcwtimer::SCGCWTIMER_SPEC>;
#[doc = "Wide Timer Sleep Mode Clock Gating Control"]
pub mod scgcwtimer;
#[doc = "DCGCWD (rw) register accessor: Watchdog Timer Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcwd`]
module"]
pub type DCGCWD = crate::Reg<dcgcwd::DCGCWD_SPEC>;
#[doc = "Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcwd;
#[doc = "DCGCTIMER (rw) register accessor: Timer Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgctimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgctimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgctimer`]
module"]
pub type DCGCTIMER = crate::Reg<dcgctimer::DCGCTIMER_SPEC>;
#[doc = "Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgctimer;
#[doc = "DCGCGPIO (rw) register accessor: General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcgpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcgpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcgpio`]
module"]
pub type DCGCGPIO = crate::Reg<dcgcgpio::DCGCGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcgpio;
#[doc = "DCGCDMA (rw) register accessor: Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcdma`]
module"]
pub type DCGCDMA = crate::Reg<dcgcdma::DCGCDMA_SPEC>;
#[doc = "Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcdma;
#[doc = "DCGCHIB (rw) register accessor: Hibernation Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgchib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgchib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgchib`]
module"]
pub type DCGCHIB = crate::Reg<dcgchib::DCGCHIB_SPEC>;
#[doc = "Hibernation Deep-Sleep Mode Clock Gating Control"]
pub mod dcgchib;
#[doc = "DCGCUART (rw) register accessor: Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcuart`]
module"]
pub type DCGCUART = crate::Reg<dcgcuart::DCGCUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcuart;
#[doc = "DCGCSSI (rw) register accessor: Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcssi`]
module"]
pub type DCGCSSI = crate::Reg<dcgcssi::DCGCSSI_SPEC>;
#[doc = "Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcssi;
#[doc = "DCGCI2C (rw) register accessor: Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgci2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgci2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgci2c`]
module"]
pub type DCGCI2C = crate::Reg<dcgci2c::DCGCI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
pub mod dcgci2c;
#[doc = "DCGCUSB (rw) register accessor: Universal Serial Bus Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcusb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcusb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcusb`]
module"]
pub type DCGCUSB = crate::Reg<dcgcusb::DCGCUSB_SPEC>;
#[doc = "Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcusb;
#[doc = "DCGCCAN (rw) register accessor: Controller Area Network Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgccan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgccan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgccan`]
module"]
pub type DCGCCAN = crate::Reg<dcgccan::DCGCCAN_SPEC>;
#[doc = "Controller Area Network Deep-Sleep Mode Clock Gating Control"]
pub mod dcgccan;
#[doc = "DCGCADC (rw) register accessor: Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcadc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcadc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcadc`]
module"]
pub type DCGCADC = crate::Reg<dcgcadc::DCGCADC_SPEC>;
#[doc = "Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcadc;
#[doc = "DCGCACMP (rw) register accessor: Analog Comparator Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcacmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcacmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcacmp`]
module"]
pub type DCGCACMP = crate::Reg<dcgcacmp::DCGCACMP_SPEC>;
#[doc = "Analog Comparator Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcacmp;
#[doc = "DCGCPWM (rw) register accessor: Pulse Width Modulator Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcpwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcpwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcpwm`]
module"]
pub type DCGCPWM = crate::Reg<dcgcpwm::DCGCPWM_SPEC>;
#[doc = "Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcpwm;
#[doc = "DCGCQEI (rw) register accessor: Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcqei::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcqei::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcqei`]
module"]
pub type DCGCQEI = crate::Reg<dcgcqei::DCGCQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcqei;
#[doc = "DCGCEEPROM (rw) register accessor: EEPROM Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgceeprom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgceeprom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgceeprom`]
module"]
pub type DCGCEEPROM = crate::Reg<dcgceeprom::DCGCEEPROM_SPEC>;
#[doc = "EEPROM Deep-Sleep Mode Clock Gating Control"]
pub mod dcgceeprom;
#[doc = "DCGCWTIMER (rw) register accessor: Wide Timer Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcwtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcwtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcwtimer`]
module"]
pub type DCGCWTIMER = crate::Reg<dcgcwtimer::DCGCWTIMER_SPEC>;
#[doc = "Wide Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcwtimer;
#[doc = "PRWD (rw) register accessor: Watchdog Timer Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prwd`]
module"]
pub type PRWD = crate::Reg<prwd::PRWD_SPEC>;
#[doc = "Watchdog Timer Peripheral Ready"]
pub mod prwd;
#[doc = "PRTIMER (rw) register accessor: Timer Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prtimer`]
module"]
pub type PRTIMER = crate::Reg<prtimer::PRTIMER_SPEC>;
#[doc = "Timer Peripheral Ready"]
pub mod prtimer;
#[doc = "PRGPIO (rw) register accessor: General-Purpose Input/Output Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prgpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prgpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prgpio`]
module"]
pub type PRGPIO = crate::Reg<prgpio::PRGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Peripheral Ready"]
pub mod prgpio;
#[doc = "PRDMA (rw) register accessor: Micro Direct Memory Access Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prdma`]
module"]
pub type PRDMA = crate::Reg<prdma::PRDMA_SPEC>;
#[doc = "Micro Direct Memory Access Peripheral Ready"]
pub mod prdma;
#[doc = "PRHIB (rw) register accessor: Hibernation Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prhib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prhib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prhib`]
module"]
pub type PRHIB = crate::Reg<prhib::PRHIB_SPEC>;
#[doc = "Hibernation Peripheral Ready"]
pub mod prhib;
#[doc = "PRUART (rw) register accessor: Universal Asynchronous Receiver/Transmitter Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pruart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pruart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pruart`]
module"]
pub type PRUART = crate::Reg<pruart::PRUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
pub mod pruart;
#[doc = "PRSSI (rw) register accessor: Synchronous Serial Interface Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prssi`]
module"]
pub type PRSSI = crate::Reg<prssi::PRSSI_SPEC>;
#[doc = "Synchronous Serial Interface Peripheral Ready"]
pub mod prssi;
#[doc = "PRI2C (rw) register accessor: Inter-Integrated Circuit Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pri2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pri2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pri2c`]
module"]
pub type PRI2C = crate::Reg<pri2c::PRI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Peripheral Ready"]
pub mod pri2c;
#[doc = "PRUSB (rw) register accessor: Universal Serial Bus Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prusb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prusb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prusb`]
module"]
pub type PRUSB = crate::Reg<prusb::PRUSB_SPEC>;
#[doc = "Universal Serial Bus Peripheral Ready"]
pub mod prusb;
#[doc = "PRCAN (rw) register accessor: Controller Area Network Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prcan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prcan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prcan`]
module"]
pub type PRCAN = crate::Reg<prcan::PRCAN_SPEC>;
#[doc = "Controller Area Network Peripheral Ready"]
pub mod prcan;
#[doc = "PRADC (rw) register accessor: Analog-to-Digital Converter Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pradc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pradc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pradc`]
module"]
pub type PRADC = crate::Reg<pradc::PRADC_SPEC>;
#[doc = "Analog-to-Digital Converter Peripheral Ready"]
pub mod pradc;
#[doc = "PRACMP (rw) register accessor: Analog Comparator Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pracmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pracmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pracmp`]
module"]
pub type PRACMP = crate::Reg<pracmp::PRACMP_SPEC>;
#[doc = "Analog Comparator Peripheral Ready"]
pub mod pracmp;
#[doc = "PRPWM (rw) register accessor: Pulse Width Modulator Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prpwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prpwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prpwm`]
module"]
pub type PRPWM = crate::Reg<prpwm::PRPWM_SPEC>;
#[doc = "Pulse Width Modulator Peripheral Ready"]
pub mod prpwm;
#[doc = "PRQEI (rw) register accessor: Quadrature Encoder Interface Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prqei::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prqei::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prqei`]
module"]
pub type PRQEI = crate::Reg<prqei::PRQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Peripheral Ready"]
pub mod prqei;
#[doc = "PREEPROM (rw) register accessor: EEPROM Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preeprom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preeprom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preeprom`]
module"]
pub type PREEPROM = crate::Reg<preeprom::PREEPROM_SPEC>;
#[doc = "EEPROM Peripheral Ready"]
pub mod preeprom;
#[doc = "PRWTIMER (rw) register accessor: Wide Timer Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prwtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prwtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prwtimer`]
module"]
pub type PRWTIMER = crate::Reg<prwtimer::PRWTIMER_SPEC>;
#[doc = "Wide Timer Peripheral Ready"]
pub mod prwtimer;
