#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `TIMER_RIS_TATORIS` reader - GPTM Timer A Time-Out Raw Interrupt"]
pub type TIMER_RIS_TATORIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_TATORIS` writer - GPTM Timer A Time-Out Raw Interrupt"]
pub type TIMER_RIS_TATORIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_RIS_CAMRIS` reader - GPTM Timer A Capture Mode Match Raw Interrupt"]
pub type TIMER_RIS_CAMRIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_CAMRIS` writer - GPTM Timer A Capture Mode Match Raw Interrupt"]
pub type TIMER_RIS_CAMRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_RIS_CAERIS` reader - GPTM Timer A Capture Mode Event Raw Interrupt"]
pub type TIMER_RIS_CAERIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_CAERIS` writer - GPTM Timer A Capture Mode Event Raw Interrupt"]
pub type TIMER_RIS_CAERIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_RIS_RTCRIS` reader - GPTM RTC Raw Interrupt"]
pub type TIMER_RIS_RTCRIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_RTCRIS` writer - GPTM RTC Raw Interrupt"]
pub type TIMER_RIS_RTCRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_RIS_TAMRIS` reader - GPTM Timer A Match Raw Interrupt"]
pub type TIMER_RIS_TAMRIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_TAMRIS` writer - GPTM Timer A Match Raw Interrupt"]
pub type TIMER_RIS_TAMRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_RIS_TBTORIS` reader - GPTM Timer B Time-Out Raw Interrupt"]
pub type TIMER_RIS_TBTORIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_TBTORIS` writer - GPTM Timer B Time-Out Raw Interrupt"]
pub type TIMER_RIS_TBTORIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_RIS_CBMRIS` reader - GPTM Timer B Capture Mode Match Raw Interrupt"]
pub type TIMER_RIS_CBMRIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_CBMRIS` writer - GPTM Timer B Capture Mode Match Raw Interrupt"]
pub type TIMER_RIS_CBMRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_RIS_CBERIS` reader - GPTM Timer B Capture Mode Event Raw Interrupt"]
pub type TIMER_RIS_CBERIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_CBERIS` writer - GPTM Timer B Capture Mode Event Raw Interrupt"]
pub type TIMER_RIS_CBERIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_RIS_TBMRIS` reader - GPTM Timer B Match Raw Interrupt"]
pub type TIMER_RIS_TBMRIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_TBMRIS` writer - GPTM Timer B Match Raw Interrupt"]
pub type TIMER_RIS_TBMRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_RIS_WUERIS` reader - GPTM Write Update Error Raw Interrupt"]
pub type TIMER_RIS_WUERIS_R = crate::BitReader;
#[doc = "Field `TIMER_RIS_WUERIS` writer - GPTM Write Update Error Raw Interrupt"]
pub type TIMER_RIS_WUERIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tatoris(&self) -> TIMER_RIS_TATORIS_R {
        TIMER_RIS_TATORIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_camris(&self) -> TIMER_RIS_CAMRIS_R {
        TIMER_RIS_CAMRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_caeris(&self) -> TIMER_RIS_CAERIS_R {
        TIMER_RIS_CAERIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_rtcris(&self) -> TIMER_RIS_RTCRIS_R {
        TIMER_RIS_RTCRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tamris(&self) -> TIMER_RIS_TAMRIS_R {
        TIMER_RIS_TAMRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbtoris(&self) -> TIMER_RIS_TBTORIS_R {
        TIMER_RIS_TBTORIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cbmris(&self) -> TIMER_RIS_CBMRIS_R {
        TIMER_RIS_CBMRIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cberis(&self) -> TIMER_RIS_CBERIS_R {
        TIMER_RIS_CBERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbmris(&self) -> TIMER_RIS_TBMRIS_R {
        TIMER_RIS_TBMRIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - GPTM Write Update Error Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_wueris(&self) -> TIMER_RIS_WUERIS_R {
        TIMER_RIS_WUERIS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_tatoris(&mut self) -> TIMER_RIS_TATORIS_W<RIS_SPEC, 0> {
        TIMER_RIS_TATORIS_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_camris(&mut self) -> TIMER_RIS_CAMRIS_W<RIS_SPEC, 1> {
        TIMER_RIS_CAMRIS_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_caeris(&mut self) -> TIMER_RIS_CAERIS_W<RIS_SPEC, 2> {
        TIMER_RIS_CAERIS_W::new(self)
    }
    #[doc = "Bit 3 - GPTM RTC Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_rtcris(&mut self) -> TIMER_RIS_RTCRIS_W<RIS_SPEC, 3> {
        TIMER_RIS_RTCRIS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_tamris(&mut self) -> TIMER_RIS_TAMRIS_W<RIS_SPEC, 4> {
        TIMER_RIS_TAMRIS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_tbtoris(&mut self) -> TIMER_RIS_TBTORIS_W<RIS_SPEC, 8> {
        TIMER_RIS_TBTORIS_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_cbmris(&mut self) -> TIMER_RIS_CBMRIS_W<RIS_SPEC, 9> {
        TIMER_RIS_CBMRIS_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_cberis(&mut self) -> TIMER_RIS_CBERIS_W<RIS_SPEC, 10> {
        TIMER_RIS_CBERIS_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_tbmris(&mut self) -> TIMER_RIS_TBMRIS_W<RIS_SPEC, 11> {
        TIMER_RIS_TBMRIS_W::new(self)
    }
    #[doc = "Bit 16 - GPTM Write Update Error Raw Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_ris_wueris(&mut self) -> TIMER_RIS_WUERIS_W<RIS_SPEC, 16> {
        TIMER_RIS_WUERIS_W::new(self)
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
#[doc = "GPTM Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
