#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `SSI_ICR_RORIC` writer - SSI Receive Overrun Interrupt Clear"]
pub type SSI_ICR_RORIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_ICR_RTIC` writer - SSI Receive Time-Out Interrupt Clear"]
pub type SSI_ICR_RTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_icr_roric(&mut self) -> SSI_ICR_RORIC_W<ICR_SPEC, 0> {
        SSI_ICR_RORIC_W::new(self)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_icr_rtic(&mut self) -> SSI_ICR_RTIC_W<ICR_SPEC, 1> {
        SSI_ICR_RTIC_W::new(self)
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
#[doc = "SSI Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
