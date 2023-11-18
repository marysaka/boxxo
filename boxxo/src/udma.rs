#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status"]
    pub stat: STAT,
    #[doc = "0x04 - DMA Configuration"]
    pub cfg: CFG,
    #[doc = "0x08 - DMA Channel Control Base Pointer"]
    pub ctlbase: CTLBASE,
    #[doc = "0x0c - DMA Alternate Channel Control Base Pointer"]
    pub altbase: ALTBASE,
    #[doc = "0x10 - DMA Channel Wait-on-Request Status"]
    pub waitstat: WAITSTAT,
    #[doc = "0x14 - DMA Channel Software Request"]
    pub swreq: SWREQ,
    #[doc = "0x18 - DMA Channel Useburst Set"]
    pub useburstset: USEBURSTSET,
    #[doc = "0x1c - DMA Channel Useburst Clear"]
    pub useburstclr: USEBURSTCLR,
    #[doc = "0x20 - DMA Channel Request Mask Set"]
    pub reqmaskset: REQMASKSET,
    #[doc = "0x24 - DMA Channel Request Mask Clear"]
    pub reqmaskclr: REQMASKCLR,
    #[doc = "0x28 - DMA Channel Enable Set"]
    pub enaset: ENASET,
    #[doc = "0x2c - DMA Channel Enable Clear"]
    pub enaclr: ENACLR,
    #[doc = "0x30 - DMA Channel Primary Alternate Set"]
    pub altset: ALTSET,
    #[doc = "0x34 - DMA Channel Primary Alternate Clear"]
    pub altclr: ALTCLR,
    #[doc = "0x38 - DMA Channel Priority Set"]
    pub prioset: PRIOSET,
    #[doc = "0x3c - DMA Channel Priority Clear"]
    pub prioclr: PRIOCLR,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - DMA Bus Error Clear"]
    pub errclr: ERRCLR,
    _reserved17: [u8; 0x04b0],
    #[doc = "0x500 - DMA Channel Assignment"]
    pub chasgn: CHASGN,
    #[doc = "0x504 - DMA Channel Interrupt Status"]
    pub chis: CHIS,
    _reserved19: [u8; 0x08],
    #[doc = "0x510 - DMA Channel Map Select 0"]
    pub chmap0: CHMAP0,
    #[doc = "0x514 - DMA Channel Map Select 1"]
    pub chmap1: CHMAP1,
    #[doc = "0x518 - DMA Channel Map Select 2"]
    pub chmap2: CHMAP2,
    #[doc = "0x51c - DMA Channel Map Select 3"]
    pub chmap3: CHMAP3,
}
#[doc = "STAT (rw) register accessor: DMA Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "DMA Status"]
pub mod stat;
#[doc = "CFG (w) register accessor: DMA Configuration\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "DMA Configuration"]
pub mod cfg;
#[doc = "CTLBASE (rw) register accessor: DMA Channel Control Base Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlbase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlbase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlbase`]
module"]
pub type CTLBASE = crate::Reg<ctlbase::CTLBASE_SPEC>;
#[doc = "DMA Channel Control Base Pointer"]
pub mod ctlbase;
#[doc = "ALTBASE (rw) register accessor: DMA Alternate Channel Control Base Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altbase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altbase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altbase`]
module"]
pub type ALTBASE = crate::Reg<altbase::ALTBASE_SPEC>;
#[doc = "DMA Alternate Channel Control Base Pointer"]
pub mod altbase;
#[doc = "WAITSTAT (rw) register accessor: DMA Channel Wait-on-Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waitstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`waitstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waitstat`]
module"]
pub type WAITSTAT = crate::Reg<waitstat::WAITSTAT_SPEC>;
#[doc = "DMA Channel Wait-on-Request Status"]
pub mod waitstat;
#[doc = "SWREQ (w) register accessor: DMA Channel Software Request\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreq`]
module"]
pub type SWREQ = crate::Reg<swreq::SWREQ_SPEC>;
#[doc = "DMA Channel Software Request"]
pub mod swreq;
#[doc = "USEBURSTSET (rw) register accessor: DMA Channel Useburst Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`useburstset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`useburstset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@useburstset`]
module"]
pub type USEBURSTSET = crate::Reg<useburstset::USEBURSTSET_SPEC>;
#[doc = "DMA Channel Useburst Set"]
pub mod useburstset;
#[doc = "USEBURSTCLR (w) register accessor: DMA Channel Useburst Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`useburstclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@useburstclr`]
module"]
pub type USEBURSTCLR = crate::Reg<useburstclr::USEBURSTCLR_SPEC>;
#[doc = "DMA Channel Useburst Clear"]
pub mod useburstclr;
#[doc = "REQMASKSET (rw) register accessor: DMA Channel Request Mask Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqmaskset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqmaskset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqmaskset`]
module"]
pub type REQMASKSET = crate::Reg<reqmaskset::REQMASKSET_SPEC>;
#[doc = "DMA Channel Request Mask Set"]
pub mod reqmaskset;
#[doc = "REQMASKCLR (w) register accessor: DMA Channel Request Mask Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqmaskclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqmaskclr`]
module"]
pub type REQMASKCLR = crate::Reg<reqmaskclr::REQMASKCLR_SPEC>;
#[doc = "DMA Channel Request Mask Clear"]
pub mod reqmaskclr;
#[doc = "ENASET (rw) register accessor: DMA Channel Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enaset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enaset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enaset`]
module"]
pub type ENASET = crate::Reg<enaset::ENASET_SPEC>;
#[doc = "DMA Channel Enable Set"]
pub mod enaset;
#[doc = "ENACLR (w) register accessor: DMA Channel Enable Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enaclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enaclr`]
module"]
pub type ENACLR = crate::Reg<enaclr::ENACLR_SPEC>;
#[doc = "DMA Channel Enable Clear"]
pub mod enaclr;
#[doc = "ALTSET (rw) register accessor: DMA Channel Primary Alternate Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altset`]
module"]
pub type ALTSET = crate::Reg<altset::ALTSET_SPEC>;
#[doc = "DMA Channel Primary Alternate Set"]
pub mod altset;
#[doc = "ALTCLR (w) register accessor: DMA Channel Primary Alternate Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altclr`]
module"]
pub type ALTCLR = crate::Reg<altclr::ALTCLR_SPEC>;
#[doc = "DMA Channel Primary Alternate Clear"]
pub mod altclr;
#[doc = "PRIOSET (rw) register accessor: DMA Channel Priority Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prioset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prioset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prioset`]
module"]
pub type PRIOSET = crate::Reg<prioset::PRIOSET_SPEC>;
#[doc = "DMA Channel Priority Set"]
pub mod prioset;
#[doc = "PRIOCLR (w) register accessor: DMA Channel Priority Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prioclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prioclr`]
module"]
pub type PRIOCLR = crate::Reg<prioclr::PRIOCLR_SPEC>;
#[doc = "DMA Channel Priority Clear"]
pub mod prioclr;
#[doc = "ERRCLR (rw) register accessor: DMA Bus Error Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errclr`]
module"]
pub type ERRCLR = crate::Reg<errclr::ERRCLR_SPEC>;
#[doc = "DMA Bus Error Clear"]
pub mod errclr;
#[doc = "CHASGN (rw) register accessor: DMA Channel Assignment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chasgn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chasgn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chasgn`]
module"]
pub type CHASGN = crate::Reg<chasgn::CHASGN_SPEC>;
#[doc = "DMA Channel Assignment"]
pub mod chasgn;
#[doc = "CHIS (rw) register accessor: DMA Channel Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chis`]
module"]
pub type CHIS = crate::Reg<chis::CHIS_SPEC>;
#[doc = "DMA Channel Interrupt Status"]
pub mod chis;
#[doc = "CHMAP0 (rw) register accessor: DMA Channel Map Select 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmap0`]
module"]
pub type CHMAP0 = crate::Reg<chmap0::CHMAP0_SPEC>;
#[doc = "DMA Channel Map Select 0"]
pub mod chmap0;
#[doc = "CHMAP1 (rw) register accessor: DMA Channel Map Select 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmap1`]
module"]
pub type CHMAP1 = crate::Reg<chmap1::CHMAP1_SPEC>;
#[doc = "DMA Channel Map Select 1"]
pub mod chmap1;
#[doc = "CHMAP2 (rw) register accessor: DMA Channel Map Select 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmap2`]
module"]
pub type CHMAP2 = crate::Reg<chmap2::CHMAP2_SPEC>;
#[doc = "DMA Channel Map Select 2"]
pub mod chmap2;
#[doc = "CHMAP3 (rw) register accessor: DMA Channel Map Select 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmap3`]
module"]
pub type CHMAP3 = crate::Reg<chmap3::CHMAP3_SPEC>;
#[doc = "DMA Channel Map Select 3"]
pub mod chmap3;
