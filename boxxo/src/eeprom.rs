#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM Size Information"]
    pub eesize: EESIZE,
    #[doc = "0x04 - EEPROM Current Block"]
    pub eeblock: EEBLOCK,
    #[doc = "0x08 - EEPROM Current Offset"]
    pub eeoffset: EEOFFSET,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - EEPROM Read-Write"]
    pub eerdwr: EERDWR,
    #[doc = "0x14 - EEPROM Read-Write with Increment"]
    pub eerdwrinc: EERDWRINC,
    #[doc = "0x18 - EEPROM Done Status"]
    pub eedone: EEDONE,
    #[doc = "0x1c - EEPROM Support Control and Status"]
    pub eesupp: EESUPP,
    #[doc = "0x20 - EEPROM Unlock"]
    pub eeunlock: EEUNLOCK,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - EEPROM Protection"]
    pub eeprot: EEPROT,
    #[doc = "0x34 - EEPROM Password"]
    pub eepass0: EEPASS0,
    #[doc = "0x38 - EEPROM Password"]
    pub eepass1: EEPASS1,
    #[doc = "0x3c - EEPROM Password"]
    pub eepass2: EEPASS2,
    #[doc = "0x40 - EEPROM Interrupt"]
    pub eeint: EEINT,
    _reserved13: [u8; 0x0c],
    #[doc = "0x50 - EEPROM Block Hide"]
    pub eehide: EEHIDE,
    _reserved14: [u8; 0x2c],
    #[doc = "0x80 - EEPROM Debug Mass Erase"]
    pub eedbgme: EEDBGME,
    _reserved15: [u8; 0x0f3c],
    #[doc = "0xfc0 - EEPROM Peripheral Properties"]
    pub pp: PP,
}
#[doc = "EESIZE (rw) register accessor: EEPROM Size Information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eesize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eesize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eesize`]
module"]
pub type EESIZE = crate::Reg<eesize::EESIZE_SPEC>;
#[doc = "EEPROM Size Information"]
pub mod eesize;
#[doc = "EEBLOCK (rw) register accessor: EEPROM Current Block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeblock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeblock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeblock`]
module"]
pub type EEBLOCK = crate::Reg<eeblock::EEBLOCK_SPEC>;
#[doc = "EEPROM Current Block"]
pub mod eeblock;
#[doc = "EEOFFSET (rw) register accessor: EEPROM Current Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeoffset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeoffset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeoffset`]
module"]
pub type EEOFFSET = crate::Reg<eeoffset::EEOFFSET_SPEC>;
#[doc = "EEPROM Current Offset"]
pub mod eeoffset;
#[doc = "EERDWR (rw) register accessor: EEPROM Read-Write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eerdwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eerdwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eerdwr`]
module"]
pub type EERDWR = crate::Reg<eerdwr::EERDWR_SPEC>;
#[doc = "EEPROM Read-Write"]
pub mod eerdwr;
#[doc = "EERDWRINC (rw) register accessor: EEPROM Read-Write with Increment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eerdwrinc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eerdwrinc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eerdwrinc`]
module"]
pub type EERDWRINC = crate::Reg<eerdwrinc::EERDWRINC_SPEC>;
#[doc = "EEPROM Read-Write with Increment"]
pub mod eerdwrinc;
#[doc = "EEDONE (rw) register accessor: EEPROM Done Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eedone::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eedone::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eedone`]
module"]
pub type EEDONE = crate::Reg<eedone::EEDONE_SPEC>;
#[doc = "EEPROM Done Status"]
pub mod eedone;
#[doc = "EESUPP (rw) register accessor: EEPROM Support Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eesupp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eesupp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eesupp`]
module"]
pub type EESUPP = crate::Reg<eesupp::EESUPP_SPEC>;
#[doc = "EEPROM Support Control and Status"]
pub mod eesupp;
#[doc = "EEUNLOCK (rw) register accessor: EEPROM Unlock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeunlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeunlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeunlock`]
module"]
pub type EEUNLOCK = crate::Reg<eeunlock::EEUNLOCK_SPEC>;
#[doc = "EEPROM Unlock"]
pub mod eeunlock;
#[doc = "EEPROT (rw) register accessor: EEPROM Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeprot::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeprot::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeprot`]
module"]
pub type EEPROT = crate::Reg<eeprot::EEPROT_SPEC>;
#[doc = "EEPROM Protection"]
pub mod eeprot;
#[doc = "EEPASS0 (rw) register accessor: EEPROM Password\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eepass0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eepass0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eepass0`]
module"]
pub type EEPASS0 = crate::Reg<eepass0::EEPASS0_SPEC>;
#[doc = "EEPROM Password"]
pub mod eepass0;
#[doc = "EEPASS1 (rw) register accessor: EEPROM Password\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eepass1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eepass1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eepass1`]
module"]
pub type EEPASS1 = crate::Reg<eepass1::EEPASS1_SPEC>;
#[doc = "EEPROM Password"]
pub mod eepass1;
#[doc = "EEPASS2 (rw) register accessor: EEPROM Password\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eepass2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eepass2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eepass2`]
module"]
pub type EEPASS2 = crate::Reg<eepass2::EEPASS2_SPEC>;
#[doc = "EEPROM Password"]
pub mod eepass2;
#[doc = "EEINT (rw) register accessor: EEPROM Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeint`]
module"]
pub type EEINT = crate::Reg<eeint::EEINT_SPEC>;
#[doc = "EEPROM Interrupt"]
pub mod eeint;
#[doc = "EEHIDE (rw) register accessor: EEPROM Block Hide\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eehide::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eehide::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eehide`]
module"]
pub type EEHIDE = crate::Reg<eehide::EEHIDE_SPEC>;
#[doc = "EEPROM Block Hide"]
pub mod eehide;
#[doc = "EEDBGME (rw) register accessor: EEPROM Debug Mass Erase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eedbgme::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eedbgme::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eedbgme`]
module"]
pub type EEDBGME = crate::Reg<eedbgme::EEDBGME_SPEC>;
#[doc = "EEPROM Debug Mass Erase"]
pub mod eedbgme;
#[doc = "PP (rw) register accessor: EEPROM Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp`]
module"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "EEPROM Peripheral Properties"]
pub mod pp;
