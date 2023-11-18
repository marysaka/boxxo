#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTM Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - GPTM Timer A Mode"]
    pub tamr: TAMR,
    #[doc = "0x08 - GPTM Timer B Mode"]
    pub tbmr: TBMR,
    #[doc = "0x0c - GPTM Control"]
    pub ctl: CTL,
    #[doc = "0x10 - GPTM Synchronize"]
    pub sync: SYNC,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - GPTM Interrupt Mask"]
    pub imr: IMR,
    #[doc = "0x1c - GPTM Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x20 - GPTM Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x24 - GPTM Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x28 - GPTM Timer A Interval Load"]
    pub tailr: TAILR,
    #[doc = "0x2c - GPTM Timer B Interval Load"]
    pub tbilr: TBILR,
    #[doc = "0x30 - GPTM Timer A Match"]
    pub tamatchr: TAMATCHR,
    #[doc = "0x34 - GPTM Timer B Match"]
    pub tbmatchr: TBMATCHR,
    #[doc = "0x38 - GPTM Timer A Prescale"]
    pub tapr: TAPR,
    #[doc = "0x3c - GPTM Timer B Prescale"]
    pub tbpr: TBPR,
    #[doc = "0x40 - GPTM TimerA Prescale Match"]
    pub tapmr: TAPMR,
    #[doc = "0x44 - GPTM TimerB Prescale Match"]
    pub tbpmr: TBPMR,
    #[doc = "0x48 - GPTM Timer A"]
    pub tar: TAR,
    #[doc = "0x4c - GPTM Timer B"]
    pub tbr: TBR,
    #[doc = "0x50 - GPTM Timer A Value"]
    pub tav: TAV,
    #[doc = "0x54 - GPTM Timer B Value"]
    pub tbv: TBV,
    #[doc = "0x58 - GPTM RTC Predivide"]
    pub rtcpd: RTCPD,
    #[doc = "0x5c - GPTM Timer A Prescale Snapshot"]
    pub taps: TAPS,
    #[doc = "0x60 - GPTM Timer B Prescale Snapshot"]
    pub tbps: TBPS,
    #[doc = "0x64 - GPTM Timer A Prescale Value"]
    pub tapv: TAPV,
    #[doc = "0x68 - GPTM Timer B Prescale Value"]
    pub tbpv: TBPV,
    _reserved26: [u8; 0x0f54],
    #[doc = "0xfc0 - GPTM Peripheral Properties"]
    pub pp: PP,
}
#[doc = "CFG (rw) register accessor: GPTM Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "GPTM Configuration"]
pub mod cfg;
#[doc = "TAMR (rw) register accessor: GPTM Timer A Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamr`]
module"]
pub type TAMR = crate::Reg<tamr::TAMR_SPEC>;
#[doc = "GPTM Timer A Mode"]
pub mod tamr;
#[doc = "TBMR (rw) register accessor: GPTM Timer B Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbmr`]
module"]
pub type TBMR = crate::Reg<tbmr::TBMR_SPEC>;
#[doc = "GPTM Timer B Mode"]
pub mod tbmr;
#[doc = "CTL (rw) register accessor: GPTM Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "GPTM Control"]
pub mod ctl;
#[doc = "SYNC (rw) register accessor: GPTM Synchronize\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`]
module"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "GPTM Synchronize"]
pub mod sync;
#[doc = "IMR (rw) register accessor: GPTM Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "GPTM Interrupt Mask"]
pub mod imr;
#[doc = "RIS (rw) register accessor: GPTM Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "GPTM Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS (rw) register accessor: GPTM Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "GPTM Masked Interrupt Status"]
pub mod mis;
#[doc = "ICR (w) register accessor: GPTM Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "GPTM Interrupt Clear"]
pub mod icr;
#[doc = "TAILR (rw) register accessor: GPTM Timer A Interval Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tailr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tailr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tailr`]
module"]
pub type TAILR = crate::Reg<tailr::TAILR_SPEC>;
#[doc = "GPTM Timer A Interval Load"]
pub mod tailr;
#[doc = "TBILR (rw) register accessor: GPTM Timer B Interval Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbilr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbilr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbilr`]
module"]
pub type TBILR = crate::Reg<tbilr::TBILR_SPEC>;
#[doc = "GPTM Timer B Interval Load"]
pub mod tbilr;
#[doc = "TAMATCHR (rw) register accessor: GPTM Timer A Match\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamatchr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamatchr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamatchr`]
module"]
pub type TAMATCHR = crate::Reg<tamatchr::TAMATCHR_SPEC>;
#[doc = "GPTM Timer A Match"]
pub mod tamatchr;
#[doc = "TBMATCHR (rw) register accessor: GPTM Timer B Match\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbmatchr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbmatchr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbmatchr`]
module"]
pub type TBMATCHR = crate::Reg<tbmatchr::TBMATCHR_SPEC>;
#[doc = "GPTM Timer B Match"]
pub mod tbmatchr;
#[doc = "TAPR (rw) register accessor: GPTM Timer A Prescale\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tapr`]
module"]
pub type TAPR = crate::Reg<tapr::TAPR_SPEC>;
#[doc = "GPTM Timer A Prescale"]
pub mod tapr;
#[doc = "TBPR (rw) register accessor: GPTM Timer B Prescale\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbpr`]
module"]
pub type TBPR = crate::Reg<tbpr::TBPR_SPEC>;
#[doc = "GPTM Timer B Prescale"]
pub mod tbpr;
#[doc = "TAPMR (rw) register accessor: GPTM TimerA Prescale Match\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tapmr`]
module"]
pub type TAPMR = crate::Reg<tapmr::TAPMR_SPEC>;
#[doc = "GPTM TimerA Prescale Match"]
pub mod tapmr;
#[doc = "TBPMR (rw) register accessor: GPTM TimerB Prescale Match\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbpmr`]
module"]
pub type TBPMR = crate::Reg<tbpmr::TBPMR_SPEC>;
#[doc = "GPTM TimerB Prescale Match"]
pub mod tbpmr;
#[doc = "TAR (rw) register accessor: GPTM Timer A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`]
module"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "GPTM Timer A"]
pub mod tar;
#[doc = "TBR (rw) register accessor: GPTM Timer B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbr`]
module"]
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
#[doc = "GPTM Timer B"]
pub mod tbr;
#[doc = "TAV (rw) register accessor: GPTM Timer A Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tav::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tav::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tav`]
module"]
pub type TAV = crate::Reg<tav::TAV_SPEC>;
#[doc = "GPTM Timer A Value"]
pub mod tav;
#[doc = "TBV (rw) register accessor: GPTM Timer B Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbv`]
module"]
pub type TBV = crate::Reg<tbv::TBV_SPEC>;
#[doc = "GPTM Timer B Value"]
pub mod tbv;
#[doc = "RTCPD (rw) register accessor: GPTM RTC Predivide\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcpd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcpd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcpd`]
module"]
pub type RTCPD = crate::Reg<rtcpd::RTCPD_SPEC>;
#[doc = "GPTM RTC Predivide"]
pub mod rtcpd;
#[doc = "TAPS (rw) register accessor: GPTM Timer A Prescale Snapshot\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taps`]
module"]
pub type TAPS = crate::Reg<taps::TAPS_SPEC>;
#[doc = "GPTM Timer A Prescale Snapshot"]
pub mod taps;
#[doc = "TBPS (rw) register accessor: GPTM Timer B Prescale Snapshot\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbps`]
module"]
pub type TBPS = crate::Reg<tbps::TBPS_SPEC>;
#[doc = "GPTM Timer B Prescale Snapshot"]
pub mod tbps;
#[doc = "TAPV (rw) register accessor: GPTM Timer A Prescale Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tapv`]
module"]
pub type TAPV = crate::Reg<tapv::TAPV_SPEC>;
#[doc = "GPTM Timer A Prescale Value"]
pub mod tapv;
#[doc = "TBPV (rw) register accessor: GPTM Timer B Prescale Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbpv`]
module"]
pub type TBPV = crate::Reg<tbpv::TBPV_SPEC>;
#[doc = "GPTM Timer B Prescale Value"]
pub mod tbpv;
#[doc = "PP (rw) register accessor: GPTM Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp`]
module"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "GPTM Peripheral Properties"]
pub mod pp;
