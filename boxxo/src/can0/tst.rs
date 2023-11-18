#[doc = "Register `TST` reader"]
pub type R = crate::R<TST_SPEC>;
#[doc = "Register `TST` writer"]
pub type W = crate::W<TST_SPEC>;
#[doc = "Field `CAN_TST_BASIC` reader - Basic Mode"]
pub type CAN_TST_BASIC_R = crate::BitReader;
#[doc = "Field `CAN_TST_BASIC` writer - Basic Mode"]
pub type CAN_TST_BASIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_TST_SILENT` reader - Silent Mode"]
pub type CAN_TST_SILENT_R = crate::BitReader;
#[doc = "Field `CAN_TST_SILENT` writer - Silent Mode"]
pub type CAN_TST_SILENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_TST_LBACK` reader - Loopback Mode"]
pub type CAN_TST_LBACK_R = crate::BitReader;
#[doc = "Field `CAN_TST_LBACK` writer - Loopback Mode"]
pub type CAN_TST_LBACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_TST_TX` reader - Transmit Control"]
pub type CAN_TST_TX_R = crate::FieldReader<CAN_TST_TX_A>;
#[doc = "Transmit Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAN_TST_TX_A {
    #[doc = "0: CAN Module Control"]
    CAN_TST_TX_CANCTL = 0,
    #[doc = "1: Sample Point"]
    CAN_TST_TX_SAMPLE = 1,
    #[doc = "2: Driven Low"]
    CAN_TST_TX_DOMINANT = 2,
    #[doc = "3: Driven High"]
    CAN_TST_TX_RECESSIVE = 3,
}
impl From<CAN_TST_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: CAN_TST_TX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAN_TST_TX_A {
    type Ux = u8;
}
impl CAN_TST_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAN_TST_TX_A {
        match self.bits {
            0 => CAN_TST_TX_A::CAN_TST_TX_CANCTL,
            1 => CAN_TST_TX_A::CAN_TST_TX_SAMPLE,
            2 => CAN_TST_TX_A::CAN_TST_TX_DOMINANT,
            3 => CAN_TST_TX_A::CAN_TST_TX_RECESSIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "CAN Module Control"]
    #[inline(always)]
    pub fn is_can_tst_tx_canctl(&self) -> bool {
        *self == CAN_TST_TX_A::CAN_TST_TX_CANCTL
    }
    #[doc = "Sample Point"]
    #[inline(always)]
    pub fn is_can_tst_tx_sample(&self) -> bool {
        *self == CAN_TST_TX_A::CAN_TST_TX_SAMPLE
    }
    #[doc = "Driven Low"]
    #[inline(always)]
    pub fn is_can_tst_tx_dominant(&self) -> bool {
        *self == CAN_TST_TX_A::CAN_TST_TX_DOMINANT
    }
    #[doc = "Driven High"]
    #[inline(always)]
    pub fn is_can_tst_tx_recessive(&self) -> bool {
        *self == CAN_TST_TX_A::CAN_TST_TX_RECESSIVE
    }
}
#[doc = "Field `CAN_TST_TX` writer - Transmit Control"]
pub type CAN_TST_TX_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CAN_TST_TX_A>;
impl<'a, REG, const O: u8> CAN_TST_TX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CAN Module Control"]
    #[inline(always)]
    pub fn can_tst_tx_canctl(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_TST_TX_A::CAN_TST_TX_CANCTL)
    }
    #[doc = "Sample Point"]
    #[inline(always)]
    pub fn can_tst_tx_sample(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_TST_TX_A::CAN_TST_TX_SAMPLE)
    }
    #[doc = "Driven Low"]
    #[inline(always)]
    pub fn can_tst_tx_dominant(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_TST_TX_A::CAN_TST_TX_DOMINANT)
    }
    #[doc = "Driven High"]
    #[inline(always)]
    pub fn can_tst_tx_recessive(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_TST_TX_A::CAN_TST_TX_RECESSIVE)
    }
}
#[doc = "Field `CAN_TST_RX` reader - Receive Observation"]
pub type CAN_TST_RX_R = crate::BitReader;
#[doc = "Field `CAN_TST_RX` writer - Receive Observation"]
pub type CAN_TST_RX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn can_tst_basic(&self) -> CAN_TST_BASIC_R {
        CAN_TST_BASIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn can_tst_silent(&self) -> CAN_TST_SILENT_R {
        CAN_TST_SILENT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn can_tst_lback(&self) -> CAN_TST_LBACK_R {
        CAN_TST_LBACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Transmit Control"]
    #[inline(always)]
    pub fn can_tst_tx(&self) -> CAN_TST_TX_R {
        CAN_TST_TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Observation"]
    #[inline(always)]
    pub fn can_tst_rx(&self) -> CAN_TST_RX_R {
        CAN_TST_RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    #[must_use]
    pub fn can_tst_basic(&mut self) -> CAN_TST_BASIC_W<TST_SPEC, 2> {
        CAN_TST_BASIC_W::new(self)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    #[must_use]
    pub fn can_tst_silent(&mut self) -> CAN_TST_SILENT_W<TST_SPEC, 3> {
        CAN_TST_SILENT_W::new(self)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn can_tst_lback(&mut self) -> CAN_TST_LBACK_W<TST_SPEC, 4> {
        CAN_TST_LBACK_W::new(self)
    }
    #[doc = "Bits 5:6 - Transmit Control"]
    #[inline(always)]
    #[must_use]
    pub fn can_tst_tx(&mut self) -> CAN_TST_TX_W<TST_SPEC, 5> {
        CAN_TST_TX_W::new(self)
    }
    #[doc = "Bit 7 - Receive Observation"]
    #[inline(always)]
    #[must_use]
    pub fn can_tst_rx(&mut self) -> CAN_TST_RX_W<TST_SPEC, 7> {
        CAN_TST_RX_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CAN Test\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TST_SPEC;
impl crate::RegisterSpec for TST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tst::R`](R) reader structure"]
impl crate::Readable for TST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tst::W`](W) writer structure"]
impl crate::Writable for TST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TST to value 0"]
impl crate::Resettable for TST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
