#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Memory Address"]
    pub fma: FMA,
    #[doc = "0x04 - Flash Memory Data"]
    pub fmd: FMD,
    #[doc = "0x08 - Flash Memory Control"]
    pub fmc: FMC,
    #[doc = "0x0c - Flash Controller Raw Interrupt Status"]
    pub fcris: FCRIS,
    #[doc = "0x10 - Flash Controller Interrupt Mask"]
    pub fcim: FCIM,
    #[doc = "0x14 - Flash Controller Masked Interrupt Status and Clear"]
    pub fcmisc: FCMISC,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Flash Memory Control 2"]
    pub fmc2: FMC2,
    _reserved7: [u8; 0x0c],
    #[doc = "0x30 - Flash Write Buffer Valid"]
    pub fwbval: FWBVAL,
    _reserved8: [u8; 0xcc],
    #[doc = "0x100 - Flash Write Buffer n"]
    pub fwbn: FWBN,
    _reserved9: [u8; 0x0ebc],
    #[doc = "0xfc0 - Flash Size"]
    pub fsize: FSIZE,
    #[doc = "0xfc4 - SRAM Size"]
    pub ssize: SSIZE,
    _reserved11: [u8; 0x04],
    _reserved_11_romswmap: [u8; 0x04],
    _reserved12: [u8; 0x0120],
    #[doc = "0x10f0 - ROM Control"]
    pub rmctl: RMCTL,
    _reserved13: [u8; 0xdc],
    #[doc = "0x11d0 - Boot Configuration"]
    pub bootcfg: BOOTCFG,
    _reserved14: [u8; 0x0c],
    #[doc = "0x11e0 - User Register 0"]
    pub userreg0: USERREG0,
    #[doc = "0x11e4 - User Register 1"]
    pub userreg1: USERREG1,
    #[doc = "0x11e8 - User Register 2"]
    pub userreg2: USERREG2,
    #[doc = "0x11ec - User Register 3"]
    pub userreg3: USERREG3,
    _reserved18: [u8; 0x10],
    #[doc = "0x1200 - Flash Memory Protection Read Enable 0"]
    pub fmpre0: FMPRE0,
    #[doc = "0x1204 - Flash Memory Protection Read Enable 1"]
    pub fmpre1: FMPRE1,
    #[doc = "0x1208 - Flash Memory Protection Read Enable 2"]
    pub fmpre2: FMPRE2,
    #[doc = "0x120c - Flash Memory Protection Read Enable 3"]
    pub fmpre3: FMPRE3,
    _reserved22: [u8; 0x01f0],
    #[doc = "0x1400 - Flash Memory Protection Program Enable 0"]
    pub fmppe0: FMPPE0,
    #[doc = "0x1404 - Flash Memory Protection Program Enable 1"]
    pub fmppe1: FMPPE1,
    #[doc = "0x1408 - Flash Memory Protection Program Enable 2"]
    pub fmppe2: FMPPE2,
    #[doc = "0x140c - Flash Memory Protection Program Enable 3"]
    pub fmppe3: FMPPE3,
}
impl RegisterBlock {
    #[doc = "0xfcc - ROM Software Map"]
    #[inline(always)]
    pub const fn flash_ctrl_alt_romswmap(&self) -> &FLASH_CTRL_ALT_ROMSWMAP {
        unsafe { &*(self as *const Self).cast::<u8>().add(4044usize).cast() }
    }
    #[doc = "0xfcc - ROM Software Map"]
    #[inline(always)]
    pub const fn romswmap(&self) -> &ROMSWMAP {
        unsafe { &*(self as *const Self).cast::<u8>().add(4044usize).cast() }
    }
}
#[doc = "FMA (rw) register accessor: Flash Memory Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fma`]
module"]
pub type FMA = crate::Reg<fma::FMA_SPEC>;
#[doc = "Flash Memory Address"]
pub mod fma;
#[doc = "FMD (rw) register accessor: Flash Memory Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmd`]
module"]
pub type FMD = crate::Reg<fmd::FMD_SPEC>;
#[doc = "Flash Memory Data"]
pub mod fmd;
#[doc = "FMC (rw) register accessor: Flash Memory Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc`]
module"]
pub type FMC = crate::Reg<fmc::FMC_SPEC>;
#[doc = "Flash Memory Control"]
pub mod fmc;
#[doc = "FCRIS (rw) register accessor: Flash Controller Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcris`]
module"]
pub type FCRIS = crate::Reg<fcris::FCRIS_SPEC>;
#[doc = "Flash Controller Raw Interrupt Status"]
pub mod fcris;
#[doc = "FCIM (rw) register accessor: Flash Controller Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcim`]
module"]
pub type FCIM = crate::Reg<fcim::FCIM_SPEC>;
#[doc = "Flash Controller Interrupt Mask"]
pub mod fcim;
#[doc = "FCMISC (rw) register accessor: Flash Controller Masked Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcmisc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcmisc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcmisc`]
module"]
pub type FCMISC = crate::Reg<fcmisc::FCMISC_SPEC>;
#[doc = "Flash Controller Masked Interrupt Status and Clear"]
pub mod fcmisc;
#[doc = "FMC2 (rw) register accessor: Flash Memory Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc2`]
module"]
pub type FMC2 = crate::Reg<fmc2::FMC2_SPEC>;
#[doc = "Flash Memory Control 2"]
pub mod fmc2;
#[doc = "FWBVAL (rw) register accessor: Flash Write Buffer Valid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwbval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwbval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwbval`]
module"]
pub type FWBVAL = crate::Reg<fwbval::FWBVAL_SPEC>;
#[doc = "Flash Write Buffer Valid"]
pub mod fwbval;
#[doc = "FWBN (rw) register accessor: Flash Write Buffer n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwbn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwbn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwbn`]
module"]
pub type FWBN = crate::Reg<fwbn::FWBN_SPEC>;
#[doc = "Flash Write Buffer n"]
pub mod fwbn;
#[doc = "FSIZE (rw) register accessor: Flash Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsize`]
module"]
pub type FSIZE = crate::Reg<fsize::FSIZE_SPEC>;
#[doc = "Flash Size"]
pub mod fsize;
#[doc = "SSIZE (rw) register accessor: SRAM Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssize`]
module"]
pub type SSIZE = crate::Reg<ssize::SSIZE_SPEC>;
#[doc = "SRAM Size"]
pub mod ssize;
#[doc = "ROMSWMAP (rw) register accessor: ROM Software Map\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`romswmap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`romswmap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@romswmap`]
module"]
pub type ROMSWMAP = crate::Reg<romswmap::ROMSWMAP_SPEC>;
#[doc = "ROM Software Map"]
pub mod romswmap;
#[doc = "FLASH_CTRL_ALT_ROMSWMAP (rw) register accessor: ROM Software Map\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ctrl_alt_romswmap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ctrl_alt_romswmap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ctrl_alt_romswmap`]
module"]
pub type FLASH_CTRL_ALT_ROMSWMAP =
    crate::Reg<flash_ctrl_alt_romswmap::FLASH_CTRL_ALT_ROMSWMAP_SPEC>;
