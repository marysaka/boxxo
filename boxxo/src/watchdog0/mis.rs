#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MIS_SPEC>;
#[doc = "Field `WDT_MIS_WDTMIS` reader - Watchdog Masked Interrupt Status"]
pub type WDT_MIS_WDTMIS_R = crate::BitReader;
#[doc = "Field `WDT_MIS_WDTMIS` writer - Watchdog Masked Interrupt Status"]
pub type WDT_MIS_WDTMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Masked Interrupt Status"]
    #[inline(always)]
    pub fn wdt_mis_wdtmis(&self) -> WDT_MIS_WDTMIS_R {
        WDT_MIS_WDTMIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_mis_wdtmis(&mut self) -> WDT_MIS_WDTMIS_W<MIS_SPEC, 0> {
        WDT_MIS_WDTMIS_W::new(self)
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
#[doc = "Watchdog Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
