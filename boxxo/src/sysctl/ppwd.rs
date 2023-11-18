#[doc = "Register `PPWD` reader"]
pub type R = crate::R<PPWD_SPEC>;
#[doc = "Register `PPWD` writer"]
pub type W = crate::W<PPWD_SPEC>;
#[doc = "Field `SYSCTL_PPWD_P0` reader - Watchdog Timer 0 Present"]
pub type SYSCTL_PPWD_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPWD_P0` writer - Watchdog Timer 0 Present"]
pub type SYSCTL_PPWD_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPWD_P1` reader - Watchdog Timer 1 Present"]
pub type SYSCTL_PPWD_P1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPWD_P1` writer - Watchdog Timer 1 Present"]
pub type SYSCTL_PPWD_P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppwd_p0(&self) -> SYSCTL_PPWD_P0_R {
        SYSCTL_PPWD_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppwd_p1(&self) -> SYSCTL_PPWD_P1_R {
        SYSCTL_PPWD_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppwd_p0(&mut self) -> SYSCTL_PPWD_P0_W<PPWD_SPEC, 0> {
        SYSCTL_PPWD_P0_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppwd_p1(&mut self) -> SYSCTL_PPWD_P1_W<PPWD_SPEC, 1> {
        SYSCTL_PPWD_P1_W::new(self)
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
#[doc = "Watchdog Timer Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPWD_SPEC;
impl crate::RegisterSpec for PPWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppwd::R`](R) reader structure"]
impl crate::Readable for PPWD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppwd::W`](W) writer structure"]
impl crate::Writable for PPWD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPWD to value 0"]
impl crate::Resettable for PPWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
