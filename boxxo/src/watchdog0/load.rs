#[doc = "Register `LOAD` reader"]
pub type R = crate::R<LOAD_SPEC>;
#[doc = "Register `LOAD` writer"]
pub type W = crate::W<LOAD_SPEC>;
#[doc = "Field `WDT_LOAD` reader - Watchdog Load Value"]
pub type WDT_LOAD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_LOAD` writer - Watchdog Load Value"]
pub type WDT_LOAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Watchdog Load Value"]
    #[inline(always)]
    pub fn wdt_load(&self) -> WDT_LOAD_R {
        WDT_LOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Load Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_load(&mut self) -> WDT_LOAD_W<LOAD_SPEC, 0> {
        WDT_LOAD_W::new(self)
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
#[doc = "Watchdog Load\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOAD_SPEC;
impl crate::RegisterSpec for LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load::R`](R) reader structure"]
impl crate::Readable for LOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`load::W`](W) writer structure"]
impl crate::Writable for LOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOAD to value 0"]
impl crate::Resettable for LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
