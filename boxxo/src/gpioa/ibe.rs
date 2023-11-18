#[doc = "Register `IBE` reader"]
pub type R = crate::R<IBE_SPEC>;
#[doc = "Register `IBE` writer"]
pub type W = crate::W<IBE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IBE_SPEC> {
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
#[doc = "GPIO Interrupt Both Edges\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBE_SPEC;
impl crate::RegisterSpec for IBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibe::R`](R) reader structure"]
impl crate::Readable for IBE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibe::W`](W) writer structure"]
impl crate::Writable for IBE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBE to value 0"]
impl crate::Resettable for IBE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
