#[doc = "Register `ERRCLR` reader"]
pub type R = crate::R<ERRCLR_SPEC>;
#[doc = "Register `ERRCLR` writer"]
pub type W = crate::W<ERRCLR_SPEC>;
#[doc = "Field `UDMA_ERRCLR_ERRCLR` reader - uDMA Bus Error Status"]
pub type UDMA_ERRCLR_ERRCLR_R = crate::BitReader;
#[doc = "Field `UDMA_ERRCLR_ERRCLR` writer - uDMA Bus Error Status"]
pub type UDMA_ERRCLR_ERRCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - uDMA Bus Error Status"]
    #[inline(always)]
    pub fn udma_errclr_errclr(&self) -> UDMA_ERRCLR_ERRCLR_R {
        UDMA_ERRCLR_ERRCLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Bus Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn udma_errclr_errclr(&mut self) -> UDMA_ERRCLR_ERRCLR_W<ERRCLR_SPEC, 0> {
        UDMA_ERRCLR_ERRCLR_W::new(self)
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
#[doc = "DMA Bus Error Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERRCLR_SPEC;
impl crate::RegisterSpec for ERRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errclr::R`](R) reader structure"]
impl crate::Readable for ERRCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`errclr::W`](W) writer structure"]
impl crate::Writable for ERRCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERRCLR to value 0"]
impl crate::Resettable for ERRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
