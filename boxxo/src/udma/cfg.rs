#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `UDMA_CFG_MASTEN` writer - Controller Master Enable"]
pub type UDMA_CFG_MASTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Controller Master Enable"]
    #[inline(always)]
    #[must_use]
    pub fn udma_cfg_masten(&mut self) -> UDMA_CFG_MASTEN_W<CFG_SPEC, 0> {
        UDMA_CFG_MASTEN_W::new(self)
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
#[doc = "DMA Configuration\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
