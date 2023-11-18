#[doc = "Register `TBMATCHR` reader"]
pub type R = crate::R<TBMATCHR_SPEC>;
#[doc = "Register `TBMATCHR` writer"]
pub type W = crate::W<TBMATCHR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TBMATCHR_SPEC> {
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
#[doc = "GPTM Timer B Match\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbmatchr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbmatchr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBMATCHR_SPEC;
impl crate::RegisterSpec for TBMATCHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbmatchr::R`](R) reader structure"]
impl crate::Readable for TBMATCHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbmatchr::W`](W) writer structure"]
impl crate::Writable for TBMATCHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBMATCHR to value 0"]
impl crate::Resettable for TBMATCHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
