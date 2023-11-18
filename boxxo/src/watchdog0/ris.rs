#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `WDT_RIS_WDTRIS` reader - Watchdog Raw Interrupt Status"]
pub type WDT_RIS_WDTRIS_R = crate::BitReader;
#[doc = "Field `WDT_RIS_WDTRIS` writer - Watchdog Raw Interrupt Status"]
pub type WDT_RIS_WDTRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Raw Interrupt Status"]
    #[inline(always)]
    pub fn wdt_ris_wdtris(&self) -> WDT_RIS_WDTRIS_R {
        WDT_RIS_WDTRIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ris_wdtris(&mut self) -> WDT_RIS_WDTRIS_W<RIS_SPEC, 0> {
        WDT_RIS_WDTRIS_W::new(self)
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
#[doc = "Watchdog Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
