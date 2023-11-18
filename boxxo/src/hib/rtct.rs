#[doc = "Register `RTCT` reader"]
pub type R = crate::R<RTCT_SPEC>;
#[doc = "Register `RTCT` writer"]
pub type W = crate::W<RTCT_SPEC>;
#[doc = "Field `HIB_RTCT_TRIM` reader - RTC Trim Value"]
pub type HIB_RTCT_TRIM_R = crate::FieldReader<u16>;
#[doc = "Field `HIB_RTCT_TRIM` writer - RTC Trim Value"]
pub type HIB_RTCT_TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC Trim Value"]
    #[inline(always)]
    pub fn hib_rtct_trim(&self) -> HIB_RTCT_TRIM_R {
        HIB_RTCT_TRIM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn hib_rtct_trim(&mut self) -> HIB_RTCT_TRIM_W<RTCT_SPEC, 0> {
        HIB_RTCT_TRIM_W::new(self)
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
#[doc = "Hibernation RTC Trim\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCT_SPEC;
impl crate::RegisterSpec for RTCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtct::R`](R) reader structure"]
impl crate::Readable for RTCT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtct::W`](W) writer structure"]
impl crate::Writable for RTCT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCT to value 0"]
impl crate::Resettable for RTCT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
