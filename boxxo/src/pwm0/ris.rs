#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `PWM_RIS_INTPWM0` reader - PWM0 Interrupt Asserted"]
pub type PWM_RIS_INTPWM0_R = crate::BitReader;
#[doc = "Field `PWM_RIS_INTPWM0` writer - PWM0 Interrupt Asserted"]
pub type PWM_RIS_INTPWM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_RIS_INTPWM1` reader - PWM1 Interrupt Asserted"]
pub type PWM_RIS_INTPWM1_R = crate::BitReader;
#[doc = "Field `PWM_RIS_INTPWM1` writer - PWM1 Interrupt Asserted"]
pub type PWM_RIS_INTPWM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_RIS_INTPWM2` reader - PWM2 Interrupt Asserted"]
pub type PWM_RIS_INTPWM2_R = crate::BitReader;
#[doc = "Field `PWM_RIS_INTPWM2` writer - PWM2 Interrupt Asserted"]
pub type PWM_RIS_INTPWM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_RIS_INTPWM3` reader - PWM3 Interrupt Asserted"]
pub type PWM_RIS_INTPWM3_R = crate::BitReader;
#[doc = "Field `PWM_RIS_INTPWM3` writer - PWM3 Interrupt Asserted"]
pub type PWM_RIS_INTPWM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_RIS_INTFAULT0` reader - Interrupt Fault PWM 0"]
pub type PWM_RIS_INTFAULT0_R = crate::BitReader;
#[doc = "Field `PWM_RIS_INTFAULT0` writer - Interrupt Fault PWM 0"]
pub type PWM_RIS_INTFAULT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_RIS_INTFAULT1` reader - Interrupt Fault PWM 1"]
pub type PWM_RIS_INTFAULT1_R = crate::BitReader;
#[doc = "Field `PWM_RIS_INTFAULT1` writer - Interrupt Fault PWM 1"]
pub type PWM_RIS_INTFAULT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM0 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm0(&self) -> PWM_RIS_INTPWM0_R {
        PWM_RIS_INTPWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm1(&self) -> PWM_RIS_INTPWM1_R {
        PWM_RIS_INTPWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm2(&self) -> PWM_RIS_INTPWM2_R {
        PWM_RIS_INTPWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm3(&self) -> PWM_RIS_INTPWM3_R {
        PWM_RIS_INTPWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Fault PWM 0"]
    #[inline(always)]
    pub fn pwm_ris_intfault0(&self) -> PWM_RIS_INTFAULT0_R {
        PWM_RIS_INTFAULT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Fault PWM 1"]
    #[inline(always)]
    pub fn pwm_ris_intfault1(&self) -> PWM_RIS_INTFAULT1_R {
        PWM_RIS_INTFAULT1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Interrupt Asserted"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ris_intpwm0(&mut self) -> PWM_RIS_INTPWM0_W<RIS_SPEC, 0> {
        PWM_RIS_INTPWM0_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Asserted"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ris_intpwm1(&mut self) -> PWM_RIS_INTPWM1_W<RIS_SPEC, 1> {
        PWM_RIS_INTPWM1_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Asserted"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ris_intpwm2(&mut self) -> PWM_RIS_INTPWM2_W<RIS_SPEC, 2> {
        PWM_RIS_INTPWM2_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Asserted"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ris_intpwm3(&mut self) -> PWM_RIS_INTPWM3_W<RIS_SPEC, 3> {
        PWM_RIS_INTPWM3_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Fault PWM 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ris_intfault0(&mut self) -> PWM_RIS_INTFAULT0_W<RIS_SPEC, 16> {
        PWM_RIS_INTFAULT0_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Fault PWM 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ris_intfault1(&mut self) -> PWM_RIS_INTFAULT1_W<RIS_SPEC, 17> {
        PWM_RIS_INTFAULT1_W::new(self)
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
#[doc = "PWM Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
