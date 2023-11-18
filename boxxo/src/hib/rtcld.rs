#[doc = "Register `RTCLD` reader"]
pub type R = crate::R<RTCLD_SPEC>;
#[doc = "Register `RTCLD` writer"]
pub type W = crate::W<RTCLD_SPEC>;
#[doc = "Field `HIB_RTCLD` reader - RTC Load"]
pub type HIB_RTCLD_R = crate::FieldReader<u32>;
#[doc = "Field `HIB_RTCLD` writer - RTC Load"]
pub type HIB_RTCLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - RTC Load"]
    #[inline(always)]
    pub fn hib_rtcld(&self) -> HIB_RTCLD_R {
        HIB_RTCLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Load"]
    #[inline(always)]
    #[must_use]
    pub fn hib_rtcld(&mut self) -> HIB_RTCLD_W<RTCLD_SPEC, 0> {
        HIB_RTCLD_W::new(self)
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
#[doc = "Hibernation RTC Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCLD_SPEC;
impl crate::RegisterSpec for RTCLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcld::R`](R) reader structure"]
impl crate::Readable for RTCLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcld::W`](W) writer structure"]
impl crate::Writable for RTCLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCLD to value 0"]
impl crate::Resettable for RTCLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
