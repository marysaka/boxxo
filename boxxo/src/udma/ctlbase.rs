#[doc = "Register `CTLBASE` reader"]
pub type R = crate::R<CTLBASE_SPEC>;
#[doc = "Register `CTLBASE` writer"]
pub type W = crate::W<CTLBASE_SPEC>;
#[doc = "Field `UDMA_CTLBASE_ADDR` reader - Channel Control Base Address"]
pub type UDMA_CTLBASE_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `UDMA_CTLBASE_ADDR` writer - Channel Control Base Address"]
pub type UDMA_CTLBASE_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 22, O, u32>;
impl R {
    #[doc = "Bits 10:31 - Channel Control Base Address"]
    #[inline(always)]
    pub fn udma_ctlbase_addr(&self) -> UDMA_CTLBASE_ADDR_R {
        UDMA_CTLBASE_ADDR_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - Channel Control Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn udma_ctlbase_addr(&mut self) -> UDMA_CTLBASE_ADDR_W<CTLBASE_SPEC, 10> {
        UDMA_CTLBASE_ADDR_W::new(self)
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
#[doc = "DMA Channel Control Base Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlbase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlbase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLBASE_SPEC;
impl crate::RegisterSpec for CTLBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlbase::R`](R) reader structure"]
impl crate::Readable for CTLBASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlbase::W`](W) writer structure"]
impl crate::Writable for CTLBASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLBASE to value 0"]
impl crate::Resettable for CTLBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
