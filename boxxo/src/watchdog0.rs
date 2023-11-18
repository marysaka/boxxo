#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Load"]
    pub load: LOAD,
    #[doc = "0x04 - Watchdog Value"]
    pub value: VALUE,
    #[doc = "0x08 - Watchdog Control"]
    pub ctl: CTL,
    #[doc = "0x0c - Watchdog Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x10 - Watchdog Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x14 - Watchdog Masked Interrupt Status"]
    pub mis: MIS,
    _reserved6: [u8; 0x0400],
    #[doc = "0x418 - Watchdog Test"]
    pub test: TEST,
    _reserved7: [u8; 0x07e4],
    #[doc = "0xc00 - Watchdog Lock"]
    pub lock: LOCK,
}
#[doc = "LOAD (rw) register accessor: Watchdog Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`]
module"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Watchdog Load"]
pub mod load;
#[doc = "VALUE (rw) register accessor: Watchdog Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`]
module"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Watchdog Value"]
pub mod value;
#[doc = "CTL (rw) register accessor: Watchdog Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Watchdog Control"]
pub mod ctl;
#[doc = "ICR (w) register accessor: Watchdog Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Watchdog Interrupt Clear"]
pub mod icr;
#[doc = "RIS (rw) register accessor: Watchdog Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Watchdog Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS (rw) register accessor: Watchdog Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Watchdog Masked Interrupt Status"]
pub mod mis;
#[doc = "TEST (rw) register accessor: Watchdog Test\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Watchdog Test"]
pub mod test;
#[doc = "LOCK (rw) register accessor: Watchdog Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Watchdog Lock"]
pub mod lock;
