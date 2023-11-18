#[doc = "Register `DCGCPWM` reader"]
pub type R = crate::R<DCGCPWM_SPEC>;
#[doc = "Register `DCGCPWM` writer"]
pub type W = crate::W<DCGCPWM_SPEC>;
#[doc = "Field `SYSCTL_DCGCPWM_D0` reader - PWM Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCPWM_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCPWM_D0` writer - PWM Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCPWM_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCPWM_D1` reader - PWM Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCPWM_D1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCPWM_D1` writer - PWM Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCPWM_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcpwm_d0(&self) -> SYSCTL_DCGCPWM_D0_R {
        SYSCTL_DCGCPWM_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcpwm_d1(&self) -> SYSCTL_DCGCPWM_D1_R {
        SYSCTL_DCGCPWM_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcpwm_d0(&mut self) -> SYSCTL_DCGCPWM_D0_W<DCGCPWM_SPEC, 0> {
        SYSCTL_DCGCPWM_D0_W::new(self)
    }
    #[doc = "Bit 1 - PWM Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcpwm_d1(&mut self) -> SYSCTL_DCGCPWM_D1_W<DCGCPWM_SPEC, 1> {
        SYSCTL_DCGCPWM_D1_W::new(self)
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
#[doc = "Pulse Width Modulator Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcpwm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcpwm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCPWM_SPEC;
impl crate::RegisterSpec for DCGCPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcpwm::R`](R) reader structure"]
impl crate::Readable for DCGCPWM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgcpwm::W`](W) writer structure"]
impl crate::Writable for DCGCPWM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCPWM to value 0"]
impl crate::Resettable for DCGCPWM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
