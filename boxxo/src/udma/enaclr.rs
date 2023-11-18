#[doc = "Register `ENACLR` writer"]
pub type W = crate::W<ENACLR_SPEC>;
#[doc = "Field `UDMA_ENACLR_CLR` writer - Clear Channel \\[n\\]
Enable Clear"]
pub type UDMA_ENACLR_CLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Clear Channel \\[n\\]
Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn udma_enaclr_clr(&mut self) -> UDMA_ENACLR_CLR_W<ENACLR_SPEC, 0> {
        UDMA_ENACLR_CLR_W::new(self)
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
#[doc = "DMA Channel Enable Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enaclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENACLR_SPEC;
impl crate::RegisterSpec for ENACLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enaclr::W`](W) writer structure"]
impl crate::Writable for ENACLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENACLR to value 0"]
impl crate::Resettable for ENACLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
