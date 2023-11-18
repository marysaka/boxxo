#[doc = "Register `IF1MCTL` reader"]
pub type R = crate::R<IF1MCTL_SPEC>;
#[doc = "Register `IF1MCTL` writer"]
pub type W = crate::W<IF1MCTL_SPEC>;
#[doc = "Field `CAN_IF1MCTL_DLC` reader - Data Length Code"]
pub type CAN_IF1MCTL_DLC_R = crate::FieldReader;
#[doc = "Field `CAN_IF1MCTL_DLC` writer - Data Length Code"]
pub type CAN_IF1MCTL_DLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CAN_IF1MCTL_EOB` reader - End of Buffer"]
pub type CAN_IF1MCTL_EOB_R = crate::BitReader;
#[doc = "Field `CAN_IF1MCTL_EOB` writer - End of Buffer"]
pub type CAN_IF1MCTL_EOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF1MCTL_TXRQST` reader - Transmit Request"]
pub type CAN_IF1MCTL_TXRQST_R = crate::BitReader;
#[doc = "Field `CAN_IF1MCTL_TXRQST` writer - Transmit Request"]
pub type CAN_IF1MCTL_TXRQST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF1MCTL_RMTEN` reader - Remote Enable"]
pub type CAN_IF1MCTL_RMTEN_R = crate::BitReader;
#[doc = "Field `CAN_IF1MCTL_RMTEN` writer - Remote Enable"]
pub type CAN_IF1MCTL_RMTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF1MCTL_RXIE` reader - Receive Interrupt Enable"]
pub type CAN_IF1MCTL_RXIE_R = crate::BitReader;
#[doc = "Field `CAN_IF1MCTL_RXIE` writer - Receive Interrupt Enable"]
pub type CAN_IF1MCTL_RXIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF1MCTL_TXIE` reader - Transmit Interrupt Enable"]
pub type CAN_IF1MCTL_TXIE_R = crate::BitReader;
#[doc = "Field `CAN_IF1MCTL_TXIE` writer - Transmit Interrupt Enable"]
pub type CAN_IF1MCTL_TXIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF1MCTL_UMASK` reader - Use Acceptance Mask"]
pub type CAN_IF1MCTL_UMASK_R = crate::BitReader;
#[doc = "Field `CAN_IF1MCTL_UMASK` writer - Use Acceptance Mask"]
pub type CAN_IF1MCTL_UMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF1MCTL_INTPND` reader - Interrupt Pending"]
pub type CAN_IF1MCTL_INTPND_R = crate::BitReader;
#[doc = "Field `CAN_IF1MCTL_INTPND` writer - Interrupt Pending"]
pub type CAN_IF1MCTL_INTPND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF1MCTL_MSGLST` reader - Message Lost"]
pub type CAN_IF1MCTL_MSGLST_R = crate::BitReader;
#[doc = "Field `CAN_IF1MCTL_MSGLST` writer - Message Lost"]
pub type CAN_IF1MCTL_MSGLST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF1MCTL_NEWDAT` reader - New Data"]
pub type CAN_IF1MCTL_NEWDAT_R = crate::BitReader;
#[doc = "Field `CAN_IF1MCTL_NEWDAT` writer - New Data"]
pub type CAN_IF1MCTL_NEWDAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn can_if1mctl_dlc(&self) -> CAN_IF1MCTL_DLC_R {
        CAN_IF1MCTL_DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn can_if1mctl_eob(&self) -> CAN_IF1MCTL_EOB_R {
        CAN_IF1MCTL_EOB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn can_if1mctl_txrqst(&self) -> CAN_IF1MCTL_TXRQST_R {
        CAN_IF1MCTL_TXRQST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn can_if1mctl_rmten(&self) -> CAN_IF1MCTL_RMTEN_R {
        CAN_IF1MCTL_RMTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn can_if1mctl_rxie(&self) -> CAN_IF1MCTL_RXIE_R {
        CAN_IF1MCTL_RXIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn can_if1mctl_txie(&self) -> CAN_IF1MCTL_TXIE_R {
        CAN_IF1MCTL_TXIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn can_if1mctl_umask(&self) -> CAN_IF1MCTL_UMASK_R {
        CAN_IF1MCTL_UMASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn can_if1mctl_intpnd(&self) -> CAN_IF1MCTL_INTPND_R {
        CAN_IF1MCTL_INTPND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Lost"]
    #[inline(always)]
    pub fn can_if1mctl_msglst(&self) -> CAN_IF1MCTL_MSGLST_R {
        CAN_IF1MCTL_MSGLST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn can_if1mctl_newdat(&self) -> CAN_IF1MCTL_NEWDAT_R {
        CAN_IF1MCTL_NEWDAT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_dlc(&mut self) -> CAN_IF1MCTL_DLC_W<IF1MCTL_SPEC, 0> {
        CAN_IF1MCTL_DLC_W::new(self)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_eob(&mut self) -> CAN_IF1MCTL_EOB_W<IF1MCTL_SPEC, 7> {
        CAN_IF1MCTL_EOB_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_txrqst(&mut self) -> CAN_IF1MCTL_TXRQST_W<IF1MCTL_SPEC, 8> {
        CAN_IF1MCTL_TXRQST_W::new(self)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_rmten(&mut self) -> CAN_IF1MCTL_RMTEN_W<IF1MCTL_SPEC, 9> {
        CAN_IF1MCTL_RMTEN_W::new(self)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_rxie(&mut self) -> CAN_IF1MCTL_RXIE_W<IF1MCTL_SPEC, 10> {
        CAN_IF1MCTL_RXIE_W::new(self)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_txie(&mut self) -> CAN_IF1MCTL_TXIE_W<IF1MCTL_SPEC, 11> {
        CAN_IF1MCTL_TXIE_W::new(self)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_umask(&mut self) -> CAN_IF1MCTL_UMASK_W<IF1MCTL_SPEC, 12> {
        CAN_IF1MCTL_UMASK_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_intpnd(&mut self) -> CAN_IF1MCTL_INTPND_W<IF1MCTL_SPEC, 13> {
        CAN_IF1MCTL_INTPND_W::new(self)
    }
    #[doc = "Bit 14 - Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_msglst(&mut self) -> CAN_IF1MCTL_MSGLST_W<IF1MCTL_SPEC, 14> {
        CAN_IF1MCTL_MSGLST_W::new(self)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    #[must_use]
    pub fn can_if1mctl_newdat(&mut self) -> CAN_IF1MCTL_NEWDAT_W<IF1MCTL_SPEC, 15> {
        CAN_IF1MCTL_NEWDAT_W::new(self)
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
#[doc = "CAN IF1 Message Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1mctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1mctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF1MCTL_SPEC;
impl crate::RegisterSpec for IF1MCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if1mctl::R`](R) reader structure"]
impl crate::Readable for IF1MCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`if1mctl::W`](W) writer structure"]
impl crate::Writable for IF1MCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF1MCTL to value 0"]
impl crate::Resettable for IF1MCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
