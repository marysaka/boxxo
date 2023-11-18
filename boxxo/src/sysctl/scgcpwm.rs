#[doc = "Register `SCGCPWM` reader"]
pub type R = crate::R<SCGCPWM_SPEC>;
#[doc = "Register `SCGCPWM` writer"]
pub type W = crate::W<SCGCPWM_SPEC>;
#[doc = "Field `SYSCTL_SCGCPWM_S0` reader - PWM Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCPWM_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCPWM_S0` writer - PWM Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCPWM_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCPWM_S1` reader - PWM Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCPWM_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCPWM_S1` writer - PWM Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCPWM_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcpwm_s0(&self) -> SYSCTL_SCGCPWM_S0_R {
        SYSCTL_SCGCPWM_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcpwm_s1(&self) -> SYSCTL_SCGCPWM_S1_R {
        SYSCTL_SCGCPWM_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcpwm_s0(&mut self) -> SYSCTL_SCGCPWM_S0_W<SCGCPWM_SPEC, 0> {
        SYSCTL_SCGCPWM_S0_W::new(self)
    }
    #[doc = "Bit 1 - PWM Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcpwm_s1(&mut self) -> SYSCTL_SCGCPWM_S1_W<SCGCPWM_SPEC, 1> {
        SYSCTL_SCGCPWM_S1_W::new(self)
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
#[doc = "Pulse Width Modulator Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcpwm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcpwm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCPWM_SPEC;
impl crate::RegisterSpec for SCGCPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcpwm::R`](R) reader structure"]
impl crate::Readable for SCGCPWM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcpwm::W`](W) writer structure"]
impl crate::Writable for SCGCPWM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCPWM to value 0"]
impl crate::Resettable for SCGCPWM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
