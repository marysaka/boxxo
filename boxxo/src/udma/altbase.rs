#[doc = "Register `ALTBASE` reader"]
pub type R = crate::R<ALTBASE_SPEC>;
#[doc = "Register `ALTBASE` writer"]
pub type W = crate::W<ALTBASE_SPEC>;
#[doc = "Field `UDMA_ALTBASE_ADDR` reader - Alternate Channel Address Pointer"]
pub type UDMA_ALTBASE_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `UDMA_ALTBASE_ADDR` writer - Alternate Channel Address Pointer"]
pub type UDMA_ALTBASE_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Alternate Channel Address Pointer"]
    #[inline(always)]
    pub fn udma_altbase_addr(&self) -> UDMA_ALTBASE_ADDR_R {
        UDMA_ALTBASE_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alternate Channel Address Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn udma_altbase_addr(&mut self) -> UDMA_ALTBASE_ADDR_W<ALTBASE_SPEC, 0> {
        UDMA_ALTBASE_ADDR_W::new(self)
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
#[doc = "DMA Alternate Channel Control Base Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altbase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altbase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALTBASE_SPEC;
impl crate::RegisterSpec for ALTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altbase::R`](R) reader structure"]
impl crate::Readable for ALTBASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`altbase::W`](W) writer structure"]
impl crate::Writable for ALTBASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALTBASE to value 0"]
impl crate::Resettable for ALTBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
