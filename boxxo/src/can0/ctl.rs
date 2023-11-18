#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `CAN_CTL_INIT` reader - Initialization"]
pub type CAN_CTL_INIT_R = crate::BitReader;
#[doc = "Field `CAN_CTL_INIT` writer - Initialization"]
pub type CAN_CTL_INIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_CTL_IE` reader - CAN Interrupt Enable"]
pub type CAN_CTL_IE_R = crate::BitReader;
#[doc = "Field `CAN_CTL_IE` writer - CAN Interrupt Enable"]
pub type CAN_CTL_IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_CTL_SIE` reader - Status Interrupt Enable"]
pub type CAN_CTL_SIE_R = crate::BitReader;
#[doc = "Field `CAN_CTL_SIE` writer - Status Interrupt Enable"]
pub type CAN_CTL_SIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_CTL_EIE` reader - Error Interrupt Enable"]
pub type CAN_CTL_EIE_R = crate::BitReader;
#[doc = "Field `CAN_CTL_EIE` writer - Error Interrupt Enable"]
pub type CAN_CTL_EIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_CTL_DAR` reader - Disable Automatic-Retransmission"]
pub type CAN_CTL_DAR_R = crate::BitReader;
#[doc = "Field `CAN_CTL_DAR` writer - Disable Automatic-Retransmission"]
pub type CAN_CTL_DAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_CTL_CCE` reader - Configuration Change Enable"]
pub type CAN_CTL_CCE_R = crate::BitReader;
#[doc = "Field `CAN_CTL_CCE` writer - Configuration Change Enable"]
pub type CAN_CTL_CCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_CTL_TEST` reader - Test Mode Enable"]
pub type CAN_CTL_TEST_R = crate::BitReader;
#[doc = "Field `CAN_CTL_TEST` writer - Test Mode Enable"]
pub type CAN_CTL_TEST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn can_ctl_init(&self) -> CAN_CTL_INIT_R {
        CAN_CTL_INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Interrupt Enable"]
    #[inline(always)]
    pub fn can_ctl_ie(&self) -> CAN_CTL_IE_R {
        CAN_CTL_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status Interrupt Enable"]
    #[inline(always)]
    pub fn can_ctl_sie(&self) -> CAN_CTL_SIE_R {
        CAN_CTL_SIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn can_ctl_eie(&self) -> CAN_CTL_EIE_R {
        CAN_CTL_EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Automatic-Retransmission"]
    #[inline(always)]
    pub fn can_ctl_dar(&self) -> CAN_CTL_DAR_R {
        CAN_CTL_DAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn can_ctl_cce(&self) -> CAN_CTL_CCE_R {
        CAN_CTL_CCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    pub fn can_ctl_test(&self) -> CAN_CTL_TEST_R {
        CAN_CTL_TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctl_init(&mut self) -> CAN_CTL_INIT_W<CTL_SPEC, 0> {
        CAN_CTL_INIT_W::new(self)
    }
    #[doc = "Bit 1 - CAN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctl_ie(&mut self) -> CAN_CTL_IE_W<CTL_SPEC, 1> {
        CAN_CTL_IE_W::new(self)
    }
    #[doc = "Bit 2 - Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctl_sie(&mut self) -> CAN_CTL_SIE_W<CTL_SPEC, 2> {
        CAN_CTL_SIE_W::new(self)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctl_eie(&mut self) -> CAN_CTL_EIE_W<CTL_SPEC, 3> {
        CAN_CTL_EIE_W::new(self)
    }
    #[doc = "Bit 5 - Disable Automatic-Retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctl_dar(&mut self) -> CAN_CTL_DAR_W<CTL_SPEC, 5> {
        CAN_CTL_DAR_W::new(self)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctl_cce(&mut self) -> CAN_CTL_CCE_W<CTL_SPEC, 6> {
        CAN_CTL_CCE_W::new(self)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctl_test(&mut self) -> CAN_CTL_TEST_W<CTL_SPEC, 7> {
        CAN_CTL_TEST_W::new(self)
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
#[doc = "CAN Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
