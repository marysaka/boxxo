#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Masked Interrupt Status"]
    pub acmis: ACMIS,
    #[doc = "0x04 - Analog Comparator Raw Interrupt Status"]
    pub acris: ACRIS,
    #[doc = "0x08 - Analog Comparator Interrupt Enable"]
    pub acinten: ACINTEN,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Analog Comparator Reference Voltage Control"]
    pub acrefctl: ACREFCTL,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - Analog Comparator Status 0"]
    pub acstat0: ACSTAT0,
    #[doc = "0x24 - Analog Comparator Control 0"]
    pub acctl0: ACCTL0,
    _reserved6: [u8; 0x18],
    #[doc = "0x40 - Analog Comparator Status 1"]
    pub acstat1: ACSTAT1,
    #[doc = "0x44 - Analog Comparator Control 1"]
    pub acctl1: ACCTL1,
    _reserved8: [u8; 0x0f78],
    #[doc = "0xfc0 - Analog Comparator Peripheral Properties"]
    pub pp: PP,
}
#[doc = "ACMIS (rw) register accessor: Analog Comparator Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acmis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acmis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmis`]
module"]
pub type ACMIS = crate::Reg<acmis::ACMIS_SPEC>;
#[doc = "Analog Comparator Masked Interrupt Status"]
pub mod acmis;
#[doc = "ACRIS (rw) register accessor: Analog Comparator Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acris`]
module"]
pub type ACRIS = crate::Reg<acris::ACRIS_SPEC>;
#[doc = "Analog Comparator Raw Interrupt Status"]
pub mod acris;
#[doc = "ACINTEN (rw) register accessor: Analog Comparator Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acinten`]
module"]
pub type ACINTEN = crate::Reg<acinten::ACINTEN_SPEC>;
#[doc = "Analog Comparator Interrupt Enable"]
pub mod acinten;
#[doc = "ACREFCTL (rw) register accessor: Analog Comparator Reference Voltage Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acrefctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acrefctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acrefctl`]
module"]
pub type ACREFCTL = crate::Reg<acrefctl::ACREFCTL_SPEC>;
#[doc = "Analog Comparator Reference Voltage Control"]
pub mod acrefctl;
#[doc = "ACSTAT0 (rw) register accessor: Analog Comparator Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acstat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acstat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acstat0`]
module"]
pub type ACSTAT0 = crate::Reg<acstat0::ACSTAT0_SPEC>;
#[doc = "Analog Comparator Status 0"]
pub mod acstat0;
#[doc = "ACCTL0 (rw) register accessor: Analog Comparator Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acctl0`]
module"]
pub type ACCTL0 = crate::Reg<acctl0::ACCTL0_SPEC>;
#[doc = "Analog Comparator Control 0"]
pub mod acctl0;
#[doc = "ACSTAT1 (rw) register accessor: Analog Comparator Status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acstat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acstat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acstat1`]
module"]
pub type ACSTAT1 = crate::Reg<acstat1::ACSTAT1_SPEC>;
#[doc = "Analog Comparator Status 1"]
pub mod acstat1;
#[doc = "ACCTL1 (rw) register accessor: Analog Comparator Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acctl1`]
module"]
pub type ACCTL1 = crate::Reg<acctl1::ACCTL1_SPEC>;
#[doc = "Analog Comparator Control 1"]
pub mod acctl1;
#[doc = "PP (rw) register accessor: Analog Comparator Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp`]
module"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "Analog Comparator Peripheral Properties"]
pub mod pp;
