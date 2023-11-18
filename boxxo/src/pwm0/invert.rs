#[doc = "Register `INVERT` reader"]
pub type R = crate::R<INVERT_SPEC>;
#[doc = "Register `INVERT` writer"]
pub type W = crate::W<INVERT_SPEC>;
#[doc = "Field `PWM_INVERT_PWM0INV` reader - Invert PWM0 Signal"]
pub type PWM_INVERT_PWM0INV_R = crate::BitReader;
#[doc = "Field `PWM_INVERT_PWM0INV` writer - Invert PWM0 Signal"]
pub type PWM_INVERT_PWM0INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INVERT_PWM1INV` reader - Invert PWM1 Signal"]
pub type PWM_INVERT_PWM1INV_R = crate::BitReader;
#[doc = "Field `PWM_INVERT_PWM1INV` writer - Invert PWM1 Signal"]
pub type PWM_INVERT_PWM1INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INVERT_PWM2INV` reader - Invert PWM2 Signal"]
pub type PWM_INVERT_PWM2INV_R = crate::BitReader;
#[doc = "Field `PWM_INVERT_PWM2INV` writer - Invert PWM2 Signal"]
pub type PWM_INVERT_PWM2INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INVERT_PWM3INV` reader - Invert PWM3 Signal"]
pub type PWM_INVERT_PWM3INV_R = crate::BitReader;
#[doc = "Field `PWM_INVERT_PWM3INV` writer - Invert PWM3 Signal"]
pub type PWM_INVERT_PWM3INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INVERT_PWM4INV` reader - Invert PWM4 Signal"]
pub type PWM_INVERT_PWM4INV_R = crate::BitReader;
#[doc = "Field `PWM_INVERT_PWM4INV` writer - Invert PWM4 Signal"]
pub type PWM_INVERT_PWM4INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INVERT_PWM5INV` reader - Invert PWM5 Signal"]
pub type PWM_INVERT_PWM5INV_R = crate::BitReader;
#[doc = "Field `PWM_INVERT_PWM5INV` writer - Invert PWM5 Signal"]
pub type PWM_INVERT_PWM5INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INVERT_PWM6INV` reader - Invert PWM6 Signal"]
pub type PWM_INVERT_PWM6INV_R = crate::BitReader;
#[doc = "Field `PWM_INVERT_PWM6INV` writer - Invert PWM6 Signal"]
pub type PWM_INVERT_PWM6INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_INVERT_PWM7INV` reader - Invert PWM7 Signal"]
pub type PWM_INVERT_PWM7INV_R = crate::BitReader;
#[doc = "Field `PWM_INVERT_PWM7INV` writer - Invert PWM7 Signal"]
pub type PWM_INVERT_PWM7INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Invert PWM0 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm0inv(&self) -> PWM_INVERT_PWM0INV_R {
        PWM_INVERT_PWM0INV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invert PWM1 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm1inv(&self) -> PWM_INVERT_PWM1INV_R {
        PWM_INVERT_PWM1INV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invert PWM2 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm2inv(&self) -> PWM_INVERT_PWM2INV_R {
        PWM_INVERT_PWM2INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert PWM3 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm3inv(&self) -> PWM_INVERT_PWM3INV_R {
        PWM_INVERT_PWM3INV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Invert PWM4 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm4inv(&self) -> PWM_INVERT_PWM4INV_R {
        PWM_INVERT_PWM4INV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert PWM5 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm5inv(&self) -> PWM_INVERT_PWM5INV_R {
        PWM_INVERT_PWM5INV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Invert PWM6 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm6inv(&self) -> PWM_INVERT_PWM6INV_R {
        PWM_INVERT_PWM6INV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Invert PWM7 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm7inv(&self) -> PWM_INVERT_PWM7INV_R {
        PWM_INVERT_PWM7INV_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invert PWM0 Signal"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_invert_pwm0inv(&mut self) -> PWM_INVERT_PWM0INV_W<INVERT_SPEC, 0> {
        PWM_INVERT_PWM0INV_W::new(self)
    }
    #[doc = "Bit 1 - Invert PWM1 Signal"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_invert_pwm1inv(&mut self) -> PWM_INVERT_PWM1INV_W<INVERT_SPEC, 1> {
        PWM_INVERT_PWM1INV_W::new(self)
    }
    #[doc = "Bit 2 - Invert PWM2 Signal"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_invert_pwm2inv(&mut self) -> PWM_INVERT_PWM2INV_W<INVERT_SPEC, 2> {
        PWM_INVERT_PWM2INV_W::new(self)
    }
    #[doc = "Bit 3 - Invert PWM3 Signal"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_invert_pwm3inv(&mut self) -> PWM_INVERT_PWM3INV_W<INVERT_SPEC, 3> {
        PWM_INVERT_PWM3INV_W::new(self)
    }
    #[doc = "Bit 4 - Invert PWM4 Signal"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_invert_pwm4inv(&mut self) -> PWM_INVERT_PWM4INV_W<INVERT_SPEC, 4> {
        PWM_INVERT_PWM4INV_W::new(self)
    }
    #[doc = "Bit 5 - Invert PWM5 Signal"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_invert_pwm5inv(&mut self) -> PWM_INVERT_PWM5INV_W<INVERT_SPEC, 5> {
        PWM_INVERT_PWM5INV_W::new(self)
    }
    #[doc = "Bit 6 - Invert PWM6 Signal"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_invert_pwm6inv(&mut self) -> PWM_INVERT_PWM6INV_W<INVERT_SPEC, 6> {
        PWM_INVERT_PWM6INV_W::new(self)
    }
    #[doc = "Bit 7 - Invert PWM7 Signal"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_invert_pwm7inv(&mut self) -> PWM_INVERT_PWM7INV_W<INVERT_SPEC, 7> {
        PWM_INVERT_PWM7INV_W::new(self)
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
#[doc = "PWM Output Inversion\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`invert::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`invert::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INVERT_SPEC;
impl crate::RegisterSpec for INVERT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`invert::R`](R) reader structure"]
impl crate::Readable for INVERT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`invert::W`](W) writer structure"]
impl crate::Writable for INVERT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INVERT to value 0"]
impl crate::Resettable for INVERT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
