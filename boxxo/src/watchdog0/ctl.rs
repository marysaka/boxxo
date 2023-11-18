#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `WDT_CTL_INTEN` reader - Watchdog Interrupt Enable"]
pub type WDT_CTL_INTEN_R = crate::BitReader;
#[doc = "Field `WDT_CTL_INTEN` writer - Watchdog Interrupt Enable"]
pub type WDT_CTL_INTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_CTL_RESEN` reader - Watchdog Reset Enable"]
pub type WDT_CTL_RESEN_R = crate::BitReader;
#[doc = "Field `WDT_CTL_RESEN` writer - Watchdog Reset Enable"]
pub type WDT_CTL_RESEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_CTL_INTTYPE` reader - Watchdog Interrupt Type"]
pub type WDT_CTL_INTTYPE_R = crate::BitReader;
#[doc = "Field `WDT_CTL_INTTYPE` writer - Watchdog Interrupt Type"]
pub type WDT_CTL_INTTYPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_CTL_WRC` reader - Write Complete"]
pub type WDT_CTL_WRC_R = crate::BitReader;
#[doc = "Field `WDT_CTL_WRC` writer - Write Complete"]
pub type WDT_CTL_WRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdt_ctl_inten(&self) -> WDT_CTL_INTEN_R {
        WDT_CTL_INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdt_ctl_resen(&self) -> WDT_CTL_RESEN_R {
        WDT_CTL_RESEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog Interrupt Type"]
    #[inline(always)]
    pub fn wdt_ctl_inttype(&self) -> WDT_CTL_INTTYPE_R {
        WDT_CTL_INTTYPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Complete"]
    #[inline(always)]
    pub fn wdt_ctl_wrc(&self) -> WDT_CTL_WRC_R {
        WDT_CTL_WRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctl_inten(&mut self) -> WDT_CTL_INTEN_W<CTL_SPEC, 0> {
        WDT_CTL_INTEN_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctl_resen(&mut self) -> WDT_CTL_RESEN_W<CTL_SPEC, 1> {
        WDT_CTL_RESEN_W::new(self)
    }
    #[doc = "Bit 2 - Watchdog Interrupt Type"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctl_inttype(&mut self) -> WDT_CTL_INTTYPE_W<CTL_SPEC, 2> {
        WDT_CTL_INTTYPE_W::new(self)
    }
    #[doc = "Bit 31 - Write Complete"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctl_wrc(&mut self) -> WDT_CTL_WRC_W<CTL_SPEC, 31> {
        WDT_CTL_WRC_W::new(self)
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
#[doc = "Watchdog Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
