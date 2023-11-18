#[doc = "Register `ISC` reader"]
pub type R = crate::R<ISC_SPEC>;
#[doc = "Register `ISC` writer"]
pub type W = crate::W<ISC_SPEC>;
#[doc = "Field `PWM_ISC_INTPWM0` reader - PWM0 Interrupt Status"]
pub type PWM_ISC_INTPWM0_R = crate::BitReader;
#[doc = "Field `PWM_ISC_INTPWM0` writer - PWM0 Interrupt Status"]
pub type PWM_ISC_INTPWM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ISC_INTPWM1` reader - PWM1 Interrupt Status"]
pub type PWM_ISC_INTPWM1_R = crate::BitReader;
#[doc = "Field `PWM_ISC_INTPWM1` writer - PWM1 Interrupt Status"]
pub type PWM_ISC_INTPWM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ISC_INTPWM2` reader - PWM2 Interrupt Status"]
pub type PWM_ISC_INTPWM2_R = crate::BitReader;
#[doc = "Field `PWM_ISC_INTPWM2` writer - PWM2 Interrupt Status"]
pub type PWM_ISC_INTPWM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ISC_INTPWM3` reader - PWM3 Interrupt Status"]
pub type PWM_ISC_INTPWM3_R = crate::BitReader;
#[doc = "Field `PWM_ISC_INTPWM3` writer - PWM3 Interrupt Status"]
pub type PWM_ISC_INTPWM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ISC_INTFAULT0` reader - FAULT0 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT0_R = crate::BitReader;
#[doc = "Field `PWM_ISC_INTFAULT0` writer - FAULT0 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ISC_INTFAULT1` reader - FAULT1 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT1_R = crate::BitReader;
#[doc = "Field `PWM_ISC_INTFAULT1` writer - FAULT1 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm0(&self) -> PWM_ISC_INTPWM0_R {
        PWM_ISC_INTPWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm1(&self) -> PWM_ISC_INTPWM1_R {
        PWM_ISC_INTPWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm2(&self) -> PWM_ISC_INTPWM2_R {
        PWM_ISC_INTPWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm3(&self) -> PWM_ISC_INTPWM3_R {
        PWM_ISC_INTPWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - FAULT0 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault0(&self) -> PWM_ISC_INTFAULT0_R {
        PWM_ISC_INTFAULT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FAULT1 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault1(&self) -> PWM_ISC_INTFAULT1_R {
        PWM_ISC_INTFAULT1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_isc_intpwm0(&mut self) -> PWM_ISC_INTPWM0_W<ISC_SPEC, 0> {
        PWM_ISC_INTPWM0_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_isc_intpwm1(&mut self) -> PWM_ISC_INTPWM1_W<ISC_SPEC, 1> {
        PWM_ISC_INTPWM1_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_isc_intpwm2(&mut self) -> PWM_ISC_INTPWM2_W<ISC_SPEC, 2> {
        PWM_ISC_INTPWM2_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_isc_intpwm3(&mut self) -> PWM_ISC_INTPWM3_W<ISC_SPEC, 3> {
        PWM_ISC_INTPWM3_W::new(self)
    }
    #[doc = "Bit 16 - FAULT0 Interrupt Asserted"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_isc_intfault0(&mut self) -> PWM_ISC_INTFAULT0_W<ISC_SPEC, 16> {
        PWM_ISC_INTFAULT0_W::new(self)
    }
    #[doc = "Bit 17 - FAULT1 Interrupt Asserted"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_isc_intfault1(&mut self) -> PWM_ISC_INTFAULT1_W<ISC_SPEC, 17> {
        PWM_ISC_INTFAULT1_W::new(self)
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
#[doc = "PWM Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISC_SPEC;
impl crate::RegisterSpec for ISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isc::R`](R) reader structure"]
impl crate::Readable for ISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`isc::W`](W) writer structure"]
impl crate::Writable for ISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISC to value 0"]
impl crate::Resettable for ISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
