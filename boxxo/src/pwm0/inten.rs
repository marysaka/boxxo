#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `PWM_INTEN_INTPWM0` reader - PWM0 Interrupt Enable"]
pub type PWM_INTEN_INTPWM0_R = crate::BitReader;
#[doc = "Field `PWM_INTEN_INTPWM0` writer - PWM0 Interrupt Enable"]
pub type PWM_INTEN_INTPWM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INTEN_INTPWM1` reader - PWM1 Interrupt Enable"]
pub type PWM_INTEN_INTPWM1_R = crate::BitReader;
#[doc = "Field `PWM_INTEN_INTPWM1` writer - PWM1 Interrupt Enable"]
pub type PWM_INTEN_INTPWM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INTEN_INTPWM2` reader - PWM2 Interrupt Enable"]
pub type PWM_INTEN_INTPWM2_R = crate::BitReader;
#[doc = "Field `PWM_INTEN_INTPWM2` writer - PWM2 Interrupt Enable"]
pub type PWM_INTEN_INTPWM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INTEN_INTPWM3` reader - PWM3 Interrupt Enable"]
pub type PWM_INTEN_INTPWM3_R = crate::BitReader;
#[doc = "Field `PWM_INTEN_INTPWM3` writer - PWM3 Interrupt Enable"]
pub type PWM_INTEN_INTPWM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INTEN_INTFAULT0` reader - Interrupt Fault 0"]
pub type PWM_INTEN_INTFAULT0_R = crate::BitReader;
#[doc = "Field `PWM_INTEN_INTFAULT0` writer - Interrupt Fault 0"]
pub type PWM_INTEN_INTFAULT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INTEN_INTFAULT1` reader - Interrupt Fault 1"]
pub type PWM_INTEN_INTFAULT1_R = crate::BitReader;
#[doc = "Field `PWM_INTEN_INTFAULT1` writer - Interrupt Fault 1"]
pub type PWM_INTEN_INTFAULT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm0(&self) -> PWM_INTEN_INTPWM0_R {
        PWM_INTEN_INTPWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm1(&self) -> PWM_INTEN_INTPWM1_R {
        PWM_INTEN_INTPWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm2(&self) -> PWM_INTEN_INTPWM2_R {
        PWM_INTEN_INTPWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm3(&self) -> PWM_INTEN_INTPWM3_R {
        PWM_INTEN_INTPWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Fault 0"]
    #[inline(always)]
    pub fn pwm_inten_intfault0(&self) -> PWM_INTEN_INTFAULT0_R {
        PWM_INTEN_INTFAULT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Fault 1"]
    #[inline(always)]
    pub fn pwm_inten_intfault1(&self) -> PWM_INTEN_INTFAULT1_R {
        PWM_INTEN_INTFAULT1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_inten_intpwm0(&mut self) -> PWM_INTEN_INTPWM0_W<INTEN_SPEC, 0> {
        PWM_INTEN_INTPWM0_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_inten_intpwm1(&mut self) -> PWM_INTEN_INTPWM1_W<INTEN_SPEC, 1> {
        PWM_INTEN_INTPWM1_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_inten_intpwm2(&mut self) -> PWM_INTEN_INTPWM2_W<INTEN_SPEC, 2> {
        PWM_INTEN_INTPWM2_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_inten_intpwm3(&mut self) -> PWM_INTEN_INTPWM3_W<INTEN_SPEC, 3> {
        PWM_INTEN_INTPWM3_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Fault 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_inten_intfault0(&mut self) -> PWM_INTEN_INTFAULT0_W<INTEN_SPEC, 16> {
        PWM_INTEN_INTFAULT0_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Fault 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_inten_intfault1(&mut self) -> PWM_INTEN_INTFAULT1_W<INTEN_SPEC, 17> {
        PWM_INTEN_INTFAULT1_W::new(self)
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
#[doc = "PWM Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
