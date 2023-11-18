#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Active Sample Sequencer"]
    pub actss: ACTSS,
    #[doc = "0x04 - ADC Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x08 - ADC Interrupt Mask"]
    pub im: IM,
    #[doc = "0x0c - ADC Interrupt Status and Clear"]
    pub isc: ISC,
    #[doc = "0x10 - ADC Overflow Status"]
    pub ostat: OSTAT,
    #[doc = "0x14 - ADC Event Multiplexer Select"]
    pub emux: EMUX,
    #[doc = "0x18 - ADC Underflow Status"]
    pub ustat: USTAT,
    #[doc = "0x1c - ADC Trigger Source Select"]
    pub tssel: TSSEL,
    #[doc = "0x20 - ADC Sample Sequencer Priority"]
    pub sspri: SSPRI,
    #[doc = "0x24 - ADC Sample Phase Control"]
    pub spc: SPC,
    #[doc = "0x28 - ADC Processor Sample Sequence Initiate"]
    pub pssi: PSSI,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - ADC Sample Averaging Control"]
    pub sac: SAC,
    #[doc = "0x34 - ADC Digital Comparator Interrupt Status and Clear"]
    pub dcisc: DCISC,
    #[doc = "0x38 - ADC Control"]
    pub ctl: CTL,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - ADC Sample Sequence Input Multiplexer Select 0"]
    pub ssmux0: SSMUX0,
    #[doc = "0x44 - ADC Sample Sequence Control 0"]
    pub ssctl0: SSCTL0,
    #[doc = "0x48 - ADC Sample Sequence Result FIFO 0"]
    pub ssfifo0: SSFIFO0,
    #[doc = "0x4c - ADC Sample Sequence FIFO 0 Status"]
    pub ssfstat0: SSFSTAT0,
    #[doc = "0x50 - ADC Sample Sequence 0 Operation"]
    pub ssop0: SSOP0,
    #[doc = "0x54 - ADC Sample Sequence 0 Digital Comparator Select"]
    pub ssdc0: SSDC0,
    _reserved20: [u8; 0x08],
    #[doc = "0x60 - ADC Sample Sequence Input Multiplexer Select 1"]
    pub ssmux1: SSMUX1,
    #[doc = "0x64 - ADC Sample Sequence Control 1"]
    pub ssctl1: SSCTL1,
    #[doc = "0x68 - ADC Sample Sequence Result FIFO 1"]
    pub ssfifo1: SSFIFO1,
    #[doc = "0x6c - ADC Sample Sequence FIFO 1 Status"]
    pub ssfstat1: SSFSTAT1,
    #[doc = "0x70 - ADC Sample Sequence 1 Operation"]
    pub ssop1: SSOP1,
    #[doc = "0x74 - ADC Sample Sequence 1 Digital Comparator Select"]
    pub ssdc1: SSDC1,
    _reserved26: [u8; 0x08],
    #[doc = "0x80 - ADC Sample Sequence Input Multiplexer Select 2"]
    pub ssmux2: SSMUX2,
    #[doc = "0x84 - ADC Sample Sequence Control 2"]
    pub ssctl2: SSCTL2,
    #[doc = "0x88 - ADC Sample Sequence Result FIFO 2"]
    pub ssfifo2: SSFIFO2,
    #[doc = "0x8c - ADC Sample Sequence FIFO 2 Status"]
    pub ssfstat2: SSFSTAT2,
    #[doc = "0x90 - ADC Sample Sequence 2 Operation"]
    pub ssop2: SSOP2,
    #[doc = "0x94 - ADC Sample Sequence 2 Digital Comparator Select"]
    pub ssdc2: SSDC2,
    _reserved32: [u8; 0x08],
    #[doc = "0xa0 - ADC Sample Sequence Input Multiplexer Select 3"]
    pub ssmux3: SSMUX3,
    #[doc = "0xa4 - ADC Sample Sequence Control 3"]
    pub ssctl3: SSCTL3,
    #[doc = "0xa8 - ADC Sample Sequence Result FIFO 3"]
    pub ssfifo3: SSFIFO3,
    #[doc = "0xac - ADC Sample Sequence FIFO 3 Status"]
    pub ssfstat3: SSFSTAT3,
    #[doc = "0xb0 - ADC Sample Sequence 3 Operation"]
    pub ssop3: SSOP3,
    #[doc = "0xb4 - ADC Sample Sequence 3 Digital Comparator Select"]
    pub ssdc3: SSDC3,
    _reserved38: [u8; 0x0c48],
    #[doc = "0xd00 - ADC Digital Comparator Reset Initial Conditions"]
    pub dcric: DCRIC,
    _reserved39: [u8; 0xfc],
    #[doc = "0xe00 - ADC Digital Comparator Control 0"]
    pub dcctl0: DCCTL0,
    #[doc = "0xe04 - ADC Digital Comparator Control 1"]
    pub dcctl1: DCCTL1,
    #[doc = "0xe08 - ADC Digital Comparator Control 2"]
    pub dcctl2: DCCTL2,
    #[doc = "0xe0c - ADC Digital Comparator Control 3"]
    pub dcctl3: DCCTL3,
    #[doc = "0xe10 - ADC Digital Comparator Control 4"]
    pub dcctl4: DCCTL4,
    #[doc = "0xe14 - ADC Digital Comparator Control 5"]
    pub dcctl5: DCCTL5,
    #[doc = "0xe18 - ADC Digital Comparator Control 6"]
    pub dcctl6: DCCTL6,
    #[doc = "0xe1c - ADC Digital Comparator Control 7"]
    pub dcctl7: DCCTL7,
    _reserved47: [u8; 0x20],
    #[doc = "0xe40 - ADC Digital Comparator Range 0"]
    pub dccmp0: DCCMP0,
    #[doc = "0xe44 - ADC Digital Comparator Range 1"]
    pub dccmp1: DCCMP1,
    #[doc = "0xe48 - ADC Digital Comparator Range 2"]
    pub dccmp2: DCCMP2,
    #[doc = "0xe4c - ADC Digital Comparator Range 3"]
    pub dccmp3: DCCMP3,
    #[doc = "0xe50 - ADC Digital Comparator Range 4"]
    pub dccmp4: DCCMP4,
    #[doc = "0xe54 - ADC Digital Comparator Range 5"]
    pub dccmp5: DCCMP5,
    #[doc = "0xe58 - ADC Digital Comparator Range 6"]
    pub dccmp6: DCCMP6,
    #[doc = "0xe5c - ADC Digital Comparator Range 7"]
    pub dccmp7: DCCMP7,
    _reserved55: [u8; 0x0160],
    #[doc = "0xfc0 - ADC Peripheral Properties"]
    pub pp: PP,
    #[doc = "0xfc4 - ADC Peripheral Configuration"]
    pub pc: PC,
    #[doc = "0xfc8 - ADC Clock Configuration"]
    pub cc: CC,
}
#[doc = "ACTSS (rw) register accessor: ADC Active Sample Sequencer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actss`]
module"]
pub type ACTSS = crate::Reg<actss::ACTSS_SPEC>;
#[doc = "ADC Active Sample Sequencer"]
pub mod actss;
#[doc = "RIS (rw) register accessor: ADC Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "ADC Raw Interrupt Status"]
pub mod ris;
#[doc = "IM (rw) register accessor: ADC Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`]
module"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "ADC Interrupt Mask"]
pub mod im;
#[doc = "ISC (rw) register accessor: ADC Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc`]
module"]
pub type ISC = crate::Reg<isc::ISC_SPEC>;
#[doc = "ADC Interrupt Status and Clear"]
pub mod isc;
#[doc = "OSTAT (rw) register accessor: ADC Overflow Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ostat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ostat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ostat`]
module"]
pub type OSTAT = crate::Reg<ostat::OSTAT_SPEC>;
#[doc = "ADC Overflow Status"]
pub mod ostat;
#[doc = "EMUX (rw) register accessor: ADC Event Multiplexer Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emux`]
module"]
pub type EMUX = crate::Reg<emux::EMUX_SPEC>;
#[doc = "ADC Event Multiplexer Select"]
pub mod emux;
#[doc = "USTAT (rw) register accessor: ADC Underflow Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ustat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ustat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ustat`]
module"]
pub type USTAT = crate::Reg<ustat::USTAT_SPEC>;
#[doc = "ADC Underflow Status"]
pub mod ustat;
#[doc = "TSSEL (rw) register accessor: ADC Trigger Source Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tssel`]
module"]
pub type TSSEL = crate::Reg<tssel::TSSEL_SPEC>;
#[doc = "ADC Trigger Source Select"]
pub mod tssel;
#[doc = "SSPRI (rw) register accessor: ADC Sample Sequencer Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sspri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspri`]
module"]
pub type SSPRI = crate::Reg<sspri::SSPRI_SPEC>;
#[doc = "ADC Sample Sequencer Priority"]
pub mod sspri;
#[doc = "SPC (rw) register accessor: ADC Sample Phase Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spc`]
module"]
pub type SPC = crate::Reg<spc::SPC_SPEC>;
#[doc = "ADC Sample Phase Control"]
pub mod spc;
#[doc = "PSSI (rw) register accessor: ADC Processor Sample Sequence Initiate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pssi`]
module"]
pub type PSSI = crate::Reg<pssi::PSSI_SPEC>;
#[doc = "ADC Processor Sample Sequence Initiate"]
pub mod pssi;
#[doc = "SAC (rw) register accessor: ADC Sample Averaging Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac`]
module"]
pub type SAC = crate::Reg<sac::SAC_SPEC>;
#[doc = "ADC Sample Averaging Control"]
pub mod sac;
#[doc = "DCISC (rw) register accessor: ADC Digital Comparator Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcisc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcisc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcisc`]
module"]
pub type DCISC = crate::Reg<dcisc::DCISC_SPEC>;
#[doc = "ADC Digital Comparator Interrupt Status and Clear"]
pub mod dcisc;
#[doc = "CTL (rw) register accessor: ADC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "ADC Control"]
pub mod ctl;
#[doc = "SSMUX0 (rw) register accessor: ADC Sample Sequence Input Multiplexer Select 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssmux0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssmux0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssmux0`]
module"]
pub type SSMUX0 = crate::Reg<ssmux0::SSMUX0_SPEC>;
#[doc = "ADC Sample Sequence Input Multiplexer Select 0"]
pub mod ssmux0;
#[doc = "SSCTL0 (rw) register accessor: ADC Sample Sequence Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssctl0`]
module"]
pub type SSCTL0 = crate::Reg<ssctl0::SSCTL0_SPEC>;
#[doc = "ADC Sample Sequence Control 0"]
pub mod ssctl0;
#[doc = "SSFIFO0 (rw) register accessor: ADC Sample Sequence Result FIFO 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfifo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfifo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssfifo0`]
module"]
pub type SSFIFO0 = crate::Reg<ssfifo0::SSFIFO0_SPEC>;
#[doc = "ADC Sample Sequence Result FIFO 0"]
pub mod ssfifo0;
#[doc = "SSFSTAT0 (rw) register accessor: ADC Sample Sequence FIFO 0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfstat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfstat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssfstat0`]
module"]
pub type SSFSTAT0 = crate::Reg<ssfstat0::SSFSTAT0_SPEC>;
#[doc = "ADC Sample Sequence FIFO 0 Status"]
pub mod ssfstat0;
#[doc = "SSOP0 (rw) register accessor: ADC Sample Sequence 0 Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssop0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssop0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssop0`]
module"]
pub type SSOP0 = crate::Reg<ssop0::SSOP0_SPEC>;
#[doc = "ADC Sample Sequence 0 Operation"]
pub mod ssop0;
#[doc = "SSDC0 (rw) register accessor: ADC Sample Sequence 0 Digital Comparator Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssdc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssdc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssdc0`]
module"]
pub type SSDC0 = crate::Reg<ssdc0::SSDC0_SPEC>;
#[doc = "ADC Sample Sequence 0 Digital Comparator Select"]
pub mod ssdc0;
#[doc = "SSMUX1 (rw) register accessor: ADC Sample Sequence Input Multiplexer Select 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssmux1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssmux1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssmux1`]
module"]
pub type SSMUX1 = crate::Reg<ssmux1::SSMUX1_SPEC>;
#[doc = "ADC Sample Sequence Input Multiplexer Select 1"]
pub mod ssmux1;
#[doc = "SSCTL1 (rw) register accessor: ADC Sample Sequence Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssctl1`]
module"]
pub type SSCTL1 = crate::Reg<ssctl1::SSCTL1_SPEC>;
#[doc = "ADC Sample Sequence Control 1"]
pub mod ssctl1;
#[doc = "SSFIFO1 (rw) register accessor: ADC Sample Sequence Result FIFO 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfifo1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfifo1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssfifo1`]
module"]
pub type SSFIFO1 = crate::Reg<ssfifo1::SSFIFO1_SPEC>;
#[doc = "ADC Sample Sequence Result FIFO 1"]
pub mod ssfifo1;
#[doc = "SSFSTAT1 (rw) register accessor: ADC Sample Sequence FIFO 1 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfstat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfstat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssfstat1`]
module"]
pub type SSFSTAT1 = crate::Reg<ssfstat1::SSFSTAT1_SPEC>;
#[doc = "ADC Sample Sequence FIFO 1 Status"]
pub mod ssfstat1;
#[doc = "SSOP1 (rw) register accessor: ADC Sample Sequence 1 Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssop1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssop1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssop1`]
module"]
pub type SSOP1 = crate::Reg<ssop1::SSOP1_SPEC>;
#[doc = "ADC Sample Sequence 1 Operation"]
pub mod ssop1;
#[doc = "SSDC1 (rw) register accessor: ADC Sample Sequence 1 Digital Comparator Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssdc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssdc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssdc1`]
module"]
pub type SSDC1 = crate::Reg<ssdc1::SSDC1_SPEC>;
#[doc = "ADC Sample Sequence 1 Digital Comparator Select"]
pub mod ssdc1;
#[doc = "SSMUX2 (rw) register accessor: ADC Sample Sequence Input Multiplexer Select 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssmux2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssmux2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssmux2`]
module"]
pub type SSMUX2 = crate::Reg<ssmux2::SSMUX2_SPEC>;
#[doc = "ADC Sample Sequence Input Multiplexer Select 2"]
pub mod ssmux2;
#[doc = "SSCTL2 (rw) register accessor: ADC Sample Sequence Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssctl2`]
module"]
pub type SSCTL2 = crate::Reg<ssctl2::SSCTL2_SPEC>;
#[doc = "ADC Sample Sequence Control 2"]
pub mod ssctl2;
#[doc = "SSFIFO2 (rw) register accessor: ADC Sample Sequence Result FIFO 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfifo2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfifo2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssfifo2`]
module"]
pub type SSFIFO2 = crate::Reg<ssfifo2::SSFIFO2_SPEC>;
#[doc = "ADC Sample Sequence Result FIFO 2"]
pub mod ssfifo2;
#[doc = "SSFSTAT2 (rw) register accessor: ADC Sample Sequence FIFO 2 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfstat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfstat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssfstat2`]
module"]
pub type SSFSTAT2 = crate::Reg<ssfstat2::SSFSTAT2_SPEC>;
#[doc = "ADC Sample Sequence FIFO 2 Status"]
pub mod ssfstat2;
#[doc = "SSOP2 (rw) register accessor: ADC Sample Sequence 2 Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssop2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssop2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssop2`]
module"]
pub type SSOP2 = crate::Reg<ssop2::SSOP2_SPEC>;
#[doc = "ADC Sample Sequence 2 Operation"]
pub mod ssop2;
#[doc = "SSDC2 (rw) register accessor: ADC Sample Sequence 2 Digital Comparator Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssdc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssdc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssdc2`]
module"]
pub type SSDC2 = crate::Reg<ssdc2::SSDC2_SPEC>;
#[doc = "ADC Sample Sequence 2 Digital Comparator Select"]
pub mod ssdc2;
#[doc = "SSMUX3 (rw) register accessor: ADC Sample Sequence Input Multiplexer Select 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssmux3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssmux3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssmux3`]
module"]
pub type SSMUX3 = crate::Reg<ssmux3::SSMUX3_SPEC>;
#[doc = "ADC Sample Sequence Input Multiplexer Select 3"]
pub mod ssmux3;
#[doc = "SSCTL3 (rw) register accessor: ADC Sample Sequence Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssctl3`]
module"]
pub type SSCTL3 = crate::Reg<ssctl3::SSCTL3_SPEC>;
#[doc = "ADC Sample Sequence Control 3"]
pub mod ssctl3;
#[doc = "SSFIFO3 (rw) register accessor: ADC Sample Sequence Result FIFO 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfifo3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfifo3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssfifo3`]
module"]
pub type SSFIFO3 = crate::Reg<ssfifo3::SSFIFO3_SPEC>;
#[doc = "ADC Sample Sequence Result FIFO 3"]
pub mod ssfifo3;
#[doc = "SSFSTAT3 (rw) register accessor: ADC Sample Sequence FIFO 3 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfstat3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfstat3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssfstat3`]
module"]
pub type SSFSTAT3 = crate::Reg<ssfstat3::SSFSTAT3_SPEC>;
#[doc = "ADC Sample Sequence FIFO 3 Status"]
pub mod ssfstat3;
#[doc = "SSOP3 (rw) register accessor: ADC Sample Sequence 3 Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssop3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssop3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssop3`]
module"]
pub type SSOP3 = crate::Reg<ssop3::SSOP3_SPEC>;
#[doc = "ADC Sample Sequence 3 Operation"]
pub mod ssop3;
#[doc = "SSDC3 (rw) register accessor: ADC Sample Sequence 3 Digital Comparator Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssdc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssdc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssdc3`]
module"]
pub type SSDC3 = crate::Reg<ssdc3::SSDC3_SPEC>;
#[doc = "ADC Sample Sequence 3 Digital Comparator Select"]
pub mod ssdc3;
#[doc = "DCRIC (w) register accessor: ADC Digital Comparator Reset Initial Conditions\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcric::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcric`]
module"]
pub type DCRIC = crate::Reg<dcric::DCRIC_SPEC>;
#[doc = "ADC Digital Comparator Reset Initial Conditions"]
pub mod dcric;
#[doc = "DCCTL0 (rw) register accessor: ADC Digital Comparator Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcctl0`]
module"]
pub type DCCTL0 = crate::Reg<dcctl0::DCCTL0_SPEC>;
#[doc = "ADC Digital Comparator Control 0"]
pub mod dcctl0;
#[doc = "DCCTL1 (rw) register accessor: ADC Digital Comparator Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcctl1`]
module"]
pub type DCCTL1 = crate::Reg<dcctl1::DCCTL1_SPEC>;
#[doc = "ADC Digital Comparator Control 1"]
pub mod dcctl1;
#[doc = "DCCTL2 (rw) register accessor: ADC Digital Comparator Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcctl2`]
module"]
pub type DCCTL2 = crate::Reg<dcctl2::DCCTL2_SPEC>;
#[doc = "ADC Digital Comparator Control 2"]
pub mod dcctl2;
#[doc = "DCCTL3 (rw) register accessor: ADC Digital Comparator Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcctl3`]
module"]
pub type DCCTL3 = crate::Reg<dcctl3::DCCTL3_SPEC>;
#[doc = "ADC Digital Comparator Control 3"]
pub mod dcctl3;
#[doc = "DCCTL4 (rw) register accessor: ADC Digital Comparator Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcctl4`]
module"]
pub type DCCTL4 = crate::Reg<dcctl4::DCCTL4_SPEC>;
#[doc = "ADC Digital Comparator Control 4"]
pub mod dcctl4;
#[doc = "DCCTL5 (rw) register accessor: ADC Digital Comparator Control 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcctl5`]
module"]
pub type DCCTL5 = crate::Reg<dcctl5::DCCTL5_SPEC>;
#[doc = "ADC Digital Comparator Control 5"]
pub mod dcctl5;
#[doc = "DCCTL6 (rw) register accessor: ADC Digital Comparator Control 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcctl6`]
module"]
pub type DCCTL6 = crate::Reg<dcctl6::DCCTL6_SPEC>;
#[doc = "ADC Digital Comparator Control 6"]
pub mod dcctl6;
#[doc = "DCCTL7 (rw) register accessor: ADC Digital Comparator Control 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcctl7`]
module"]
pub type DCCTL7 = crate::Reg<dcctl7::DCCTL7_SPEC>;
#[doc = "ADC Digital Comparator Control 7"]
pub mod dcctl7;
#[doc = "DCCMP0 (rw) register accessor: ADC Digital Comparator Range 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccmp0`]
module"]
pub type DCCMP0 = crate::Reg<dccmp0::DCCMP0_SPEC>;
#[doc = "ADC Digital Comparator Range 0"]
pub mod dccmp0;
#[doc = "DCCMP1 (rw) register accessor: ADC Digital Comparator Range 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccmp1`]
module"]
pub type DCCMP1 = crate::Reg<dccmp1::DCCMP1_SPEC>;
#[doc = "ADC Digital Comparator Range 1"]
pub mod dccmp1;
#[doc = "DCCMP2 (rw) register accessor: ADC Digital Comparator Range 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccmp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccmp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccmp2`]
module"]
pub type DCCMP2 = crate::Reg<dccmp2::DCCMP2_SPEC>;
#[doc = "ADC Digital Comparator Range 2"]
pub mod dccmp2;
#[doc = "DCCMP3 (rw) register accessor: ADC Digital Comparator Range 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccmp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccmp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccmp3`]
module"]
pub type DCCMP3 = crate::Reg<dccmp3::DCCMP3_SPEC>;
#[doc = "ADC Digital Comparator Range 3"]
pub mod dccmp3;
#[doc = "DCCMP4 (rw) register accessor: ADC Digital Comparator Range 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccmp4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccmp4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccmp4`]
module"]
pub type DCCMP4 = crate::Reg<dccmp4::DCCMP4_SPEC>;
#[doc = "ADC Digital Comparator Range 4"]
pub mod dccmp4;
#[doc = "DCCMP5 (rw) register accessor: ADC Digital Comparator Range 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccmp5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccmp5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccmp5`]
module"]
pub type DCCMP5 = crate::Reg<dccmp5::DCCMP5_SPEC>;
#[doc = "ADC Digital Comparator Range 5"]
pub mod dccmp5;
#[doc = "DCCMP6 (rw) register accessor: ADC Digital Comparator Range 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccmp6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccmp6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccmp6`]
module"]
pub type DCCMP6 = crate::Reg<dccmp6::DCCMP6_SPEC>;
#[doc = "ADC Digital Comparator Range 6"]
pub mod dccmp6;
#[doc = "DCCMP7 (rw) register accessor: ADC Digital Comparator Range 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccmp7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccmp7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccmp7`]
module"]
pub type DCCMP7 = crate::Reg<dccmp7::DCCMP7_SPEC>;
#[doc = "ADC Digital Comparator Range 7"]
pub mod dccmp7;
#[doc = "PP (rw) register accessor: ADC Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp`]
module"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "ADC Peripheral Properties"]
pub mod pp;
#[doc = "PC (rw) register accessor: ADC Peripheral Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc`]
module"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "ADC Peripheral Configuration"]
pub mod pc;
#[doc = "CC (rw) register accessor: ADC Clock Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`]
module"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "ADC Clock Configuration"]
pub mod cc;
