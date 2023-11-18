#[doc = "Register `REQMASKSET` reader"]
pub type R = crate::R<REQMASKSET_SPEC>;
#[doc = "Register `REQMASKSET` writer"]
pub type W = crate::W<REQMASKSET_SPEC>;
#[doc = "Field `UDMA_REQMASKSET_SET` reader - Channel \\[n\\]
Request Mask Set"]
pub type UDMA_REQMASKSET_SET_R = crate::FieldReader<u32>;
#[doc = "Field `UDMA_REQMASKSET_SET` writer - Channel \\[n\\]
Request Mask Set"]
pub type UDMA_REQMASKSET_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Request Mask Set"]
    #[inline(always)]
    pub fn udma_reqmaskset_set(&self) -> UDMA_REQMASKSET_SET_R {
        UDMA_REQMASKSET_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn udma_reqmaskset_set(&mut self) -> UDMA_REQMASKSET_SET_W<REQMASKSET_SPEC, 0> {
        UDMA_REQMASKSET_SET_W::new(self)
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
#[doc = "DMA Channel Request Mask Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqmaskset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqmaskset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REQMASKSET_SPEC;
impl crate::RegisterSpec for REQMASKSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqmaskset::R`](R) reader structure"]
impl crate::Readable for REQMASKSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reqmaskset::W`](W) writer structure"]
impl crate::Writable for REQMASKSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REQMASKSET to value 0"]
impl crate::Resettable for REQMASKSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
