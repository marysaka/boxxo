#[doc = "Register `PRWD` reader"]
pub type R = crate::R<PRWD_SPEC>;
#[doc = "Register `PRWD` writer"]
pub type W = crate::W<PRWD_SPEC>;
#[doc = "Field `SYSCTL_PRWD_R0` reader - Watchdog Timer 0 Peripheral Ready"]
pub type SYSCTL_PRWD_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRWD_R0` writer - Watchdog Timer 0 Peripheral Ready"]
pub type SYSCTL_PRWD_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRWD_R1` reader - Watchdog Timer 1 Peripheral Ready"]
pub type SYSCTL_PRWD_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRWD_R1` writer - Watchdog Timer 1 Peripheral Ready"]
pub type SYSCTL_PRWD_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwd_r0(&self) -> SYSCTL_PRWD_R0_R {
        SYSCTL_PRWD_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwd_r1(&self) -> SYSCTL_PRWD_R1_R {
        SYSCTL_PRWD_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer 0 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prwd_r0(&mut self) -> SYSCTL_PRWD_R0_W<PRWD_SPEC, 0> {
        SYSCTL_PRWD_R0_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prwd_r1(&mut self) -> SYSCTL_PRWD_R1_W<PRWD_SPEC, 1> {
        SYSCTL_PRWD_R1_W::new(self)
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
#[doc = "Watchdog Timer Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRWD_SPEC;
impl crate::RegisterSpec for PRWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prwd::R`](R) reader structure"]
impl crate::Readable for PRWD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prwd::W`](W) writer structure"]
impl crate::Writable for PRWD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRWD to value 0"]
impl crate::Resettable for PRWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
