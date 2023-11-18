#[doc = "Register `USEBURSTCLR` writer"]
pub type W = crate::W<USEBURSTCLR_SPEC>;
#[doc = "Field `UDMA_USEBURSTCLR_CLR` writer - Channel \\[n\\]
Useburst Clear"]
pub type UDMA_USEBURSTCLR_CLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Useburst Clear"]
    #[inline(always)]
    #[must_use]
    pub fn udma_useburstclr_clr(&mut self) -> UDMA_USEBURSTCLR_CLR_W<USEBURSTCLR_SPEC, 0> {
        UDMA_USEBURSTCLR_CLR_W::new(self)
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
#[doc = "DMA Channel Useburst Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`useburstclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USEBURSTCLR_SPEC;
impl crate::RegisterSpec for USEBURSTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`useburstclr::W`](W) writer structure"]
impl crate::Writable for USEBURSTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USEBURSTCLR to value 0"]
impl crate::Resettable for USEBURSTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
