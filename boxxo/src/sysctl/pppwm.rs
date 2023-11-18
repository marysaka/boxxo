#[doc = "Register `PPPWM` reader"]
pub type R = crate::R<PPPWM_SPEC>;
#[doc = "Register `PPPWM` writer"]
pub type W = crate::W<PPPWM_SPEC>;
#[doc = "Field `SYSCTL_PPPWM_P0` reader - PWM Module 0 Present"]
pub type SYSCTL_PPPWM_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPPWM_P0` writer - PWM Module 0 Present"]
pub type SYSCTL_PPPWM_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPPWM_P1` reader - PWM Module 1 Present"]
pub type SYSCTL_PPPWM_P1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPPWM_P1` writer - PWM Module 1 Present"]
pub type SYSCTL_PPPWM_P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_pppwm_p0(&self) -> SYSCTL_PPPWM_P0_R {
        SYSCTL_PPPWM_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_pppwm_p1(&self) -> SYSCTL_PPPWM_P1_R {
        SYSCTL_PPPWM_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pppwm_p0(&mut self) -> SYSCTL_PPPWM_P0_W<PPPWM_SPEC, 0> {
        SYSCTL_PPPWM_P0_W::new(self)
    }
    #[doc = "Bit 1 - PWM Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pppwm_p1(&mut self) -> SYSCTL_PPPWM_P1_W<PPPWM_SPEC, 1> {
        SYSCTL_PPPWM_P1_W::new(self)
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
#[doc = "Pulse Width Modulator Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pppwm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pppwm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPPWM_SPEC;
impl crate::RegisterSpec for PPPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pppwm::R`](R) reader structure"]
impl crate::Readable for PPPWM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pppwm::W`](W) writer structure"]
impl crate::Writable for PPPWM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPPWM to value 0"]
impl crate::Resettable for PPPWM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