#[doc = "ROM Software Map"]
pub mod flash_ctrl_alt_romswmap;
#[doc = "RMCTL (rw) register accessor: ROM Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmctl`]
module"]
pub type RMCTL = crate::Reg<rmctl::RMCTL_SPEC>;
#[doc = "ROM Control"]
pub mod rmctl;
#[doc = "BOOTCFG (rw) register accessor: Boot Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bootcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bootcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootcfg`]
module"]
pub type BOOTCFG = crate::Reg<bootcfg::BOOTCFG_SPEC>;
#[doc = "Boot Configuration"]
pub mod bootcfg;
#[doc = "USERREG0 (rw) register accessor: User Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userreg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userreg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userreg0`]
module"]
pub type USERREG0 = crate::Reg<userreg0::USERREG0_SPEC>;
#[doc = "User Register 0"]
pub mod userreg0;
#[doc = "USERREG1 (rw) register accessor: User Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userreg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userreg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userreg1`]
module"]
pub type USERREG1 = crate::Reg<userreg1::USERREG1_SPEC>;
#[doc = "User Register 1"]
pub mod userreg1;
#[doc = "USERREG2 (rw) register accessor: User Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userreg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userreg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userreg2`]
module"]
pub type USERREG2 = crate::Reg<userreg2::USERREG2_SPEC>;
#[doc = "User Register 2"]
pub mod userreg2;
#[doc = "USERREG3 (rw) register accessor: User Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userreg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userreg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userreg3`]
module"]
pub type USERREG3 = crate::Reg<userreg3::USERREG3_SPEC>;
#[doc = "User Register 3"]
pub mod userreg3;
#[doc = "FMPRE0 (rw) register accessor: Flash Memory Protection Read Enable 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmpre0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmpre0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmpre0`]
module"]
pub type FMPRE0 = crate::Reg<fmpre0::FMPRE0_SPEC>;
#[doc = "Flash Memory Protection Read Enable 0"]
pub mod fmpre0;
#[doc = "FMPRE1 (rw) register accessor: Flash Memory Protection Read Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmpre1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmpre1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmpre1`]
module"]
pub type FMPRE1 = crate::Reg<fmpre1::FMPRE1_SPEC>;
#[doc = "Flash Memory Protection Read Enable 1"]
pub mod fmpre1;
#[doc = "FMPRE2 (rw) register accessor: Flash Memory Protection Read Enable 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmpre2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmpre2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmpre2`]
module"]
pub type FMPRE2 = crate::Reg<fmpre2::FMPRE2_SPEC>;
#[doc = "Flash Memory Protection Read Enable 2"]
pub mod fmpre2;
#[doc = "FMPRE3 (rw) register accessor: Flash Memory Protection Read Enable 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmpre3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmpre3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmpre3`]
module"]
pub type FMPRE3 = crate::Reg<fmpre3::FMPRE3_SPEC>;
#[doc = "Flash Memory Protection Read Enable 3"]
pub mod fmpre3;
#[doc = "FMPPE0 (rw) register accessor: Flash Memory Protection Program Enable 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmppe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmppe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmppe0`]
module"]
pub type FMPPE0 = crate::Reg<fmppe0::FMPPE0_SPEC>;
#[doc = "Flash Memory Protection Program Enable 0"]
pub mod fmppe0;
#[doc = "FMPPE1 (rw) register accessor: Flash Memory Protection Program Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmppe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmppe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmppe1`]
module"]
pub type FMPPE1 = crate::Reg<fmppe1::FMPPE1_SPEC>;
#[doc = "Flash Memory Protection Program Enable 1"]
pub mod fmppe1;
#[doc = "FMPPE2 (rw) register accessor: Flash Memory Protection Program Enable 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmppe2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmppe2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmppe2`]
module"]
pub type FMPPE2 = crate::Reg<fmppe2::FMPPE2_SPEC>;
#[doc = "Flash Memory Protection Program Enable 2"]
pub mod fmppe2;
#[doc = "FMPPE3 (rw) register accessor: Flash Memory Protection Program Enable 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmppe3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmppe3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmppe3`]
module"]
pub type FMPPE3 = crate::Reg<fmppe3::FMPPE3_SPEC>;
#[doc = "Flash Memory Protection Program Enable 3"]
pub mod fmppe3;
