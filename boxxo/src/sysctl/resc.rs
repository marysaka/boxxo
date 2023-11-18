#[doc = "Register `RESC` reader"]
pub type R = crate::R<RESC_SPEC>;
#[doc = "Register `RESC` writer"]
pub type W = crate::W<RESC_SPEC>;
#[doc = "Field `SYSCTL_RESC_EXT` reader - External Reset"]
pub type SYSCTL_RESC_EXT_R = crate::BitReader;
#[doc = "Field `SYSCTL_RESC_EXT` writer - External Reset"]
pub type SYSCTL_RESC_EXT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RESC_POR` reader - Power-On Reset"]
pub type SYSCTL_RESC_POR_R = crate::BitReader;
#[doc = "Field `SYSCTL_RESC_POR` writer - Power-On Reset"]
pub type SYSCTL_RESC_POR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RESC_BOR` reader - Brown-Out Reset"]
pub type SYSCTL_RESC_BOR_R = crate::BitReader;
#[doc = "Field `SYSCTL_RESC_BOR` writer - Brown-Out Reset"]
pub type SYSCTL_RESC_BOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RESC_WDT0` reader - Watchdog Timer 0 Reset"]
pub type SYSCTL_RESC_WDT0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RESC_WDT0` writer - Watchdog Timer 0 Reset"]
pub type SYSCTL_RESC_WDT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RESC_SW` reader - Software Reset"]
pub type SYSCTL_RESC_SW_R = crate::BitReader;
#[doc = "Field `SYSCTL_RESC_SW` writer - Software Reset"]
pub type SYSCTL_RESC_SW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RESC_WDT1` reader - Watchdog Timer 1 Reset"]
pub type SYSCTL_RESC_WDT1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RESC_WDT1` writer - Watchdog Timer 1 Reset"]
pub type SYSCTL_RESC_WDT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RESC_MOSCFAIL` reader - MOSC Failure Reset"]
pub type SYSCTL_RESC_MOSCFAIL_R = crate::BitReader;
#[doc = "Field `SYSCTL_RESC_MOSCFAIL` writer - MOSC Failure Reset"]
pub type SYSCTL_RESC_MOSCFAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - External Reset"]
    #[inline(always)]
    pub fn sysctl_resc_ext(&self) -> SYSCTL_RESC_EXT_R {
        SYSCTL_RESC_EXT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power-On Reset"]
    #[inline(always)]
    pub fn sysctl_resc_por(&self) -> SYSCTL_RESC_POR_R {
        SYSCTL_RESC_POR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown-Out Reset"]
    #[inline(always)]
    pub fn sysctl_resc_bor(&self) -> SYSCTL_RESC_BOR_R {
        SYSCTL_RESC_BOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt0(&self) -> SYSCTL_RESC_WDT0_R {
        SYSCTL_RESC_WDT0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Reset"]
    #[inline(always)]
    pub fn sysctl_resc_sw(&self) -> SYSCTL_RESC_SW_R {
        SYSCTL_RESC_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Watchdog Timer 1 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt1(&self) -> SYSCTL_RESC_WDT1_R {
        SYSCTL_RESC_WDT1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - MOSC Failure Reset"]
    #[inline(always)]
    pub fn sysctl_resc_moscfail(&self) -> SYSCTL_RESC_MOSCFAIL_R {
        SYSCTL_RESC_MOSCFAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_resc_ext(&mut self) -> SYSCTL_RESC_EXT_W<RESC_SPEC, 0> {
        SYSCTL_RESC_EXT_W::new(self)
    }
    #[doc = "Bit 1 - Power-On Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_resc_por(&mut self) -> SYSCTL_RESC_POR_W<RESC_SPEC, 1> {
        SYSCTL_RESC_POR_W::new(self)
    }
    #[doc = "Bit 2 - Brown-Out Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_resc_bor(&mut self) -> SYSCTL_RESC_BOR_W<RESC_SPEC, 2> {
        SYSCTL_RESC_BOR_W::new(self)
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_resc_wdt0(&mut self) -> SYSCTL_RESC_WDT0_W<RESC_SPEC, 3> {
        SYSCTL_RESC_WDT0_W::new(self)
    }
    #[doc = "Bit 4 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_resc_sw(&mut self) -> SYSCTL_RESC_SW_W<RESC_SPEC, 4> {
        SYSCTL_RESC_SW_W::new(self)
    }
    #[doc = "Bit 5 - Watchdog Timer 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_resc_wdt1(&mut self) -> SYSCTL_RESC_WDT1_W<RESC_SPEC, 5> {
        SYSCTL_RESC_WDT1_W::new(self)
    }
    #[doc = "Bit 16 - MOSC Failure Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_resc_moscfail(&mut self) -> SYSCTL_RESC_MOSCFAIL_W<RESC_SPEC, 16> {
        SYSCTL_RESC_MOSCFAIL_W::new(self)
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
#[doc = "Reset Cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESC_SPEC;
impl crate::RegisterSpec for RESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resc::R`](R) reader structure"]
impl crate::Readable for RESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`resc::W`](W) writer structure"]
impl crate::Writable for RESC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESC to value 0"]
impl crate::Resettable for RESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
