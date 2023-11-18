#[doc = "Register `SLR` reader"]
pub type R = crate::R<SLR_SPEC>;
#[doc = "Register `SLR` writer"]
pub type W = crate::W<SLR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
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
#[doc = "GPIO Slew Rate Control Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLR_SPEC;
impl crate::RegisterSpec for SLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slr::R`](R) reader structure"]
impl crate::Readable for SLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slr::W`](W) writer structure"]
impl crate::Writable for SLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLR to value 0"]
impl crate::Resettable for SLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
