#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MIS_SPEC>;
#[doc = "Field `TIMER_MIS_TATOMIS` reader - GPTM Timer A Time-Out Masked Interrupt"]
pub type TIMER_MIS_TATOMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_TATOMIS` writer - GPTM Timer A Time-Out Masked Interrupt"]
pub type TIMER_MIS_TATOMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_MIS_CAMMIS` reader - GPTM Timer A Capture Mode Match Masked Interrupt"]
pub type TIMER_MIS_CAMMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_CAMMIS` writer - GPTM Timer A Capture Mode Match Masked Interrupt"]
pub type TIMER_MIS_CAMMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_MIS_CAEMIS` reader - GPTM Timer A Capture Mode Event Masked Interrupt"]
pub type TIMER_MIS_CAEMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_CAEMIS` writer - GPTM Timer A Capture Mode Event Masked Interrupt"]
pub type TIMER_MIS_CAEMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_MIS_RTCMIS` reader - GPTM RTC Masked Interrupt"]
pub type TIMER_MIS_RTCMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_RTCMIS` writer - GPTM RTC Masked Interrupt"]
pub type TIMER_MIS_RTCMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_MIS_TAMMIS` reader - GPTM Timer A Match Masked Interrupt"]
pub type TIMER_MIS_TAMMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_TAMMIS` writer - GPTM Timer A Match Masked Interrupt"]
pub type TIMER_MIS_TAMMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_MIS_TBTOMIS` reader - GPTM Timer B Time-Out Masked Interrupt"]
pub type TIMER_MIS_TBTOMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_TBTOMIS` writer - GPTM Timer B Time-Out Masked Interrupt"]
pub type TIMER_MIS_TBTOMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_MIS_CBMMIS` reader - GPTM Timer B Capture Mode Match Masked Interrupt"]
pub type TIMER_MIS_CBMMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_CBMMIS` writer - GPTM Timer B Capture Mode Match Masked Interrupt"]
pub type TIMER_MIS_CBMMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_MIS_CBEMIS` reader - GPTM Timer B Capture Mode Event Masked Interrupt"]
pub type TIMER_MIS_CBEMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_CBEMIS` writer - GPTM Timer B Capture Mode Event Masked Interrupt"]
pub type TIMER_MIS_CBEMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_MIS_TBMMIS` reader - GPTM Timer B Match Masked Interrupt"]
pub type TIMER_MIS_TBMMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_TBMMIS` writer - GPTM Timer B Match Masked Interrupt"]
pub type TIMER_MIS_TBMMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_MIS_WUEMIS` reader - GPTM Write Update Error Masked Interrupt"]
pub type TIMER_MIS_WUEMIS_R = crate::BitReader;
#[doc = "Field `TIMER_MIS_WUEMIS` writer - GPTM Write Update Error Masked Interrupt"]
pub type TIMER_MIS_WUEMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tatomis(&self) -> TIMER_MIS_TATOMIS_R {
        TIMER_MIS_TATOMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cammis(&self) -> TIMER_MIS_CAMMIS_R {
        TIMER_MIS_CAMMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_caemis(&self) -> TIMER_MIS_CAEMIS_R {
        TIMER_MIS_CAEMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_rtcmis(&self) -> TIMER_MIS_RTCMIS_R {
        TIMER_MIS_RTCMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tammis(&self) -> TIMER_MIS_TAMMIS_R {
        TIMER_MIS_TAMMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbtomis(&self) -> TIMER_MIS_TBTOMIS_R {
        TIMER_MIS_TBTOMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbmmis(&self) -> TIMER_MIS_CBMMIS_R {
        TIMER_MIS_CBMMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbemis(&self) -> TIMER_MIS_CBEMIS_R {
        TIMER_MIS_CBEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbmmis(&self) -> TIMER_MIS_TBMMIS_R {
        TIMER_MIS_TBMMIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - GPTM Write Update Error Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_wuemis(&self) -> TIMER_MIS_WUEMIS_R {
        TIMER_MIS_WUEMIS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_tatomis(&mut self) -> TIMER_MIS_TATOMIS_W<MIS_SPEC, 0> {
        TIMER_MIS_TATOMIS_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_cammis(&mut self) -> TIMER_MIS_CAMMIS_W<MIS_SPEC, 1> {
        TIMER_MIS_CAMMIS_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_caemis(&mut self) -> TIMER_MIS_CAEMIS_W<MIS_SPEC, 2> {
        TIMER_MIS_CAEMIS_W::new(self)
    }
    #[doc = "Bit 3 - GPTM RTC Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_rtcmis(&mut self) -> TIMER_MIS_RTCMIS_W<MIS_SPEC, 3> {
        TIMER_MIS_RTCMIS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_tammis(&mut self) -> TIMER_MIS_TAMMIS_W<MIS_SPEC, 4> {
        TIMER_MIS_TAMMIS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_tbtomis(&mut self) -> TIMER_MIS_TBTOMIS_W<MIS_SPEC, 8> {
        TIMER_MIS_TBTOMIS_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_cbmmis(&mut self) -> TIMER_MIS_CBMMIS_W<MIS_SPEC, 9> {
        TIMER_MIS_CBMMIS_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_cbemis(&mut self) -> TIMER_MIS_CBEMIS_W<MIS_SPEC, 10> {
        TIMER_MIS_CBEMIS_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_tbmmis(&mut self) -> TIMER_MIS_TBMMIS_W<MIS_SPEC, 11> {
        TIMER_MIS_TBMMIS_W::new(self)
    }
    #[doc = "Bit 16 - GPTM Write Update Error Masked Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mis_wuemis(&mut self) -> TIMER_MIS_WUEMIS_W<MIS_SPEC, 16> {
        TIMER_MIS_WUEMIS_W::new(self)
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
#[doc = "GPTM Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
