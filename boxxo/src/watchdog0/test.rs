#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Field `WDT_TEST_STALL` reader - Watchdog Stall Enable"]
pub type WDT_TEST_STALL_R = crate::BitReader;
#[doc = "Field `WDT_TEST_STALL` writer - Watchdog Stall Enable"]
pub type WDT_TEST_STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 8 - Watchdog Stall Enable"]
    #[inline(always)]
    pub fn wdt_test_stall(&self) -> WDT_TEST_STALL_R {
        WDT_TEST_STALL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Watchdog Stall Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_test_stall(&mut self) -> WDT_TEST_STALL_W<TEST_SPEC, 8> {
        WDT_TEST_STALL_W::new(self)
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
#[doc = "Watchdog Test\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
