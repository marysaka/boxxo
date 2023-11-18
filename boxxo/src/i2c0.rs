#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Master Slave Address"]
    pub msa: MSA,
    _reserved_1_mcs: [u8; 0x04],
    #[doc = "0x08 - I2C Master Data"]
    pub mdr: MDR,
    #[doc = "0x0c - I2C Master Timer Period"]
    pub mtpr: MTPR,
    #[doc = "0x10 - I2C Master Interrupt Mask"]
    pub mimr: MIMR,
    #[doc = "0x14 - I2C Master Raw Interrupt Status"]
    pub mris: MRIS,
    #[doc = "0x18 - I2C Master Masked Interrupt Status"]
    pub mmis: MMIS,
    #[doc = "0x1c - I2C Master Interrupt Clear"]
    pub micr: MICR,
    #[doc = "0x20 - I2C Master Configuration"]
    pub mcr: MCR,
    #[doc = "0x24 - I2C Master Clock Low Timeout Count"]
    pub mclkocnt: MCLKOCNT,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - I2C Master Bus Monitor"]
    pub mbmon: MBMON,
    _reserved11: [u8; 0x08],
    #[doc = "0x38 - I2C Master Configuration 2"]
    pub mcr2: MCR2,
    _reserved12: [u8; 0x07c4],
    #[doc = "0x800 - I2C Slave Own Address"]
    pub soar: SOAR,
    _reserved_13_scsr: [u8; 0x04],
    #[doc = "0x808 - I2C Slave Data"]
    pub sdr: SDR,
    #[doc = "0x80c - I2C Slave Interrupt Mask"]
    pub simr: SIMR,
    #[doc = "0x810 - I2C Slave Raw Interrupt Status"]
    pub sris: SRIS,
    #[doc = "0x814 - I2C Slave Masked Interrupt Status"]
    pub smis: SMIS,
    #[doc = "0x818 - I2C Slave Interrupt Clear"]
    pub sicr: SICR,
    #[doc = "0x81c - I2C Slave Own Address 2"]
    pub soar2: SOAR2,
    #[doc = "0x820 - I2C Slave ACK Control"]
    pub sackctl: SACKCTL,
    _reserved21: [u8; 0x079c],
    #[doc = "0xfc0 - I2C Peripheral Properties"]
    pub pp: PP,
    #[doc = "0xfc4 - I2C Peripheral Configuration"]
    pub pc: PC,
}
impl RegisterBlock {
    #[doc = "0x04 - I2C Master Control/Status"]
    #[inline(always)]
    pub const fn i2c0_alt_mcs(&self) -> &I2C0_ALT_MCS {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - I2C Master Control/Status"]
    #[inline(always)]
    pub const fn mcs(&self) -> &MCS {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x804 - I2C Slave Control/Status"]
    #[inline(always)]
    pub const fn i2c0_alt_scsr(&self) -> &I2C0_ALT_SCSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2052usize).cast() }
    }
    #[doc = "0x804 - I2C Slave Control/Status"]
    #[inline(always)]
    pub const fn scsr(&self) -> &SCSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2052usize).cast() }
    }
}
#[doc = "MSA (rw) register accessor: I2C Master Slave Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msa`]
module"]
pub type MSA = crate::Reg<msa::MSA_SPEC>;
#[doc = "I2C Master Slave Address"]
pub mod msa;
#[doc = "MCS (rw) register accessor: I2C Master Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcs`]
module"]
pub type MCS = crate::Reg<mcs::MCS_SPEC>;
#[doc = "I2C Master Control/Status"]
pub mod mcs;
#[doc = "I2C0_ALT_MCS (rw) register accessor: I2C Master Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_alt_mcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_alt_mcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_alt_mcs`]
module"]
pub type I2C0_ALT_MCS = crate::Reg<i2c0_alt_mcs::I2C0_ALT_MCS_SPEC>;
#[doc = "I2C Master Control/Status"]
pub mod i2c0_alt_mcs;
#[doc = "MDR (rw) register accessor: I2C Master Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdr`]
module"]
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
#[doc = "I2C Master Data"]
pub mod mdr;
#[doc = "MTPR (rw) register accessor: I2C Master Timer Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtpr`]
module"]
pub type MTPR = crate::Reg<mtpr::MTPR_SPEC>;
#[doc = "I2C Master Timer Period"]
pub mod mtpr;
#[doc = "MIMR (rw) register accessor: I2C Master Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mimr`]
module"]
pub type MIMR = crate::Reg<mimr::MIMR_SPEC>;
#[doc = "I2C Master Interrupt Mask"]
pub mod mimr;
#[doc = "MRIS (rw) register accessor: I2C Master Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mris`]
module"]
pub type MRIS = crate::Reg<mris::MRIS_SPEC>;
#[doc = "I2C Master Raw Interrupt Status"]
pub mod mris;
#[doc = "MMIS (rw) register accessor: I2C Master Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmis`]
module"]
pub type MMIS = crate::Reg<mmis::MMIS_SPEC>;
#[doc = "I2C Master Masked Interrupt Status"]
pub mod mmis;
#[doc = "MICR (w) register accessor: I2C Master Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`micr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@micr`]
module"]
pub type MICR = crate::Reg<micr::MICR_SPEC>;
#[doc = "I2C Master Interrupt Clear"]
pub mod micr;
#[doc = "MCR (rw) register accessor: I2C Master Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "I2C Master Configuration"]
pub mod mcr;
#[doc = "MCLKOCNT (rw) register accessor: I2C Master Clock Low Timeout Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkocnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkocnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkocnt`]
module"]
pub type MCLKOCNT = crate::Reg<mclkocnt::MCLKOCNT_SPEC>;
#[doc = "I2C Master Clock Low Timeout Count"]
pub mod mclkocnt;
#[doc = "MBMON (rw) register accessor: I2C Master Bus Monitor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbmon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbmon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbmon`]
module"]
pub type MBMON = crate::Reg<mbmon::MBMON_SPEC>;
#[doc = "I2C Master Bus Monitor"]
pub mod mbmon;
#[doc = "MCR2 (rw) register accessor: I2C Master Configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr2`]
module"]
pub type MCR2 = crate::Reg<mcr2::MCR2_SPEC>;
#[doc = "I2C Master Configuration 2"]
pub mod mcr2;
#[doc = "SOAR (rw) register accessor: I2C Slave Own Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soar`]
module"]
pub type SOAR = crate::Reg<soar::SOAR_SPEC>;
#[doc = "I2C Slave Own Address"]
pub mod soar;
#[doc = "SCSR (rw) register accessor: I2C Slave Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsr`]
module"]
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
#[doc = "I2C Slave Control/Status"]
pub mod scsr;
#[doc = "I2C0_ALT_SCSR (rw) register accessor: I2C Slave Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_alt_scsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_alt_scsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_alt_scsr`]
module"]
pub type I2C0_ALT_SCSR = crate::Reg<i2c0_alt_scsr::I2C0_ALT_SCSR_SPEC>;
#[doc = "I2C Slave Control/Status"]
pub mod i2c0_alt_scsr;
#[doc = "SDR (rw) register accessor: I2C Slave Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdr`]
module"]
pub type SDR = crate::Reg<sdr::SDR_SPEC>;
#[doc = "I2C Slave Data"]
pub mod sdr;
#[doc = "SIMR (rw) register accessor: I2C Slave Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simr`]
module"]
pub type SIMR = crate::Reg<simr::SIMR_SPEC>;
#[doc = "I2C Slave Interrupt Mask"]
pub mod simr;
#[doc = "SRIS (rw) register accessor: I2C Slave Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sris`]
module"]
pub type SRIS = crate::Reg<sris::SRIS_SPEC>;
#[doc = "I2C Slave Raw Interrupt Status"]
pub mod sris;
#[doc = "SMIS (rw) register accessor: I2C Slave Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smis`]
module"]
pub type SMIS = crate::Reg<smis::SMIS_SPEC>;
#[doc = "I2C Slave Masked Interrupt Status"]
pub mod smis;
#[doc = "SICR (w) register accessor: I2C Slave Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sicr`]
module"]
pub type SICR = crate::Reg<sicr::SICR_SPEC>;
#[doc = "I2C Slave Interrupt Clear"]
pub mod sicr;
#[doc = "SOAR2 (rw) register accessor: I2C Slave Own Address 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soar2`]
module"]
pub type SOAR2 = crate::Reg<soar2::SOAR2_SPEC>;
#[doc = "I2C Slave Own Address 2"]
pub mod soar2;
#[doc = "SACKCTL (rw) register accessor: I2C Slave ACK Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sackctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sackctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sackctl`]
module"]
pub type SACKCTL = crate::Reg<sackctl::SACKCTL_SPEC>;
#[doc = "I2C Slave ACK Control"]
pub mod sackctl;
#[doc = "PP (rw) register accessor: I2C Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp`]
module"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "I2C Peripheral Properties"]
pub mod pp;
#[doc = "PC (rw) register accessor: I2C Peripheral Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc`]
module"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "I2C Peripheral Configuration"]
pub mod pc;
