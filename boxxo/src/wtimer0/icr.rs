#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `TIMER_ICR_TATOCINT` writer - GPTM Timer A Time-Out Raw Interrupt"]
pub type TIMER_ICR_TATOCINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_ICR_CAMCINT` writer - GPTM Timer A Capture Mode Match Interrupt Clear"]
pub type TIMER_ICR_CAMCINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_ICR_CAECINT` writer - GPTM Timer A Capture Mode Event Interrupt Clear"]
pub type TIMER_ICR_CAECINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_ICR_RTCCINT` writer - GPTM RTC Interrupt Clear"]
pub type TIMER_ICR_RTCCINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_ICR_TAMCINT` writer - GPTM Timer A Match Interrupt Clear"]
pub type TIMER_ICR_TAMCINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_ICR_TBTOCINT` writer - GPTM Timer B Time-Out Interrupt Clear"]
pub type TIMER_ICR_TBTOCINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_ICR_CBMCINT` writer - GPTM Timer B Capture Mode Match Interrupt Clear"]
pub type TIMER_ICR_CBMCINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_ICR_CBECINT` writer - GPTM Timer B Capture Mode Event Interrupt Clear"]
pub type TIMER_ICR_CBECINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_ICR_TBMCINT` writer - GPTM Timer B Match Interrupt Clear"]
pub type TIMER_ICR_TBMCINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_ICR_WUECINT` writer - 32/64-Bit GPTM Write Update Error Interrupt Clear"]
pub type TIMER_ICR_WUECINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_tatocint(&mut self) -> TIMER_ICR_TATOCINT_W<ICR_SPEC, 0> {
        TIMER_ICR_TATOCINT_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_camcint(&mut self) -> TIMER_ICR_CAMCINT_W<ICR_SPEC, 1> {
        TIMER_ICR_CAMCINT_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_caecint(&mut self) -> TIMER_ICR_CAECINT_W<ICR_SPEC, 2> {
        TIMER_ICR_CAECINT_W::new(self)
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_rtccint(&mut self) -> TIMER_ICR_RTCCINT_W<ICR_SPEC, 3> {
        TIMER_ICR_RTCCINT_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_tamcint(&mut self) -> TIMER_ICR_TAMCINT_W<ICR_SPEC, 4> {
        TIMER_ICR_TAMCINT_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_tbtocint(&mut self) -> TIMER_ICR_TBTOCINT_W<ICR_SPEC, 8> {
        TIMER_ICR_TBTOCINT_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_cbmcint(&mut self) -> TIMER_ICR_CBMCINT_W<ICR_SPEC, 9> {
        TIMER_ICR_CBMCINT_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_cbecint(&mut self) -> TIMER_ICR_CBECINT_W<ICR_SPEC, 10> {
        TIMER_ICR_CBECINT_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_tbmcint(&mut self) -> TIMER_ICR_TBMCINT_W<ICR_SPEC, 11> {
        TIMER_ICR_TBMCINT_W::new(self)
    }
    #[doc = "Bit 16 - 32/64-Bit GPTM Write Update Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timer_icr_wuecint(&mut self) -> TIMER_ICR_WUECINT_W<ICR_SPEC, 16> {
        TIMER_ICR_WUECINT_W::new(self)
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
#[doc = "GPTM Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
