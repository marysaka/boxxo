#[doc = "Register `TBR` reader"]
pub type R = crate::R<TBR_SPEC>;
#[doc = "Register `TBR` writer"]
pub type W = crate::W<TBR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TBR_SPEC> {
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
#[doc = "GPTM Timer B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBR_SPEC;
impl crate::RegisterSpec for TBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbr::R`](R) reader structure"]
impl crate::Readable for TBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbr::W`](W) writer structure"]
impl crate::Writable for TBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBR to value 0"]
impl crate::Resettable for TBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
