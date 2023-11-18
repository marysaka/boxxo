#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hibernation RTC Counter"]
    pub rtcc: RTCC,
    #[doc = "0x04 - Hibernation RTC Match 0"]
    pub rtcm0: RTCM0,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Hibernation RTC Load"]
    pub rtcld: RTCLD,
    #[doc = "0x10 - Hibernation Control"]
    pub ctl: CTL,
    #[doc = "0x14 - Hibernation Interrupt Mask"]
    pub im: IM,
    #[doc = "0x18 - Hibernation Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x1c - Hibernation Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x20 - Hibernation Interrupt Clear"]
    pub ic: IC,
    #[doc = "0x24 - Hibernation RTC Trim"]
    pub rtct: RTCT,
    #[doc = "0x28 - Hibernation RTC Sub Seconds"]
    pub rtcss: RTCSS,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Hibernation Data"]
    pub data: DATA,
}
#[doc = "RTCC (rw) register accessor: Hibernation RTC Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcc`]
module"]
pub type RTCC = crate::Reg<rtcc::RTCC_SPEC>;
#[doc = "Hibernation RTC Counter"]
pub mod rtcc;
#[doc = "RTCM0 (rw) register accessor: Hibernation RTC Match 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcm0`]
module"]
pub type RTCM0 = crate::Reg<rtcm0::RTCM0_SPEC>;
#[doc = "Hibernation RTC Match 0"]
pub mod rtcm0;
#[doc = "RTCLD (rw) register accessor: Hibernation RTC Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcld`]
module"]
pub type RTCLD = crate::Reg<rtcld::RTCLD_SPEC>;
#[doc = "Hibernation RTC Load"]
pub mod rtcld;
#[doc = "CTL (rw) register accessor: Hibernation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Hibernation Control"]
pub mod ctl;
#[doc = "IM (rw) register accessor: Hibernation Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`]
module"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "Hibernation Interrupt Mask"]
pub mod im;
#[doc = "RIS (rw) register accessor: Hibernation Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Hibernation Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS (rw) register accessor: Hibernation Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Hibernation Masked Interrupt Status"]
pub mod mis;
#[doc = "IC (rw) register accessor: Hibernation Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ic::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ic`]
module"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "Hibernation Interrupt Clear"]
pub mod ic;
#[doc = "RTCT (rw) register accessor: Hibernation RTC Trim\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtct`]
module"]
pub type RTCT = crate::Reg<rtct::RTCT_SPEC>;
#[doc = "Hibernation RTC Trim"]
pub mod rtct;
#[doc = "RTCSS (rw) register accessor: Hibernation RTC Sub Seconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcss`]
module"]
pub type RTCSS = crate::Reg<rtcss::RTCSS_SPEC>;
#[doc = "Hibernation RTC Sub Seconds"]
pub mod rtcss;
#[doc = "DATA (rw) register accessor: Hibernation Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Hibernation Data"]
pub mod data;
