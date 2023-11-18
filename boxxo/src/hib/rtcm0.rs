#[doc = "Register `RTCM0` reader"]
pub type R = crate::R<RTCM0_SPEC>;
#[doc = "Register `RTCM0` writer"]
pub type W = crate::W<RTCM0_SPEC>;
#[doc = "Field `HIB_RTCM0` reader - RTC Match 0"]
pub type HIB_RTCM0_R = crate::FieldReader<u32>;
#[doc = "Field `HIB_RTCM0` writer - RTC Match 0"]
pub type HIB_RTCM0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - RTC Match 0"]
    #[inline(always)]
    pub fn hib_rtcm0(&self) -> HIB_RTCM0_R {
        HIB_RTCM0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Match 0"]
    #[inline(always)]
    #[must_use]
    pub fn hib_rtcm0(&mut self) -> HIB_RTCM0_W<RTCM0_SPEC, 0> {
        HIB_RTCM0_W::new(self)
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
#[doc = "Hibernation RTC Match 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcm0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcm0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCM0_SPEC;
impl crate::RegisterSpec for RTCM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcm0::R`](R) reader structure"]
impl crate::Readable for RTCM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcm0::W`](W) writer structure"]
impl crate::Writable for RTCM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCM0 to value 0"]
impl crate::Resettable for RTCM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
