#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Data"]
    pub dr: DR,
    _reserved_1_rsr: [u8; 0x04],
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - UART Flag"]
    pub fr: FR,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - UART IrDA Low-Power Register"]
    pub ilpr: ILPR,
    #[doc = "0x24 - UART Integer Baud-Rate Divisor"]
    pub ibrd: IBRD,
    #[doc = "0x28 - UART Fractional Baud-Rate Divisor"]
    pub fbrd: FBRD,
    #[doc = "0x2c - UART Line Control"]
    pub lcrh: LCRH,
    #[doc = "0x30 - UART Control"]
    pub ctl: CTL,
    #[doc = "0x34 - UART Interrupt FIFO Level Select"]
    pub ifls: IFLS,
    #[doc = "0x38 - UART Interrupt Mask"]
    pub im: IM,
    #[doc = "0x3c - UART Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x40 - UART Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x44 - UART Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x48 - UART DMA Control"]
    pub dmactl: DMACTL,
    _reserved14: [u8; 0x58],
    #[doc = "0xa4 - UART 9-Bit Self Address"]
    pub _9bitaddr: _9BITADDR,
    #[doc = "0xa8 - UART 9-Bit Self Address Mask"]
    pub _9bitamask: _9BITAMASK,
    _reserved16: [u8; 0x0f14],
    #[doc = "0xfc0 - UART Peripheral Properties"]
    pub pp: PP,
    _reserved17: [u8; 0x04],
    #[doc = "0xfc8 - UART Clock Configuration"]
    pub cc: CC,
}
impl RegisterBlock {
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub const fn uart_alt_ecr(&self) -> &UART_ALT_ECR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "DR (rw) register accessor: UART Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "UART Data"]
pub mod dr;
#[doc = "RSR (rw) register accessor: UART Receive Status/Error Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "UART Receive Status/Error Clear"]
pub mod rsr;
#[doc = "UART_ALT_ECR (rw) register accessor: UART Receive Status/Error Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_alt_ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_alt_ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_alt_ecr`]
module"]
pub type UART_ALT_ECR = crate::Reg<uart_alt_ecr::UART_ALT_ECR_SPEC>;
#[doc = "UART Receive Status/Error Clear"]
pub mod uart_alt_ecr;
#[doc = "FR (rw) register accessor: UART Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`]
module"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "UART Flag"]
pub mod fr;
#[doc = "ILPR (rw) register accessor: UART IrDA Low-Power Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ilpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ilpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ilpr`]
module"]
pub type ILPR = crate::Reg<ilpr::ILPR_SPEC>;
#[doc = "UART IrDA Low-Power Register"]
pub mod ilpr;
#[doc = "IBRD (rw) register accessor: UART Integer Baud-Rate Divisor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibrd`]
module"]
pub type IBRD = crate::Reg<ibrd::IBRD_SPEC>;
#[doc = "UART Integer Baud-Rate Divisor"]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: UART Fractional Baud-Rate Divisor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbrd`]
module"]
pub type FBRD = crate::Reg<fbrd::FBRD_SPEC>;
#[doc = "UART Fractional Baud-Rate Divisor"]
pub mod fbrd;
#[doc = "LCRH (rw) register accessor: UART Line Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcrh`]
module"]
pub type LCRH = crate::Reg<lcrh::LCRH_SPEC>;
#[doc = "UART Line Control"]
pub mod lcrh;
#[doc = "CTL (rw) register accessor: UART Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "UART Control"]
pub mod ctl;
#[doc = "IFLS (rw) register accessor: UART Interrupt FIFO Level Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`]
module"]
pub type IFLS = crate::Reg<ifls::IFLS_SPEC>;
#[doc = "UART Interrupt FIFO Level Select"]
pub mod ifls;
#[doc = "IM (rw) register accessor: UART Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`]
module"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "UART Interrupt Mask"]
pub mod im;
#[doc = "RIS (rw) register accessor: UART Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "UART Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS (rw) register accessor: UART Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "UART Masked Interrupt Status"]
pub mod mis;
#[doc = "ICR (w) register accessor: UART Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "UART Interrupt Clear"]
pub mod icr;
#[doc = "DMACTL (rw) register accessor: UART DMA Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl`]
module"]
pub type DMACTL = crate::Reg<dmactl::DMACTL_SPEC>;
#[doc = "UART DMA Control"]
pub mod dmactl;
#[doc = "_9BITADDR (rw) register accessor: UART 9-Bit Self Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_9bitaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_9bitaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9bitaddr`]
module"]
pub type _9BITADDR = crate::Reg<_9bitaddr::_9BITADDR_SPEC>;
#[doc = "UART 9-Bit Self Address"]
pub mod _9bitaddr;
#[doc = "_9BITAMASK (rw) register accessor: UART 9-Bit Self Address Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_9bitamask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_9bitamask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9bitamask`]
module"]
pub type _9BITAMASK = crate::Reg<_9bitamask::_9BITAMASK_SPEC>;
#[doc = "UART 9-Bit Self Address Mask"]
pub mod _9bitamask;
#[doc = "PP (rw) register accessor: UART Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp`]
module"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "UART Peripheral Properties"]
pub mod pp;
#[doc = "CC (rw) register accessor: UART Clock Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`]
module"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "UART Clock Configuration"]
pub mod cc;
