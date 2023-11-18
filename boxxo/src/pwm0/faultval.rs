#[doc = "Register `FAULTVAL` reader"]
pub type R = crate::R<FAULTVAL_SPEC>;
#[doc = "Register `FAULTVAL` writer"]
pub type W = crate::W<FAULTVAL_SPEC>;
#[doc = "Field `PWM_FAULTVAL_PWM0` reader - PWM0 Fault Value"]
pub type PWM_FAULTVAL_PWM0_R = crate::BitReader;
#[doc = "Field `PWM_FAULTVAL_PWM0` writer - PWM0 Fault Value"]
pub type PWM_FAULTVAL_PWM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULTVAL_PWM1` reader - PWM1 Fault Value"]
pub type PWM_FAULTVAL_PWM1_R = crate::BitReader;
#[doc = "Field `PWM_FAULTVAL_PWM1` writer - PWM1 Fault Value"]
pub type PWM_FAULTVAL_PWM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULTVAL_PWM2` reader - PWM2 Fault Value"]
pub type PWM_FAULTVAL_PWM2_R = crate::BitReader;
#[doc = "Field `PWM_FAULTVAL_PWM2` writer - PWM2 Fault Value"]
pub type PWM_FAULTVAL_PWM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULTVAL_PWM3` reader - PWM3 Fault Value"]
pub type PWM_FAULTVAL_PWM3_R = crate::BitReader;
#[doc = "Field `PWM_FAULTVAL_PWM3` writer - PWM3 Fault Value"]
pub type PWM_FAULTVAL_PWM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULTVAL_PWM4` reader - PWM4 Fault Value"]
pub type PWM_FAULTVAL_PWM4_R = crate::BitReader;
#[doc = "Field `PWM_FAULTVAL_PWM4` writer - PWM4 Fault Value"]
pub type PWM_FAULTVAL_PWM4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULTVAL_PWM5` reader - PWM5 Fault Value"]
pub type PWM_FAULTVAL_PWM5_R = crate::BitReader;
#[doc = "Field `PWM_FAULTVAL_PWM5` writer - PWM5 Fault Value"]
pub type PWM_FAULTVAL_PWM5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULTVAL_PWM6` reader - PWM6 Fault Value"]
pub type PWM_FAULTVAL_PWM6_R = crate::BitReader;
#[doc = "Field `PWM_FAULTVAL_PWM6` writer - PWM6 Fault Value"]
pub type PWM_FAULTVAL_PWM6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULTVAL_PWM7` reader - PWM7 Fault Value"]
pub type PWM_FAULTVAL_PWM7_R = crate::BitReader;
#[doc = "Field `PWM_FAULTVAL_PWM7` writer - PWM7 Fault Value"]
pub type PWM_FAULTVAL_PWM7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM0 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm0(&self) -> PWM_FAULTVAL_PWM0_R {
        PWM_FAULTVAL_PWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm1(&self) -> PWM_FAULTVAL_PWM1_R {
        PWM_FAULTVAL_PWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm2(&self) -> PWM_FAULTVAL_PWM2_R {
        PWM_FAULTVAL_PWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm3(&self) -> PWM_FAULTVAL_PWM3_R {
        PWM_FAULTVAL_PWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm4(&self) -> PWM_FAULTVAL_PWM4_R {
        PWM_FAULTVAL_PWM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM5 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm5(&self) -> PWM_FAULTVAL_PWM5_R {
        PWM_FAULTVAL_PWM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm6(&self) -> PWM_FAULTVAL_PWM6_R {
        PWM_FAULTVAL_PWM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM7 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm7(&self) -> PWM_FAULTVAL_PWM7_R {
        PWM_FAULTVAL_PWM7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Fault Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_faultval_pwm0(&mut self) -> PWM_FAULTVAL_PWM0_W<FAULTVAL_SPEC, 0> {
        PWM_FAULTVAL_PWM0_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 Fault Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_faultval_pwm1(&mut self) -> PWM_FAULTVAL_PWM1_W<FAULTVAL_SPEC, 1> {
        PWM_FAULTVAL_PWM1_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 Fault Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_faultval_pwm2(&mut self) -> PWM_FAULTVAL_PWM2_W<FAULTVAL_SPEC, 2> {
        PWM_FAULTVAL_PWM2_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 Fault Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_faultval_pwm3(&mut self) -> PWM_FAULTVAL_PWM3_W<FAULTVAL_SPEC, 3> {
        PWM_FAULTVAL_PWM3_W::new(self)
    }
    #[doc = "Bit 4 - PWM4 Fault Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_faultval_pwm4(&mut self) -> PWM_FAULTVAL_PWM4_W<FAULTVAL_SPEC, 4> {
        PWM_FAULTVAL_PWM4_W::new(self)
    }
    #[doc = "Bit 5 - PWM5 Fault Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_faultval_pwm5(&mut self) -> PWM_FAULTVAL_PWM5_W<FAULTVAL_SPEC, 5> {
        PWM_FAULTVAL_PWM5_W::new(self)
    }
    #[doc = "Bit 6 - PWM6 Fault Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_faultval_pwm6(&mut self) -> PWM_FAULTVAL_PWM6_W<FAULTVAL_SPEC, 6> {
        PWM_FAULTVAL_PWM6_W::new(self)
    }
    #[doc = "Bit 7 - PWM7 Fault Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_faultval_pwm7(&mut self) -> PWM_FAULTVAL_PWM7_W<FAULTVAL_SPEC, 7> {
        PWM_FAULTVAL_PWM7_W::new(self)
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
#[doc = "PWM Fault Condition Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faultval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faultval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FAULTVAL_SPEC;
impl crate::RegisterSpec for FAULTVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`faultval::R`](R) reader structure"]
impl crate::Readable for FAULTVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`faultval::W`](W) writer structure"]
impl crate::Writable for FAULTVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAULTVAL to value 0"]
impl crate::Resettable for FAULTVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
