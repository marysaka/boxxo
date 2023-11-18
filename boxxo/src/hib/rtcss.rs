#[doc = "Register `RTCSS` reader"]
pub type R = crate::R<RTCSS_SPEC>;
#[doc = "Register `RTCSS` writer"]
pub type W = crate::W<RTCSS_SPEC>;
#[doc = "Field `HIB_RTCSS_RTCSSC` reader - RTC Sub Seconds Count"]
pub type HIB_RTCSS_RTCSSC_R = crate::FieldReader<u16>;
#[doc = "Field `HIB_RTCSS_RTCSSC` writer - RTC Sub Seconds Count"]
pub type HIB_RTCSS_RTCSSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `HIB_RTCSS_RTCSSM` reader - RTC Sub Seconds Match"]
pub type HIB_RTCSS_RTCSSM_R = crate::FieldReader<u16>;
#[doc = "Field `HIB_RTCSS_RTCSSM` writer - RTC Sub Seconds Match"]
pub type HIB_RTCSS_RTCSSM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
impl R {
    #[doc = "Bits 0:14 - RTC Sub Seconds Count"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssc(&self) -> HIB_RTCSS_RTCSSC_R {
        HIB_RTCSS_RTCSSC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - RTC Sub Seconds Match"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssm(&self) -> HIB_RTCSS_RTCSSM_R {
        HIB_RTCSS_RTCSSM_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - RTC Sub Seconds Count"]
    #[inline(always)]
    #[must_use]
    pub fn hib_rtcss_rtcssc(&mut self) -> HIB_RTCSS_RTCSSC_W<RTCSS_SPEC, 0> {
        HIB_RTCSS_RTCSSC_W::new(self)
    }
    #[doc = "Bits 16:30 - RTC Sub Seconds Match"]
    #[inline(always)]
    #[must_use]
    pub fn hib_rtcss_rtcssm(&mut self) -> HIB_RTCSS_RTCSSM_W<RTCSS_SPEC, 16> {
        HIB_RTCSS_RTCSSM_W::new(self)
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
#[doc = "Hibernation RTC Sub Seconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcss::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcss::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCSS_SPEC;
impl crate::RegisterSpec for RTCSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcss::R`](R) reader structure"]
impl crate::Readable for RTCSS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcss::W`](W) writer structure"]
impl crate::Writable for RTCSS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCSS to value 0"]
impl crate::Resettable for RTCSS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
