#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Exception Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x04 - System Exception Interrupt Mask"]
    pub im: IM,
    #[doc = "0x08 - System Exception Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x0c - System Exception Interrupt Clear"]
    pub ic: IC,
}
#[doc = "RIS (rw) register accessor: System Exception Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "System Exception Raw Interrupt Status"]
pub mod ris;
#[doc = "IM (rw) register accessor: System Exception Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`]
module"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "System Exception Interrupt Mask"]
pub mod im;
#[doc = "MIS (rw) register accessor: System Exception Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "System Exception Masked Interrupt Status"]
pub mod mis;
#[doc = "IC (w) register accessor: System Exception Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ic`]
module"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "System Exception Interrupt Clear"]
pub mod ic;
