#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x03fc],
    #[doc = "0x3fc - GPIO Data"]
    pub data: DATA,
    #[doc = "0x400 - GPIO Direction"]
    pub dir: DIR,
    #[doc = "0x404 - GPIO Interrupt Sense"]
    pub is: IS,
    #[doc = "0x408 - GPIO Interrupt Both Edges"]
    pub ibe: IBE,
    #[doc = "0x40c - GPIO Interrupt Event"]
    pub iev: IEV,
    #[doc = "0x410 - GPIO Interrupt Mask"]
    pub im: IM,
    #[doc = "0x414 - GPIO Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x418 - GPIO Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x41c - GPIO Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x420 - GPIO Alternate Function Select"]
    pub afsel: AFSEL,
    _reserved10: [u8; 0xdc],
    #[doc = "0x500 - GPIO 2-mA Drive Select"]
    pub dr2r: DR2R,
    #[doc = "0x504 - GPIO 4-mA Drive Select"]
    pub dr4r: DR4R,
    #[doc = "0x508 - GPIO 8-mA Drive Select"]
    pub dr8r: DR8R,
    #[doc = "0x50c - GPIO Open Drain Select"]
    pub odr: ODR,
    #[doc = "0x510 - GPIO Pull-Up Select"]
    pub pur: PUR,
    #[doc = "0x514 - GPIO Pull-Down Select"]
    pub pdr: PDR,
    #[doc = "0x518 - GPIO Slew Rate Control Select"]
    pub slr: SLR,
    #[doc = "0x51c - GPIO Digital Enable"]
    pub den: DEN,
    #[doc = "0x520 - GPIO Lock"]
    pub lock: LOCK,
    #[doc = "0x524 - GPIO Commit"]
    pub cr: CR,
    #[doc = "0x528 - GPIO Analog Mode Select"]
    pub amsel: AMSEL,
    #[doc = "0x52c - GPIO Port Control"]
    pub pctl: PCTL,
    #[doc = "0x530 - GPIO ADC Control"]
    pub adcctl: ADCCTL,
    #[doc = "0x534 - GPIO DMA Control"]
    pub dmactl: DMACTL,
}
#[doc = "DATA (rw) register accessor: GPIO Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "GPIO Data"]
pub mod data;
#[doc = "DIR (rw) register accessor: GPIO Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "GPIO Direction"]
pub mod dir;
#[doc = "IS (rw) register accessor: GPIO Interrupt Sense\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is`]
module"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "GPIO Interrupt Sense"]
pub mod is;
#[doc = "IBE (rw) register accessor: GPIO Interrupt Both Edges\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibe`]
module"]
pub type IBE = crate::Reg<ibe::IBE_SPEC>;
#[doc = "GPIO Interrupt Both Edges"]
pub mod ibe;
#[doc = "IEV (rw) register accessor: GPIO Interrupt Event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iev`]
module"]
pub type IEV = crate::Reg<iev::IEV_SPEC>;
#[doc = "GPIO Interrupt Event"]
pub mod iev;
#[doc = "IM (rw) register accessor: GPIO Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`]
module"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "GPIO Interrupt Mask"]
pub mod im;
#[doc = "RIS (rw) register accessor: GPIO Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "GPIO Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS (rw) register accessor: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "GPIO Masked Interrupt Status"]
pub mod mis;
#[doc = "ICR (w) register accessor: GPIO Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "GPIO Interrupt Clear"]
pub mod icr;
#[doc = "AFSEL (rw) register accessor: GPIO Alternate Function Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afsel`]
module"]
pub type AFSEL = crate::Reg<afsel::AFSEL_SPEC>;
#[doc = "GPIO Alternate Function Select"]
pub mod afsel;
#[doc = "DR2R (rw) register accessor: GPIO 2-mA Drive Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2r`]
module"]
pub type DR2R = crate::Reg<dr2r::DR2R_SPEC>;
#[doc = "GPIO 2-mA Drive Select"]
pub mod dr2r;
#[doc = "DR4R (rw) register accessor: GPIO 4-mA Drive Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4r`]
module"]
pub type DR4R = crate::Reg<dr4r::DR4R_SPEC>;
#[doc = "GPIO 4-mA Drive Select"]
pub mod dr4r;
#[doc = "DR8R (rw) register accessor: GPIO 8-mA Drive Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr8r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr8r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8r`]
module"]
pub type DR8R = crate::Reg<dr8r::DR8R_SPEC>;
#[doc = "GPIO 8-mA Drive Select"]
pub mod dr8r;
#[doc = "ODR (rw) register accessor: GPIO Open Drain Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "GPIO Open Drain Select"]
pub mod odr;
#[doc = "PUR (rw) register accessor: GPIO Pull-Up Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pur`]
module"]
pub type PUR = crate::Reg<pur::PUR_SPEC>;
#[doc = "GPIO Pull-Up Select"]
pub mod pur;
#[doc = "PDR (rw) register accessor: GPIO Pull-Down Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr`]
module"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "GPIO Pull-Down Select"]
pub mod pdr;
#[doc = "SLR (rw) register accessor: GPIO Slew Rate Control Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slr`]
module"]
pub type SLR = crate::Reg<slr::SLR_SPEC>;
#[doc = "GPIO Slew Rate Control Select"]
pub mod slr;
#[doc = "DEN (rw) register accessor: GPIO Digital Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`den::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`den::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@den`]
module"]
pub type DEN = crate::Reg<den::DEN_SPEC>;
#[doc = "GPIO Digital Enable"]
pub mod den;
#[doc = "LOCK (rw) register accessor: GPIO Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "GPIO Lock"]
pub mod lock;
#[doc = "CR (r) register accessor: GPIO Commit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "GPIO Commit"]
pub mod cr;
#[doc = "AMSEL (rw) register accessor: GPIO Analog Mode Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amsel`]
module"]
pub type AMSEL = crate::Reg<amsel::AMSEL_SPEC>;
#[doc = "GPIO Analog Mode Select"]
pub mod amsel;
#[doc = "PCTL (rw) register accessor: GPIO Port Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pctl`]
module"]
pub type PCTL = crate::Reg<pctl::PCTL_SPEC>;
#[doc = "GPIO Port Control"]
pub mod pctl;
#[doc = "ADCCTL (rw) register accessor: GPIO ADC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl`]
module"]
pub type ADCCTL = crate::Reg<adcctl::ADCCTL_SPEC>;
#[doc = "GPIO ADC Control"]
pub mod adcctl;
#[doc = "DMACTL (rw) register accessor: GPIO DMA Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl`]
module"]
pub type DMACTL = crate::Reg<dmactl::DMACTL_SPEC>;
#[doc = "GPIO DMA Control"]
pub mod dmactl;
