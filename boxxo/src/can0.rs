#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Control"]
    pub ctl: CTL,
    #[doc = "0x04 - CAN Status"]
    pub sts: STS,
    #[doc = "0x08 - CAN Error Counter"]
    pub err: ERR,
    #[doc = "0x0c - CAN Bit Timing"]
    pub bit_: BIT,
    #[doc = "0x10 - CAN Interrupt"]
    pub int: INT,
    #[doc = "0x14 - CAN Test"]
    pub tst: TST,
    #[doc = "0x18 - CAN Baud Rate Prescaler Extension"]
    pub brpe: BRPE,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - CAN IF1 Command Request"]
    pub if1crq: IF1CRQ,
    _reserved_8_if1cmsk: [u8; 0x04],
    #[doc = "0x28 - CAN IF1 Mask 1"]
    pub if1msk1: IF1MSK1,
    #[doc = "0x2c - CAN IF1 Mask 2"]
    pub if1msk2: IF1MSK2,
    #[doc = "0x30 - CAN IF1 Arbitration 1"]
    pub if1arb1: IF1ARB1,
    #[doc = "0x34 - CAN IF1 Arbitration 2"]
    pub if1arb2: IF1ARB2,
    #[doc = "0x38 - CAN IF1 Message Control"]
    pub if1mctl: IF1MCTL,
    #[doc = "0x3c - CAN IF1 Data A1"]
    pub if1da1: IF1DA1,
    #[doc = "0x40 - CAN IF1 Data A2"]
    pub if1da2: IF1DA2,
    #[doc = "0x44 - CAN IF1 Data B1"]
    pub if1db1: IF1DB1,
    #[doc = "0x48 - CAN IF1 Data B2"]
    pub if1db2: IF1DB2,
    _reserved18: [u8; 0x34],
    #[doc = "0x80 - CAN IF2 Command Request"]
    pub if2crq: IF2CRQ,
    _reserved_19_if2cmsk: [u8; 0x04],
    #[doc = "0x88 - CAN IF2 Mask 1"]
    pub if2msk1: IF2MSK1,
    #[doc = "0x8c - CAN IF2 Mask 2"]
    pub if2msk2: IF2MSK2,
    #[doc = "0x90 - CAN IF2 Arbitration 1"]
    pub if2arb1: IF2ARB1,
    #[doc = "0x94 - CAN IF2 Arbitration 2"]
    pub if2arb2: IF2ARB2,
    #[doc = "0x98 - CAN IF2 Message Control"]
    pub if2mctl: IF2MCTL,
    #[doc = "0x9c - CAN IF2 Data A1"]
    pub if2da1: IF2DA1,
    #[doc = "0xa0 - CAN IF2 Data A2"]
    pub if2da2: IF2DA2,
    #[doc = "0xa4 - CAN IF2 Data B1"]
    pub if2db1: IF2DB1,
    #[doc = "0xa8 - CAN IF2 Data B2"]
    pub if2db2: IF2DB2,
    _reserved29: [u8; 0x54],
    #[doc = "0x100 - CAN Transmission Request 1"]
    pub txrq1: TXRQ1,
    #[doc = "0x104 - CAN Transmission Request 2"]
    pub txrq2: TXRQ2,
    _reserved31: [u8; 0x18],
    #[doc = "0x120 - CAN New Data 1"]
    pub nwda1: NWDA1,
    #[doc = "0x124 - CAN New Data 2"]
    pub nwda2: NWDA2,
    _reserved33: [u8; 0x18],
    #[doc = "0x140 - CAN Message 1 Interrupt Pending"]
    pub msg1int: MSG1INT,
    #[doc = "0x144 - CAN Message 2 Interrupt Pending"]
    pub msg2int: MSG2INT,
    _reserved35: [u8; 0x18],
    #[doc = "0x160 - CAN Message 1 Valid"]
    pub msg1val: MSG1VAL,
    #[doc = "0x164 - CAN Message 2 Valid"]
    pub msg2val: MSG2VAL,
}
impl RegisterBlock {
    #[doc = "0x24 - CAN IF1 Command Mask"]
    #[inline(always)]
    pub const fn can0_alt_if1cmsk(&self) -> &CAN0_ALT_IF1CMSK {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x24 - CAN IF1 Command Mask"]
    #[inline(always)]
    pub const fn if1cmsk(&self) -> &IF1CMSK {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x84 - CAN IF2 Command Mask"]
    #[inline(always)]
    pub const fn can0_alt_if2cmsk(&self) -> &CAN0_ALT_IF2CMSK {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
    #[doc = "0x84 - CAN IF2 Command Mask"]
    #[inline(always)]
    pub const fn if2cmsk(&self) -> &IF2CMSK {
        unsafe { &*(self as *const Self).cast::<u8>().add(132usize).cast() }
    }
}
#[doc = "CTL (rw) register accessor: CAN Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "CAN Control"]
pub mod ctl;
#[doc = "STS (rw) register accessor: CAN Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "CAN Status"]
pub mod sts;
#[doc = "ERR (rw) register accessor: CAN Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err`]
module"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "CAN Error Counter"]
pub mod err;
#[doc = "BIT (rw) register accessor: CAN Bit Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bit_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bit_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bit_`]
module"]
pub type BIT = crate::Reg<bit_::BIT_SPEC>;
#[doc = "CAN Bit Timing"]
pub mod bit_;
#[doc = "INT (rw) register accessor: CAN Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "CAN Interrupt"]
pub mod int;
#[doc = "TST (rw) register accessor: CAN Test\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tst`]
module"]
pub type TST = crate::Reg<tst::TST_SPEC>;
#[doc = "CAN Test"]
pub mod tst;
#[doc = "BRPE (rw) register accessor: CAN Baud Rate Prescaler Extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brpe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brpe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brpe`]
module"]
pub type BRPE = crate::Reg<brpe::BRPE_SPEC>;
#[doc = "CAN Baud Rate Prescaler Extension"]
pub mod brpe;
#[doc = "IF1CRQ (rw) register accessor: CAN IF1 Command Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1crq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1crq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1crq`]
module"]
pub type IF1CRQ = crate::Reg<if1crq::IF1CRQ_SPEC>;
#[doc = "CAN IF1 Command Request"]
pub mod if1crq;
#[doc = "IF1CMSK (rw) register accessor: CAN IF1 Command Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1cmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1cmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1cmsk`]
module"]
pub type IF1CMSK = crate::Reg<if1cmsk::IF1CMSK_SPEC>;
#[doc = "CAN IF1 Command Mask"]
pub mod if1cmsk;
#[doc = "CAN0_ALT_IF1CMSK (rw) register accessor: CAN IF1 Command Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can0_alt_if1cmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can0_alt_if1cmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can0_alt_if1cmsk`]
module"]
pub type CAN0_ALT_IF1CMSK = crate::Reg<can0_alt_if1cmsk::CAN0_ALT_IF1CMSK_SPEC>;
#[doc = "CAN IF1 Command Mask"]
pub mod can0_alt_if1cmsk;
#[doc = "IF1MSK1 (rw) register accessor: CAN IF1 Mask 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1msk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1msk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1msk1`]
module"]
pub type IF1MSK1 = crate::Reg<if1msk1::IF1MSK1_SPEC>;
#[doc = "CAN IF1 Mask 1"]
pub mod if1msk1;
#[doc = "IF1MSK2 (rw) register accessor: CAN IF1 Mask 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1msk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1msk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1msk2`]
module"]
pub type IF1MSK2 = crate::Reg<if1msk2::IF1MSK2_SPEC>;
#[doc = "CAN IF1 Mask 2"]
pub mod if1msk2;
#[doc = "IF1ARB1 (rw) register accessor: CAN IF1 Arbitration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1arb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1arb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1arb1`]
module"]
pub type IF1ARB1 = crate::Reg<if1arb1::IF1ARB1_SPEC>;
#[doc = "CAN IF1 Arbitration 1"]
pub mod if1arb1;
#[doc = "IF1ARB2 (rw) register accessor: CAN IF1 Arbitration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1arb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1arb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1arb2`]
module"]
pub type IF1ARB2 = crate::Reg<if1arb2::IF1ARB2_SPEC>;
#[doc = "CAN IF1 Arbitration 2"]
pub mod if1arb2;
#[doc = "IF1MCTL (rw) register accessor: CAN IF1 Message Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1mctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1mctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1mctl`]
module"]
pub type IF1MCTL = crate::Reg<if1mctl::IF1MCTL_SPEC>;
#[doc = "CAN IF1 Message Control"]
pub mod if1mctl;
#[doc = "IF1DA1 (rw) register accessor: CAN IF1 Data A1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1da1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1da1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1da1`]
module"]
pub type IF1DA1 = crate::Reg<if1da1::IF1DA1_SPEC>;
#[doc = "CAN IF1 Data A1"]
pub mod if1da1;
#[doc = "IF1DA2 (rw) register accessor: CAN IF1 Data A2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1da2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1da2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1da2`]
module"]
pub type IF1DA2 = crate::Reg<if1da2::IF1DA2_SPEC>;
#[doc = "CAN IF1 Data A2"]
pub mod if1da2;
#[doc = "IF1DB1 (rw) register accessor: CAN IF1 Data B1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1db1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1db1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1db1`]
module"]
pub type IF1DB1 = crate::Reg<if1db1::IF1DB1_SPEC>;
#[doc = "CAN IF1 Data B1"]
pub mod if1db1;
#[doc = "IF1DB2 (rw) register accessor: CAN IF1 Data B2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1db2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1db2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1db2`]
module"]
pub type IF1DB2 = crate::Reg<if1db2::IF1DB2_SPEC>;
#[doc = "CAN IF1 Data B2"]
pub mod if1db2;
#[doc = "IF2CRQ (rw) register accessor: CAN IF2 Command Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2crq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2crq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2crq`]
module"]
pub type IF2CRQ = crate::Reg<if2crq::IF2CRQ_SPEC>;
#[doc = "CAN IF2 Command Request"]
pub mod if2crq;
#[doc = "IF2CMSK (rw) register accessor: CAN IF2 Command Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2cmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2cmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2cmsk`]
module"]
pub type IF2CMSK = crate::Reg<if2cmsk::IF2CMSK_SPEC>;
#[doc = "CAN IF2 Command Mask"]
pub mod if2cmsk;
#[doc = "CAN0_ALT_IF2CMSK (rw) register accessor: CAN IF2 Command Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can0_alt_if2cmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can0_alt_if2cmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can0_alt_if2cmsk`]
module"]
pub type CAN0_ALT_IF2CMSK = crate::Reg<can0_alt_if2cmsk::CAN0_ALT_IF2CMSK_SPEC>;
#[doc = "CAN IF2 Command Mask"]
pub mod can0_alt_if2cmsk;
#[doc = "IF2MSK1 (rw) register accessor: CAN IF2 Mask 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2msk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2msk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2msk1`]
module"]
pub type IF2MSK1 = crate::Reg<if2msk1::IF2MSK1_SPEC>;
#[doc = "CAN IF2 Mask 1"]
pub mod if2msk1;
#[doc = "IF2MSK2 (rw) register accessor: CAN IF2 Mask 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2msk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2msk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2msk2`]
module"]
pub type IF2MSK2 = crate::Reg<if2msk2::IF2MSK2_SPEC>;
#[doc = "CAN IF2 Mask 2"]
pub mod if2msk2;
#[doc = "IF2ARB1 (rw) register accessor: CAN IF2 Arbitration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2arb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2arb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2arb1`]
module"]
pub type IF2ARB1 = crate::Reg<if2arb1::IF2ARB1_SPEC>;
#[doc = "CAN IF2 Arbitration 1"]
pub mod if2arb1;
#[doc = "IF2ARB2 (rw) register accessor: CAN IF2 Arbitration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2arb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2arb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2arb2`]
module"]
pub type IF2ARB2 = crate::Reg<if2arb2::IF2ARB2_SPEC>;
#[doc = "CAN IF2 Arbitration 2"]
pub mod if2arb2;
#[doc = "IF2MCTL (rw) register accessor: CAN IF2 Message Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2mctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2mctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2mctl`]
module"]
pub type IF2MCTL = crate::Reg<if2mctl::IF2MCTL_SPEC>;
#[doc = "CAN IF2 Message Control"]
pub mod if2mctl;
#[doc = "IF2DA1 (rw) register accessor: CAN IF2 Data A1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2da1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2da1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2da1`]
module"]
pub type IF2DA1 = crate::Reg<if2da1::IF2DA1_SPEC>;
#[doc = "CAN IF2 Data A1"]
pub mod if2da1;
#[doc = "IF2DA2 (rw) register accessor: CAN IF2 Data A2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2da2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2da2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2da2`]
module"]
pub type IF2DA2 = crate::Reg<if2da2::IF2DA2_SPEC>;
#[doc = "CAN IF2 Data A2"]
pub mod if2da2;
#[doc = "IF2DB1 (rw) register accessor: CAN IF2 Data B1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2db1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2db1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2db1`]
module"]
pub type IF2DB1 = crate::Reg<if2db1::IF2DB1_SPEC>;
#[doc = "CAN IF2 Data B1"]
pub mod if2db1;
#[doc = "IF2DB2 (rw) register accessor: CAN IF2 Data B2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2db2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2db2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if2db2`]
module"]
pub type IF2DB2 = crate::Reg<if2db2::IF2DB2_SPEC>;
#[doc = "CAN IF2 Data B2"]
pub mod if2db2;
#[doc = "TXRQ1 (rw) register accessor: CAN Transmission Request 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txrq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txrq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txrq1`]
module"]
pub type TXRQ1 = crate::Reg<txrq1::TXRQ1_SPEC>;
#[doc = "CAN Transmission Request 1"]
pub mod txrq1;
#[doc = "TXRQ2 (rw) register accessor: CAN Transmission Request 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txrq2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txrq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txrq2`]
module"]
pub type TXRQ2 = crate::Reg<txrq2::TXRQ2_SPEC>;
#[doc = "CAN Transmission Request 2"]
pub mod txrq2;
#[doc = "NWDA1 (rw) register accessor: CAN New Data 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nwda1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nwda1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nwda1`]
module"]
pub type NWDA1 = crate::Reg<nwda1::NWDA1_SPEC>;
#[doc = "CAN New Data 1"]
pub mod nwda1;
#[doc = "NWDA2 (rw) register accessor: CAN New Data 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nwda2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nwda2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nwda2`]
module"]
pub type NWDA2 = crate::Reg<nwda2::NWDA2_SPEC>;
#[doc = "CAN New Data 2"]
pub mod nwda2;
#[doc = "MSG1INT (rw) register accessor: CAN Message 1 Interrupt Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg1int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg1int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg1int`]
module"]
pub type MSG1INT = crate::Reg<msg1int::MSG1INT_SPEC>;
#[doc = "CAN Message 1 Interrupt Pending"]
pub mod msg1int;
#[doc = "MSG2INT (rw) register accessor: CAN Message 2 Interrupt Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg2int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg2int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg2int`]
module"]
pub type MSG2INT = crate::Reg<msg2int::MSG2INT_SPEC>;
#[doc = "CAN Message 2 Interrupt Pending"]
pub mod msg2int;
#[doc = "MSG1VAL (rw) register accessor: CAN Message 1 Valid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg1val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg1val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg1val`]
module"]
pub type MSG1VAL = crate::Reg<msg1val::MSG1VAL_SPEC>;
#[doc = "CAN Message 1 Valid"]
pub mod msg1val;
#[doc = "MSG2VAL (rw) register accessor: CAN Message 2 Valid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg2val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg2val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg2val`]
module"]
pub type MSG2VAL = crate::Reg<msg2val::MSG2VAL_SPEC>;
#[doc = "CAN Message 2 Valid"]
pub mod msg2val;
