#[doc = "Register `PRIOSET` reader"]
pub type R = crate::R<PRIOSET_SPEC>;
#[doc = "Register `PRIOSET` writer"]
pub type W = crate::W<PRIOSET_SPEC>;
#[doc = "Field `UDMA_PRIOSET_SET` reader - Channel \\[n\\]
Priority Set"]
pub type UDMA_PRIOSET_SET_R = crate::FieldReader<u32>;
#[doc = "Field `UDMA_PRIOSET_SET` writer - Channel \\[n\\]
Priority Set"]
pub type UDMA_PRIOSET_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Priority Set"]
    #[inline(always)]
    pub fn udma_prioset_set(&self) -> UDMA_PRIOSET_SET_R {
        UDMA_PRIOSET_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Priority Set"]
    #[inline(always)]
    #[must_use]
    pub fn udma_prioset_set(&mut self) -> UDMA_PRIOSET_SET_W<PRIOSET_SPEC, 0> {
        UDMA_PRIOSET_SET_W::new(self)
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
#[doc = "DMA Channel Priority Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prioset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prioset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIOSET_SPEC;
impl crate::RegisterSpec for PRIOSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prioset::R`](R) reader structure"]
impl crate::Readable for PRIOSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prioset::W`](W) writer structure"]
impl crate::Writable for PRIOSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIOSET to value 0"]
impl crate::Resettable for PRIOSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
