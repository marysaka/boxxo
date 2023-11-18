#[doc = "Register `DEN` reader"]
pub type R = crate::R<DEN_SPEC>;
#[doc = "Register `DEN` writer"]
pub type W = crate::W<DEN_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEN_SPEC> {
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
#[doc = "GPIO Digital Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`den::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`den::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEN_SPEC;
impl crate::RegisterSpec for DEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`den::R`](R) reader structure"]
impl crate::Readable for DEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`den::W`](W) writer structure"]
impl crate::Writable for DEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEN to value 0"]
impl crate::Resettable for DEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
