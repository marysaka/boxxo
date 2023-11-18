#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Master Control"]
    pub ctl: CTL,
    #[doc = "0x04 - PWM Time Base Sync"]
    pub sync: SYNC,
    #[doc = "0x08 - PWM Output Enable"]
    pub enable: ENABLE,
    #[doc = "0x0c - PWM Output Inversion"]
    pub invert: INVERT,
    #[doc = "0x10 - PWM Output Fault"]
    pub fault: FAULT,
    #[doc = "0x14 - PWM Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x18 - PWM Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x1c - PWM Interrupt Status and Clear"]
    pub isc: ISC,
    #[doc = "0x20 - PWM Status"]
    pub status: STATUS,
    #[doc = "0x24 - PWM Fault Condition Value"]
    pub faultval: FAULTVAL,
    #[doc = "0x28 - PWM Enable Update"]
    pub enupd: ENUPD,
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - PWM0 Control"]
    pub _0_ctl: _0_CTL,
    #[doc = "0x44 - PWM0 Interrupt and Trigger Enable"]
    pub _0_inten: _0_INTEN,
    #[doc = "0x48 - PWM0 Raw Interrupt Status"]
    pub _0_ris: _0_RIS,
    #[doc = "0x4c - PWM0 Interrupt Status and Clear"]
    pub _0_isc: _0_ISC,
    #[doc = "0x50 - PWM0 Load"]
    pub _0_load: _0_LOAD,
    #[doc = "0x54 - PWM0 Counter"]
    pub _0_count: _0_COUNT,
    #[doc = "0x58 - PWM0 Compare A"]
    pub _0_cmpa: _0_CMPA,
    #[doc = "0x5c - PWM0 Compare B"]
    pub _0_cmpb: _0_CMPB,
    #[doc = "0x60 - PWM0 Generator A Control"]
    pub _0_gena: _0_GENA,
    #[doc = "0x64 - PWM0 Generator B Control"]
    pub _0_genb: _0_GENB,
    #[doc = "0x68 - PWM0 Dead-Band Control"]
    pub _0_dbctl: _0_DBCTL,
    #[doc = "0x6c - PWM0 Dead-Band Rising-Edge Delay"]
    pub _0_dbrise: _0_DBRISE,
    #[doc = "0x70 - PWM0 Dead-Band Falling-Edge-Delay"]
    pub _0_dbfall: _0_DBFALL,
    #[doc = "0x74 - PWM0 Fault Source 0"]
    pub _0_fltsrc0: _0_FLTSRC0,
    #[doc = "0x78 - PWM0 Fault Source 1"]
    pub _0_fltsrc1: _0_FLTSRC1,
    #[doc = "0x7c - PWM0 Minimum Fault Period"]
    pub _0_minfltper: _0_MINFLTPER,
    #[doc = "0x80 - PWM1 Control"]
    pub _1_ctl: _1_CTL,
    #[doc = "0x84 - PWM1 Interrupt and Trigger Enable"]
    pub _1_inten: _1_INTEN,
    #[doc = "0x88 - PWM1 Raw Interrupt Status"]
    pub _1_ris: _1_RIS,
    #[doc = "0x8c - PWM1 Interrupt Status and Clear"]
    pub _1_isc: _1_ISC,
    #[doc = "0x90 - PWM1 Load"]
    pub _1_load: _1_LOAD,
    #[doc = "0x94 - PWM1 Counter"]
    pub _1_count: _1_COUNT,
    #[doc = "0x98 - PWM1 Compare A"]
    pub _1_cmpa: _1_CMPA,
    #[doc = "0x9c - PWM1 Compare B"]
    pub _1_cmpb: _1_CMPB,
    #[doc = "0xa0 - PWM1 Generator A Control"]
    pub _1_gena: _1_GENA,
    #[doc = "0xa4 - PWM1 Generator B Control"]
    pub _1_genb: _1_GENB,
    #[doc = "0xa8 - PWM1 Dead-Band Control"]
    pub _1_dbctl: _1_DBCTL,
    #[doc = "0xac - PWM1 Dead-Band Rising-Edge Delay"]
    pub _1_dbrise: _1_DBRISE,
    #[doc = "0xb0 - PWM1 Dead-Band Falling-Edge-Delay"]
    pub _1_dbfall: _1_DBFALL,
    #[doc = "0xb4 - PWM1 Fault Source 0"]
    pub _1_fltsrc0: _1_FLTSRC0,
    #[doc = "0xb8 - PWM1 Fault Source 1"]
    pub _1_fltsrc1: _1_FLTSRC1,
    #[doc = "0xbc - PWM1 Minimum Fault Period"]
    pub _1_minfltper: _1_MINFLTPER,
    #[doc = "0xc0 - PWM2 Control"]
    pub _2_ctl: _2_CTL,
    #[doc = "0xc4 - PWM2 Interrupt and Trigger Enable"]
    pub _2_inten: _2_INTEN,
    #[doc = "0xc8 - PWM2 Raw Interrupt Status"]
    pub _2_ris: _2_RIS,
    #[doc = "0xcc - PWM2 Interrupt Status and Clear"]
    pub _2_isc: _2_ISC,
    #[doc = "0xd0 - PWM2 Load"]
    pub _2_load: _2_LOAD,
    #[doc = "0xd4 - PWM2 Counter"]
    pub _2_count: _2_COUNT,
    #[doc = "0xd8 - PWM2 Compare A"]
    pub _2_cmpa: _2_CMPA,
    #[doc = "0xdc - PWM2 Compare B"]
    pub _2_cmpb: _2_CMPB,
    #[doc = "0xe0 - PWM2 Generator A Control"]
    pub _2_gena: _2_GENA,
    #[doc = "0xe4 - PWM2 Generator B Control"]
    pub _2_genb: _2_GENB,
    #[doc = "0xe8 - PWM2 Dead-Band Control"]
    pub _2_dbctl: _2_DBCTL,
    #[doc = "0xec - PWM2 Dead-Band Rising-Edge Delay"]
    pub _2_dbrise: _2_DBRISE,
    #[doc = "0xf0 - PWM2 Dead-Band Falling-Edge-Delay"]
    pub _2_dbfall: _2_DBFALL,
    #[doc = "0xf4 - PWM2 Fault Source 0"]
    pub _2_fltsrc0: _2_FLTSRC0,
    #[doc = "0xf8 - PWM2 Fault Source 1"]
    pub _2_fltsrc1: _2_FLTSRC1,
    #[doc = "0xfc - PWM2 Minimum Fault Period"]
    pub _2_minfltper: _2_MINFLTPER,
    #[doc = "0x100 - PWM3 Control"]
    pub _3_ctl: _3_CTL,
    #[doc = "0x104 - PWM3 Interrupt and Trigger Enable"]
    pub _3_inten: _3_INTEN,
    #[doc = "0x108 - PWM3 Raw Interrupt Status"]
    pub _3_ris: _3_RIS,
    #[doc = "0x10c - PWM3 Interrupt Status and Clear"]
    pub _3_isc: _3_ISC,
    #[doc = "0x110 - PWM3 Load"]
    pub _3_load: _3_LOAD,
    #[doc = "0x114 - PWM3 Counter"]
    pub _3_count: _3_COUNT,
    #[doc = "0x118 - PWM3 Compare A"]
    pub _3_cmpa: _3_CMPA,
    #[doc = "0x11c - PWM3 Compare B"]
    pub _3_cmpb: _3_CMPB,
    #[doc = "0x120 - PWM3 Generator A Control"]
    pub _3_gena: _3_GENA,
    #[doc = "0x124 - PWM3 Generator B Control"]
    pub _3_genb: _3_GENB,
    #[doc = "0x128 - PWM3 Dead-Band Control"]
    pub _3_dbctl: _3_DBCTL,
    #[doc = "0x12c - PWM3 Dead-Band Rising-Edge Delay"]
    pub _3_dbrise: _3_DBRISE,
    #[doc = "0x130 - PWM3 Dead-Band Falling-Edge-Delay"]
    pub _3_dbfall: _3_DBFALL,
    #[doc = "0x134 - PWM3 Fault Source 0"]
    pub _3_fltsrc0: _3_FLTSRC0,
    #[doc = "0x138 - PWM3 Fault Source 1"]
    pub _3_fltsrc1: _3_FLTSRC1,
    #[doc = "0x13c - PWM3 Minimum Fault Period"]
    pub _3_minfltper: _3_MINFLTPER,
    _reserved75: [u8; 0x06c0],
    #[doc = "0x800 - PWM0 Fault Pin Logic Sense"]
    pub _0_fltsen: _0_FLTSEN,
    #[doc = "0x804 - PWM0 Fault Status 0"]
    pub _0_fltstat0: _0_FLTSTAT0,
    #[doc = "0x808 - PWM0 Fault Status 1"]
    pub _0_fltstat1: _0_FLTSTAT1,
    _reserved78: [u8; 0x74],
    #[doc = "0x880 - PWM1 Fault Pin Logic Sense"]
    pub _1_fltsen: _1_FLTSEN,
    #[doc = "0x884 - PWM1 Fault Status 0"]
    pub _1_fltstat0: _1_FLTSTAT0,
    #[doc = "0x888 - PWM1 Fault Status 1"]
    pub _1_fltstat1: _1_FLTSTAT1,
    _reserved81: [u8; 0x78],
    #[doc = "0x904 - PWM2 Fault Status 0"]
    pub _2_fltstat0: _2_FLTSTAT0,
    #[doc = "0x908 - PWM2 Fault Status 1"]
    pub _2_fltstat1: _2_FLTSTAT1,
    _reserved83: [u8; 0x78],
    #[doc = "0x984 - PWM3 Fault Status 0"]
    pub _3_fltstat0: _3_FLTSTAT0,
    #[doc = "0x988 - PWM3 Fault Status 1"]
    pub _3_fltstat1: _3_FLTSTAT1,
    _reserved85: [u8; 0x0634],
    #[doc = "0xfc0 - PWM Peripheral Properties"]
    pub pp: PP,
}
#[doc = "CTL (rw) register accessor: PWM Master Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "PWM Master Control"]
pub mod ctl;
#[doc = "SYNC (rw) register accessor: PWM Time Base Sync\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`]
module"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "PWM Time Base Sync"]
pub mod sync;
#[doc = "ENABLE (rw) register accessor: PWM Output Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "PWM Output Enable"]
pub mod enable;
#[doc = "INVERT (rw) register accessor: PWM Output Inversion\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`invert::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`invert::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@invert`]
module"]
pub type INVERT = crate::Reg<invert::INVERT_SPEC>;
#[doc = "PWM Output Inversion"]
pub mod invert;
#[doc = "FAULT (rw) register accessor: PWM Output Fault\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault`]
module"]
pub type FAULT = crate::Reg<fault::FAULT_SPEC>;
#[doc = "PWM Output Fault"]
pub mod fault;
#[doc = "INTEN (rw) register accessor: PWM Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "PWM Interrupt Enable"]
pub mod inten;
#[doc = "RIS (rw) register accessor: PWM Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "PWM Raw Interrupt Status"]
pub mod ris;
#[doc = "ISC (rw) register accessor: PWM Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc`]
module"]
pub type ISC = crate::Reg<isc::ISC_SPEC>;
#[doc = "PWM Interrupt Status and Clear"]
pub mod isc;
#[doc = "STATUS (rw) register accessor: PWM Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "PWM Status"]
pub mod status;
#[doc = "FAULTVAL (rw) register accessor: PWM Fault Condition Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faultval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faultval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faultval`]
module"]
pub type FAULTVAL = crate::Reg<faultval::FAULTVAL_SPEC>;
#[doc = "PWM Fault Condition Value"]
pub mod faultval;
#[doc = "ENUPD (rw) register accessor: PWM Enable Update\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enupd`]
module"]
pub type ENUPD = crate::Reg<enupd::ENUPD_SPEC>;
#[doc = "PWM Enable Update"]
pub mod enupd;
#[doc = "_0_CTL (rw) register accessor: PWM0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_ctl`]
module"]
pub type _0_CTL = crate::Reg<_0_ctl::_0_CTL_SPEC>;
#[doc = "PWM0 Control"]
pub mod _0_ctl;
#[doc = "_0_INTEN (rw) register accessor: PWM0 Interrupt and Trigger Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_inten`]
module"]
pub type _0_INTEN = crate::Reg<_0_inten::_0_INTEN_SPEC>;
#[doc = "PWM0 Interrupt and Trigger Enable"]
pub mod _0_inten;
#[doc = "_0_RIS (rw) register accessor: PWM0 Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_ris`]
module"]
pub type _0_RIS = crate::Reg<_0_ris::_0_RIS_SPEC>;
#[doc = "PWM0 Raw Interrupt Status"]
pub mod _0_ris;
#[doc = "_0_ISC (rw) register accessor: PWM0 Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_isc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_isc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_isc`]
module"]
pub type _0_ISC = crate::Reg<_0_isc::_0_ISC_SPEC>;
#[doc = "PWM0 Interrupt Status and Clear"]
pub mod _0_isc;
#[doc = "_0_LOAD (rw) register accessor: PWM0 Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_load`]
module"]
pub type _0_LOAD = crate::Reg<_0_load::_0_LOAD_SPEC>;
#[doc = "PWM0 Load"]
pub mod _0_load;
#[doc = "_0_COUNT (rw) register accessor: PWM0 Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_count`]
module"]
pub type _0_COUNT = crate::Reg<_0_count::_0_COUNT_SPEC>;
#[doc = "PWM0 Counter"]
pub mod _0_count;
#[doc = "_0_CMPA (rw) register accessor: PWM0 Compare A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_cmpa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_cmpa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_cmpa`]
module"]
pub type _0_CMPA = crate::Reg<_0_cmpa::_0_CMPA_SPEC>;
#[doc = "PWM0 Compare A"]
pub mod _0_cmpa;
#[doc = "_0_CMPB (rw) register accessor: PWM0 Compare B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_cmpb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_cmpb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_cmpb`]
module"]
pub type _0_CMPB = crate::Reg<_0_cmpb::_0_CMPB_SPEC>;
#[doc = "PWM0 Compare B"]
pub mod _0_cmpb;
#[doc = "_0_GENA (rw) register accessor: PWM0 Generator A Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_gena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_gena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_gena`]
module"]
pub type _0_GENA = crate::Reg<_0_gena::_0_GENA_SPEC>;
#[doc = "PWM0 Generator A Control"]
pub mod _0_gena;
#[doc = "_0_GENB (rw) register accessor: PWM0 Generator B Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_genb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_genb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_genb`]
module"]
pub type _0_GENB = crate::Reg<_0_genb::_0_GENB_SPEC>;
#[doc = "PWM0 Generator B Control"]
pub mod _0_genb;
#[doc = "_0_DBCTL (rw) register accessor: PWM0 Dead-Band Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_dbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_dbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_dbctl`]
module"]
pub type _0_DBCTL = crate::Reg<_0_dbctl::_0_DBCTL_SPEC>;
#[doc = "PWM0 Dead-Band Control"]
pub mod _0_dbctl;
#[doc = "_0_DBRISE (rw) register accessor: PWM0 Dead-Band Rising-Edge Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_dbrise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_dbrise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_dbrise`]
module"]
pub type _0_DBRISE = crate::Reg<_0_dbrise::_0_DBRISE_SPEC>;
#[doc = "PWM0 Dead-Band Rising-Edge Delay"]
pub mod _0_dbrise;
#[doc = "_0_DBFALL (rw) register accessor: PWM0 Dead-Band Falling-Edge-Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_dbfall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_dbfall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_dbfall`]
module"]
pub type _0_DBFALL = crate::Reg<_0_dbfall::_0_DBFALL_SPEC>;
#[doc = "PWM0 Dead-Band Falling-Edge-Delay"]
pub mod _0_dbfall;
#[doc = "_0_FLTSRC0 (rw) register accessor: PWM0 Fault Source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_fltsrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_fltsrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_fltsrc0`]
module"]
pub type _0_FLTSRC0 = crate::Reg<_0_fltsrc0::_0_FLTSRC0_SPEC>;
#[doc = "PWM0 Fault Source 0"]
pub mod _0_fltsrc0;
#[doc = "_0_FLTSRC1 (rw) register accessor: PWM0 Fault Source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_fltsrc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_fltsrc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_fltsrc1`]
module"]
pub type _0_FLTSRC1 = crate::Reg<_0_fltsrc1::_0_FLTSRC1_SPEC>;
#[doc = "PWM0 Fault Source 1"]
pub mod _0_fltsrc1;
#[doc = "_0_MINFLTPER (rw) register accessor: PWM0 Minimum Fault Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_minfltper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_minfltper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_minfltper`]
module"]
pub type _0_MINFLTPER = crate::Reg<_0_minfltper::_0_MINFLTPER_SPEC>;
#[doc = "PWM0 Minimum Fault Period"]
pub mod _0_minfltper;
#[doc = "_1_CTL (rw) register accessor: PWM1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_ctl`]
module"]
pub type _1_CTL = crate::Reg<_1_ctl::_1_CTL_SPEC>;
#[doc = "PWM1 Control"]
pub mod _1_ctl;
#[doc = "_1_INTEN (rw) register accessor: PWM1 Interrupt and Trigger Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_inten`]
module"]
pub type _1_INTEN = crate::Reg<_1_inten::_1_INTEN_SPEC>;
#[doc = "PWM1 Interrupt and Trigger Enable"]
pub mod _1_inten;
#[doc = "_1_RIS (rw) register accessor: PWM1 Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_ris`]
module"]
pub type _1_RIS = crate::Reg<_1_ris::_1_RIS_SPEC>;
#[doc = "PWM1 Raw Interrupt Status"]
pub mod _1_ris;
#[doc = "_1_ISC (rw) register accessor: PWM1 Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_isc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_isc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_isc`]
module"]
pub type _1_ISC = crate::Reg<_1_isc::_1_ISC_SPEC>;
#[doc = "PWM1 Interrupt Status and Clear"]
pub mod _1_isc;
#[doc = "_1_LOAD (rw) register accessor: PWM1 Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_load`]
module"]
pub type _1_LOAD = crate::Reg<_1_load::_1_LOAD_SPEC>;
#[doc = "PWM1 Load"]
pub mod _1_load;
#[doc = "_1_COUNT (rw) register accessor: PWM1 Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_count`]
module"]
pub type _1_COUNT = crate::Reg<_1_count::_1_COUNT_SPEC>;
#[doc = "PWM1 Counter"]
pub mod _1_count;
#[doc = "_1_CMPA (rw) register accessor: PWM1 Compare A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_cmpa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_cmpa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_cmpa`]
module"]
pub type _1_CMPA = crate::Reg<_1_cmpa::_1_CMPA_SPEC>;
#[doc = "PWM1 Compare A"]
pub mod _1_cmpa;
#[doc = "_1_CMPB (rw) register accessor: PWM1 Compare B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_cmpb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_cmpb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_cmpb`]
module"]
pub type _1_CMPB = crate::Reg<_1_cmpb::_1_CMPB_SPEC>;
#[doc = "PWM1 Compare B"]
pub mod _1_cmpb;
#[doc = "_1_GENA (rw) register accessor: PWM1 Generator A Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_gena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_gena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_gena`]
module"]
pub type _1_GENA = crate::Reg<_1_gena::_1_GENA_SPEC>;
#[doc = "PWM1 Generator A Control"]
pub mod _1_gena;
#[doc = "_1_GENB (rw) register accessor: PWM1 Generator B Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_genb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_genb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_genb`]
module"]
pub type _1_GENB = crate::Reg<_1_genb::_1_GENB_SPEC>;
#[doc = "PWM1 Generator B Control"]
pub mod _1_genb;
#[doc = "_1_DBCTL (rw) register accessor: PWM1 Dead-Band Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_dbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_dbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_dbctl`]
module"]
pub type _1_DBCTL = crate::Reg<_1_dbctl::_1_DBCTL_SPEC>;
#[doc = "PWM1 Dead-Band Control"]
pub mod _1_dbctl;
#[doc = "_1_DBRISE (rw) register accessor: PWM1 Dead-Band Rising-Edge Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_dbrise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_dbrise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_dbrise`]
module"]
pub type _1_DBRISE = crate::Reg<_1_dbrise::_1_DBRISE_SPEC>;
#[doc = "PWM1 Dead-Band Rising-Edge Delay"]
pub mod _1_dbrise;
#[doc = "_1_DBFALL (rw) register accessor: PWM1 Dead-Band Falling-Edge-Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_dbfall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_dbfall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_dbfall`]
module"]
pub type _1_DBFALL = crate::Reg<_1_dbfall::_1_DBFALL_SPEC>;
#[doc = "PWM1 Dead-Band Falling-Edge-Delay"]
pub mod _1_dbfall;
#[doc = "_1_FLTSRC0 (rw) register accessor: PWM1 Fault Source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_fltsrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_fltsrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_fltsrc0`]
module"]
pub type _1_FLTSRC0 = crate::Reg<_1_fltsrc0::_1_FLTSRC0_SPEC>;
#[doc = "PWM1 Fault Source 0"]
pub mod _1_fltsrc0;
#[doc = "_1_FLTSRC1 (rw) register accessor: PWM1 Fault Source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_fltsrc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_fltsrc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_fltsrc1`]
module"]
pub type _1_FLTSRC1 = crate::Reg<_1_fltsrc1::_1_FLTSRC1_SPEC>;
#[doc = "PWM1 Fault Source 1"]
pub mod _1_fltsrc1;
#[doc = "_1_MINFLTPER (rw) register accessor: PWM1 Minimum Fault Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_minfltper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_minfltper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_minfltper`]
module"]
pub type _1_MINFLTPER = crate::Reg<_1_minfltper::_1_MINFLTPER_SPEC>;
#[doc = "PWM1 Minimum Fault Period"]
pub mod _1_minfltper;
#[doc = "_2_CTL (rw) register accessor: PWM2 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_ctl`]
module"]
pub type _2_CTL = crate::Reg<_2_ctl::_2_CTL_SPEC>;
#[doc = "PWM2 Control"]
pub mod _2_ctl;
#[doc = "_2_INTEN (rw) register accessor: PWM2 Interrupt and Trigger Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_inten`]
module"]
pub type _2_INTEN = crate::Reg<_2_inten::_2_INTEN_SPEC>;
#[doc = "PWM2 Interrupt and Trigger Enable"]
pub mod _2_inten;
#[doc = "_2_RIS (rw) register accessor: PWM2 Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_ris`]
module"]
pub type _2_RIS = crate::Reg<_2_ris::_2_RIS_SPEC>;
#[doc = "PWM2 Raw Interrupt Status"]
pub mod _2_ris;
#[doc = "_2_ISC (rw) register accessor: PWM2 Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_isc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_isc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_isc`]
module"]
pub type _2_ISC = crate::Reg<_2_isc::_2_ISC_SPEC>;
#[doc = "PWM2 Interrupt Status and Clear"]
pub mod _2_isc;
#[doc = "_2_LOAD (rw) register accessor: PWM2 Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_load`]
module"]
pub type _2_LOAD = crate::Reg<_2_load::_2_LOAD_SPEC>;
#[doc = "PWM2 Load"]
pub mod _2_load;
#[doc = "_2_COUNT (rw) register accessor: PWM2 Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_count`]
module"]
pub type _2_COUNT = crate::Reg<_2_count::_2_COUNT_SPEC>;
#[doc = "PWM2 Counter"]
pub mod _2_count;
#[doc = "_2_CMPA (rw) register accessor: PWM2 Compare A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_cmpa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_cmpa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_cmpa`]
module"]
pub type _2_CMPA = crate::Reg<_2_cmpa::_2_CMPA_SPEC>;
#[doc = "PWM2 Compare A"]
pub mod _2_cmpa;
#[doc = "_2_CMPB (rw) register accessor: PWM2 Compare B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_cmpb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_cmpb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_cmpb`]
module"]
pub type _2_CMPB = crate::Reg<_2_cmpb::_2_CMPB_SPEC>;
#[doc = "PWM2 Compare B"]
pub mod _2_cmpb;
#[doc = "_2_GENA (rw) register accessor: PWM2 Generator A Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_gena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_gena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_gena`]
module"]
pub type _2_GENA = crate::Reg<_2_gena::_2_GENA_SPEC>;
#[doc = "PWM2 Generator A Control"]
pub mod _2_gena;
#[doc = "_2_GENB (rw) register accessor: PWM2 Generator B Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_genb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_genb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_genb`]
module"]
pub type _2_GENB = crate::Reg<_2_genb::_2_GENB_SPEC>;
#[doc = "PWM2 Generator B Control"]
pub mod _2_genb;
#[doc = "_2_DBCTL (rw) register accessor: PWM2 Dead-Band Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_dbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_dbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_dbctl`]
module"]
pub type _2_DBCTL = crate::Reg<_2_dbctl::_2_DBCTL_SPEC>;
#[doc = "PWM2 Dead-Band Control"]
pub mod _2_dbctl;
#[doc = "_2_DBRISE (rw) register accessor: PWM2 Dead-Band Rising-Edge Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_dbrise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_dbrise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_dbrise`]
module"]
pub type _2_DBRISE = crate::Reg<_2_dbrise::_2_DBRISE_SPEC>;
#[doc = "PWM2 Dead-Band Rising-Edge Delay"]
pub mod _2_dbrise;
#[doc = "_2_DBFALL (rw) register accessor: PWM2 Dead-Band Falling-Edge-Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_dbfall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_dbfall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_dbfall`]
module"]
pub type _2_DBFALL = crate::Reg<_2_dbfall::_2_DBFALL_SPEC>;
#[doc = "PWM2 Dead-Band Falling-Edge-Delay"]
pub mod _2_dbfall;
#[doc = "_2_FLTSRC0 (rw) register accessor: PWM2 Fault Source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_fltsrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_fltsrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_fltsrc0`]
module"]
pub type _2_FLTSRC0 = crate::Reg<_2_fltsrc0::_2_FLTSRC0_SPEC>;
#[doc = "PWM2 Fault Source 0"]
pub mod _2_fltsrc0;
#[doc = "_2_FLTSRC1 (rw) register accessor: PWM2 Fault Source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_fltsrc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_fltsrc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_fltsrc1`]
module"]
pub type _2_FLTSRC1 = crate::Reg<_2_fltsrc1::_2_FLTSRC1_SPEC>;
#[doc = "PWM2 Fault Source 1"]
pub mod _2_fltsrc1;
#[doc = "_2_MINFLTPER (rw) register accessor: PWM2 Minimum Fault Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_minfltper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_minfltper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_minfltper`]
module"]
pub type _2_MINFLTPER = crate::Reg<_2_minfltper::_2_MINFLTPER_SPEC>;
#[doc = "PWM2 Minimum Fault Period"]
pub mod _2_minfltper;
#[doc = "_3_CTL (rw) register accessor: PWM3 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_ctl`]
module"]
pub type _3_CTL = crate::Reg<_3_ctl::_3_CTL_SPEC>;
#[doc = "PWM3 Control"]
pub mod _3_ctl;
#[doc = "_3_INTEN (rw) register accessor: PWM3 Interrupt and Trigger Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_inten`]
module"]
pub type _3_INTEN = crate::Reg<_3_inten::_3_INTEN_SPEC>;
#[doc = "PWM3 Interrupt and Trigger Enable"]
pub mod _3_inten;
#[doc = "_3_RIS (rw) register accessor: PWM3 Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_ris`]
module"]
pub type _3_RIS = crate::Reg<_3_ris::_3_RIS_SPEC>;
#[doc = "PWM3 Raw Interrupt Status"]
pub mod _3_ris;
#[doc = "_3_ISC (rw) register accessor: PWM3 Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_isc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_isc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_isc`]
module"]
pub type _3_ISC = crate::Reg<_3_isc::_3_ISC_SPEC>;
#[doc = "PWM3 Interrupt Status and Clear"]
pub mod _3_isc;
#[doc = "_3_LOAD (rw) register accessor: PWM3 Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_load`]
module"]
pub type _3_LOAD = crate::Reg<_3_load::_3_LOAD_SPEC>;
#[doc = "PWM3 Load"]
pub mod _3_load;
#[doc = "_3_COUNT (rw) register accessor: PWM3 Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_count`]
module"]
pub type _3_COUNT = crate::Reg<_3_count::_3_COUNT_SPEC>;
#[doc = "PWM3 Counter"]
pub mod _3_count;
#[doc = "_3_CMPA (rw) register accessor: PWM3 Compare A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_cmpa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_cmpa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_cmpa`]
module"]
pub type _3_CMPA = crate::Reg<_3_cmpa::_3_CMPA_SPEC>;
#[doc = "PWM3 Compare A"]
pub mod _3_cmpa;
#[doc = "_3_CMPB (rw) register accessor: PWM3 Compare B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_cmpb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_cmpb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_cmpb`]
module"]
pub type _3_CMPB = crate::Reg<_3_cmpb::_3_CMPB_SPEC>;
#[doc = "PWM3 Compare B"]
pub mod _3_cmpb;
#[doc = "_3_GENA (rw) register accessor: PWM3 Generator A Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_gena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_gena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_gena`]
module"]
pub type _3_GENA = crate::Reg<_3_gena::_3_GENA_SPEC>;
#[doc = "PWM3 Generator A Control"]
pub mod _3_gena;
#[doc = "_3_GENB (rw) register accessor: PWM3 Generator B Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_genb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_genb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_genb`]
module"]
pub type _3_GENB = crate::Reg<_3_genb::_3_GENB_SPEC>;
#[doc = "PWM3 Generator B Control"]
pub mod _3_genb;
#[doc = "_3_DBCTL (rw) register accessor: PWM3 Dead-Band Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_dbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_dbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_dbctl`]
module"]
pub type _3_DBCTL = crate::Reg<_3_dbctl::_3_DBCTL_SPEC>;
#[doc = "PWM3 Dead-Band Control"]
pub mod _3_dbctl;
#[doc = "_3_DBRISE (rw) register accessor: PWM3 Dead-Band Rising-Edge Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_dbrise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_dbrise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_dbrise`]
module"]
pub type _3_DBRISE = crate::Reg<_3_dbrise::_3_DBRISE_SPEC>;
#[doc = "PWM3 Dead-Band Rising-Edge Delay"]
pub mod _3_dbrise;
#[doc = "_3_DBFALL (rw) register accessor: PWM3 Dead-Band Falling-Edge-Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_dbfall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_dbfall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_dbfall`]
module"]
pub type _3_DBFALL = crate::Reg<_3_dbfall::_3_DBFALL_SPEC>;
#[doc = "PWM3 Dead-Band Falling-Edge-Delay"]
pub mod _3_dbfall;
#[doc = "_3_FLTSRC0 (rw) register accessor: PWM3 Fault Source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_fltsrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_fltsrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_fltsrc0`]
module"]
pub type _3_FLTSRC0 = crate::Reg<_3_fltsrc0::_3_FLTSRC0_SPEC>;
#[doc = "PWM3 Fault Source 0"]
pub mod _3_fltsrc0;
#[doc = "_3_FLTSRC1 (rw) register accessor: PWM3 Fault Source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_fltsrc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_fltsrc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_fltsrc1`]
module"]
pub type _3_FLTSRC1 = crate::Reg<_3_fltsrc1::_3_FLTSRC1_SPEC>;
#[doc = "PWM3 Fault Source 1"]
pub mod _3_fltsrc1;
#[doc = "_3_MINFLTPER (rw) register accessor: PWM3 Minimum Fault Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_minfltper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_minfltper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_minfltper`]
module"]
pub type _3_MINFLTPER = crate::Reg<_3_minfltper::_3_MINFLTPER_SPEC>;
#[doc = "PWM3 Minimum Fault Period"]
pub mod _3_minfltper;
#[doc = "_0_FLTSEN (rw) register accessor: PWM0 Fault Pin Logic Sense\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_fltsen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_fltsen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_fltsen`]
module"]
pub type _0_FLTSEN = crate::Reg<_0_fltsen::_0_FLTSEN_SPEC>;
#[doc = "PWM0 Fault Pin Logic Sense"]
pub mod _0_fltsen;
#[doc = "_0_FLTSTAT0 (r) register accessor: PWM0 Fault Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_fltstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_fltstat0`]
module"]
pub type _0_FLTSTAT0 = crate::Reg<_0_fltstat0::_0_FLTSTAT0_SPEC>;
#[doc = "PWM0 Fault Status 0"]
pub mod _0_fltstat0;
#[doc = "_0_FLTSTAT1 (r) register accessor: PWM0 Fault Status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_fltstat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_fltstat1`]
module"]
pub type _0_FLTSTAT1 = crate::Reg<_0_fltstat1::_0_FLTSTAT1_SPEC>;
#[doc = "PWM0 Fault Status 1"]
pub mod _0_fltstat1;
#[doc = "_1_FLTSEN (rw) register accessor: PWM1 Fault Pin Logic Sense\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_fltsen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_fltsen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_fltsen`]
module"]
pub type _1_FLTSEN = crate::Reg<_1_fltsen::_1_FLTSEN_SPEC>;
#[doc = "PWM1 Fault Pin Logic Sense"]
pub mod _1_fltsen;
#[doc = "_1_FLTSTAT0 (r) register accessor: PWM1 Fault Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_fltstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_fltstat0`]
module"]
pub type _1_FLTSTAT0 = crate::Reg<_1_fltstat0::_1_FLTSTAT0_SPEC>;
#[doc = "PWM1 Fault Status 0"]
pub mod _1_fltstat0;
#[doc = "_1_FLTSTAT1 (r) register accessor: PWM1 Fault Status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_fltstat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_fltstat1`]
module"]
pub type _1_FLTSTAT1 = crate::Reg<_1_fltstat1::_1_FLTSTAT1_SPEC>;
#[doc = "PWM1 Fault Status 1"]
pub mod _1_fltstat1;
#[doc = "_2_FLTSTAT0 (r) register accessor: PWM2 Fault Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_fltstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_fltstat0`]
module"]
pub type _2_FLTSTAT0 = crate::Reg<_2_fltstat0::_2_FLTSTAT0_SPEC>;
#[doc = "PWM2 Fault Status 0"]
pub mod _2_fltstat0;
#[doc = "_2_FLTSTAT1 (r) register accessor: PWM2 Fault Status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_fltstat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_fltstat1`]
module"]
pub type _2_FLTSTAT1 = crate::Reg<_2_fltstat1::_2_FLTSTAT1_SPEC>;
#[doc = "PWM2 Fault Status 1"]
pub mod _2_fltstat1;
#[doc = "_3_FLTSTAT0 (r) register accessor: PWM3 Fault Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_fltstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_fltstat0`]
module"]
pub type _3_FLTSTAT0 = crate::Reg<_3_fltstat0::_3_FLTSTAT0_SPEC>;
#[doc = "PWM3 Fault Status 0"]
pub mod _3_fltstat0;
#[doc = "_3_FLTSTAT1 (r) register accessor: PWM3 Fault Status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_fltstat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_fltstat1`]
module"]
pub type _3_FLTSTAT1 = crate::Reg<_3_fltstat1::_3_FLTSTAT1_SPEC>;
#[doc = "PWM3 Fault Status 1"]
pub mod _3_fltstat1;
#[doc = "PP (rw) register accessor: PWM Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp`]
module"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "PWM Peripheral Properties"]
pub mod pp;
