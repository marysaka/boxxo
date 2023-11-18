#[doc = "Register `SCGCWD` reader"]
pub type R = crate::R<SCGCWD_SPEC>;
#[doc = "Register `SCGCWD` writer"]
pub type W = crate::W<SCGCWD_SPEC>;
#[doc = "Field `SYSCTL_SCGCWD_S0` reader - Watchdog Timer 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWD_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCWD_S0` writer - Watchdog Timer 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWD_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCWD_S1` reader - Watchdog Timer 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWD_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCWD_S1` writer - Watchdog Timer 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWD_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwd_s0(&self) -> SYSCTL_SCGCWD_S0_R {
        SYSCTL_SCGCWD_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwd_s1(&self) -> SYSCTL_SCGCWD_S1_R {
        SYSCTL_SCGCWD_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcwd_s0(&mut self) -> SYSCTL_SCGCWD_S0_W<SCGCWD_SPEC, 0> {
        SYSCTL_SCGCWD_S0_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcwd_s1(&mut self) -> SYSCTL_SCGCWD_S1_W<SCGCWD_SPEC, 1> {
        SYSCTL_SCGCWD_S1_W::new(self)
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
#[doc = "Watchdog Timer Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCWD_SPEC;
impl crate::RegisterSpec for SCGCWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcwd::R`](R) reader structure"]
impl crate::Readable for SCGCWD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcwd::W`](W) writer structure"]
impl crate::Writable for SCGCWD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCWD to value 0"]
impl crate::Resettable for SCGCWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
