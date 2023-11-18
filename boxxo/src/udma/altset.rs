#[doc = "Register `ALTSET` reader"]
pub type R = crate::R<ALTSET_SPEC>;
#[doc = "Register `ALTSET` writer"]
pub type W = crate::W<ALTSET_SPEC>;
#[doc = "Field `UDMA_ALTSET_SET` reader - Channel \\[n\\]
Alternate Set"]
pub type UDMA_ALTSET_SET_R = crate::FieldReader<u32>;
#[doc = "Field `UDMA_ALTSET_SET` writer - Channel \\[n\\]
Alternate Set"]
pub type UDMA_ALTSET_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Alternate Set"]
    #[inline(always)]
    pub fn udma_altset_set(&self) -> UDMA_ALTSET_SET_R {
        UDMA_ALTSET_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Alternate Set"]
    #[inline(always)]
    #[must_use]
    pub fn udma_altset_set(&mut self) -> UDMA_ALTSET_SET_W<ALTSET_SPEC, 0> {
        UDMA_ALTSET_SET_W::new(self)
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
#[doc = "DMA Channel Primary Alternate Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALTSET_SPEC;
impl crate::RegisterSpec for ALTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altset::R`](R) reader structure"]
impl crate::Readable for ALTSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`altset::W`](W) writer structure"]
impl crate::Writable for ALTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALTSET to value 0"]
impl crate::Resettable for ALTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
