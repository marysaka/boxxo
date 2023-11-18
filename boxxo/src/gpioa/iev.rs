#[doc = "Register `IEV` reader"]
pub type R = crate::R<IEV_SPEC>;
#[doc = "Register `IEV` writer"]
pub type W = crate::W<IEV_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IEV_SPEC> {
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
#[doc = "GPIO Interrupt Event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEV_SPEC;
impl crate::RegisterSpec for IEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iev::R`](R) reader structure"]
impl crate::Readable for IEV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iev::W`](W) writer structure"]
impl crate::Writable for IEV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEV to value 0"]
impl crate::Resettable for IEV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
