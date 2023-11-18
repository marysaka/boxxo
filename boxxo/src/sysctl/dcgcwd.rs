#[doc = "Register `DCGCWD` reader"]
pub type R = crate::R<DCGCWD_SPEC>;
#[doc = "Register `DCGCWD` writer"]
pub type W = crate::W<DCGCWD_SPEC>;
#[doc = "Field `SYSCTL_DCGCWD_D0` reader - Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWD_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCWD_D0` writer - Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWD_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCWD_D1` reader - Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWD_D1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCWD_D1` writer - Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWD_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcwd_d0(&self) -> SYSCTL_DCGCWD_D0_R {
        SYSCTL_DCGCWD_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcwd_d1(&self) -> SYSCTL_DCGCWD_D1_R {
        SYSCTL_DCGCWD_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcwd_d0(&mut self) -> SYSCTL_DCGCWD_D0_W<DCGCWD_SPEC, 0> {
        SYSCTL_DCGCWD_D0_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcwd_d1(&mut self) -> SYSCTL_DCGCWD_D1_W<DCGCWD_SPEC, 1> {
        SYSCTL_DCGCWD_D1_W::new(self)
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
#[doc = "Watchdog Timer Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCWD_SPEC;
impl crate::RegisterSpec for DCGCWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcwd::R`](R) reader structure"]
impl crate::Readable for DCGCWD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgcwd::W`](W) writer structure"]
impl crate::Writable for DCGCWD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCWD to value 0"]
impl crate::Resettable for DCGCWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
