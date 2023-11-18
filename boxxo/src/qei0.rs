#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QEI Control"]
    pub ctl: CTL,
    #[doc = "0x04 - QEI Status"]
    pub stat: STAT,
    #[doc = "0x08 - QEI Position"]
    pub pos: POS,
    #[doc = "0x0c - QEI Maximum Position"]
    pub maxpos: MAXPOS,
    #[doc = "0x10 - QEI Timer Load"]
    pub load: LOAD,
    #[doc = "0x14 - QEI Timer"]
    pub time: TIME,
    #[doc = "0x18 - QEI Velocity Counter"]
    pub count: COUNT,
    #[doc = "0x1c - QEI Velocity"]
    pub speed: SPEED,
    #[doc = "0x20 - QEI Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x24 - QEI Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x28 - QEI Interrupt Status and Clear"]
    pub isc: ISC,
}
#[doc = "CTL (rw) register accessor: QEI Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "QEI Control"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: QEI Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "QEI Status"]
pub mod stat;
#[doc = "POS (rw) register accessor: QEI Position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pos`]
module"]
pub type POS = crate::Reg<pos::POS_SPEC>;
#[doc = "QEI Position"]
pub mod pos;
#[doc = "MAXPOS (rw) register accessor: QEI Maximum Position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxpos`]
module"]
pub type MAXPOS = crate::Reg<maxpos::MAXPOS_SPEC>;
#[doc = "QEI Maximum Position"]
pub mod maxpos;
#[doc = "LOAD (rw) register accessor: QEI Timer Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`]
module"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "QEI Timer Load"]
pub mod load;
#[doc = "TIME (rw) register accessor: QEI Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "QEI Timer"]
pub mod time;
#[doc = "COUNT (rw) register accessor: QEI Velocity Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "QEI Velocity Counter"]
pub mod count;
#[doc = "SPEED (rw) register accessor: QEI Velocity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`speed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`speed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@speed`]
module"]
pub type SPEED = crate::Reg<speed::SPEED_SPEC>;
#[doc = "QEI Velocity"]
pub mod speed;
#[doc = "INTEN (rw) register accessor: QEI Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "QEI Interrupt Enable"]
pub mod inten;
#[doc = "RIS (rw) register accessor: QEI Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "QEI Raw Interrupt Status"]
pub mod ris;
#[doc = "ISC (rw) register accessor: QEI Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc`]
module"]
pub type ISC = crate::Reg<isc::ISC_SPEC>;
#[doc = "QEI Interrupt Status and Clear"]
pub mod isc;
