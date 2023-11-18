#[doc = "Register `RCGCPWM` reader"]
pub type R = crate::R<RCGCPWM_SPEC>;
#[doc = "Register `RCGCPWM` writer"]
pub type W = crate::W<RCGCPWM_SPEC>;
#[doc = "Field `SYSCTL_RCGCPWM_R0` reader - PWM Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCPWM_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCPWM_R0` writer - PWM Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCPWM_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCPWM_R1` reader - PWM Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCPWM_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCPWM_R1` writer - PWM Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCPWM_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcpwm_r0(&self) -> SYSCTL_RCGCPWM_R0_R {
        SYSCTL_RCGCPWM_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcpwm_r1(&self) -> SYSCTL_RCGCPWM_R1_R {
        SYSCTL_RCGCPWM_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcpwm_r0(&mut self) -> SYSCTL_RCGCPWM_R0_W<RCGCPWM_SPEC, 0> {
        SYSCTL_RCGCPWM_R0_W::new(self)
    }
    #[doc = "Bit 1 - PWM Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcpwm_r1(&mut self) -> SYSCTL_RCGCPWM_R1_W<RCGCPWM_SPEC, 1> {
        SYSCTL_RCGCPWM_R1_W::new(self)
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
#[doc = "Pulse Width Modulator Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcpwm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcpwm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGCPWM_SPEC;
impl crate::RegisterSpec for RCGCPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgcpwm::R`](R) reader structure"]
impl crate::Readable for RCGCPWM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgcpwm::W`](W) writer structure"]
impl crate::Writable for RCGCPWM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCPWM to value 0"]
impl crate::Resettable for RCGCPWM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
