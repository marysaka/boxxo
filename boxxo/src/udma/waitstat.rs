#[doc = "Register `WAITSTAT` reader"]
pub type R = crate::R<WAITSTAT_SPEC>;
#[doc = "Register `WAITSTAT` writer"]
pub type W = crate::W<WAITSTAT_SPEC>;
#[doc = "Field `UDMA_WAITSTAT_WAITREQ` reader - Channel \\[n\\]
Wait Status"]
pub type UDMA_WAITSTAT_WAITREQ_R = crate::FieldReader<u32>;
#[doc = "Field `UDMA_WAITSTAT_WAITREQ` writer - Channel \\[n\\]
Wait Status"]
pub type UDMA_WAITSTAT_WAITREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Wait Status"]
    #[inline(always)]
    pub fn udma_waitstat_waitreq(&self) -> UDMA_WAITSTAT_WAITREQ_R {
        UDMA_WAITSTAT_WAITREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Wait Status"]
    #[inline(always)]
    #[must_use]
    pub fn udma_waitstat_waitreq(&mut self) -> UDMA_WAITSTAT_WAITREQ_W<WAITSTAT_SPEC, 0> {
        UDMA_WAITSTAT_WAITREQ_W::new(self)
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
#[doc = "DMA Channel Wait-on-Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waitstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`waitstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAITSTAT_SPEC;
impl crate::RegisterSpec for WAITSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waitstat::R`](R) reader structure"]
impl crate::Readable for WAITSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`waitstat::W`](W) writer structure"]
impl crate::Writable for WAITSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAITSTAT to value 0"]
impl crate::Resettable for WAITSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
