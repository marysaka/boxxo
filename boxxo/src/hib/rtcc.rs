#[doc = "Register `RTCC` reader"]
pub type R = crate::R<RTCC_SPEC>;
#[doc = "Register `RTCC` writer"]
pub type W = crate::W<RTCC_SPEC>;
#[doc = "Field `HIB_RTCC` reader - RTC Counter"]
pub type HIB_RTCC_R = crate::FieldReader<u32>;
#[doc = "Field `HIB_RTCC` writer - RTC Counter"]
pub type HIB_RTCC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - RTC Counter"]
    #[inline(always)]
    pub fn hib_rtcc(&self) -> HIB_RTCC_R {
        HIB_RTCC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Counter"]
    #[inline(always)]
    #[must_use]
    pub fn hib_rtcc(&mut self) -> HIB_RTCC_W<RTCC_SPEC, 0> {
        HIB_RTCC_W::new(self)
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
#[doc = "Hibernation RTC Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCC_SPEC;
impl crate::RegisterSpec for RTCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcc::R`](R) reader structure"]
impl crate::Readable for RTCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcc::W`](W) writer structure"]
impl crate::Writable for RTCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCC to value 0"]
impl crate::Resettable for RTCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
