#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<ENABLE_SPEC>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<ENABLE_SPEC>;
#[doc = "Field `PWM_ENABLE_PWM0EN` reader - PWM0 Output Enable"]
pub type PWM_ENABLE_PWM0EN_R = crate::BitReader;
#[doc = "Field `PWM_ENABLE_PWM0EN` writer - PWM0 Output Enable"]
pub type PWM_ENABLE_PWM0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ENABLE_PWM1EN` reader - PWM1 Output Enable"]
pub type PWM_ENABLE_PWM1EN_R = crate::BitReader;
#[doc = "Field `PWM_ENABLE_PWM1EN` writer - PWM1 Output Enable"]
pub type PWM_ENABLE_PWM1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ENABLE_PWM2EN` reader - PWM2 Output Enable"]
pub type PWM_ENABLE_PWM2EN_R = crate::BitReader;
#[doc = "Field `PWM_ENABLE_PWM2EN` writer - PWM2 Output Enable"]
pub type PWM_ENABLE_PWM2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ENABLE_PWM3EN` reader - PWM3 Output Enable"]
pub type PWM_ENABLE_PWM3EN_R = crate::BitReader;
#[doc = "Field `PWM_ENABLE_PWM3EN` writer - PWM3 Output Enable"]
pub type PWM_ENABLE_PWM3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ENABLE_PWM4EN` reader - PWM4 Output Enable"]
pub type PWM_ENABLE_PWM4EN_R = crate::BitReader;
#[doc = "Field `PWM_ENABLE_PWM4EN` writer - PWM4 Output Enable"]
pub type PWM_ENABLE_PWM4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ENABLE_PWM5EN` reader - PWM5 Output Enable"]
pub type PWM_ENABLE_PWM5EN_R = crate::BitReader;
#[doc = "Field `PWM_ENABLE_PWM5EN` writer - PWM5 Output Enable"]
pub type PWM_ENABLE_PWM5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ENABLE_PWM6EN` reader - PWM6 Output Enable"]
pub type PWM_ENABLE_PWM6EN_R = crate::BitReader;
#[doc = "Field `PWM_ENABLE_PWM6EN` writer - PWM6 Output Enable"]
pub type PWM_ENABLE_PWM6EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_ENABLE_PWM7EN` reader - PWM7 Output Enable"]
pub type PWM_ENABLE_PWM7EN_R = crate::BitReader;
#[doc = "Field `PWM_ENABLE_PWM7EN` writer - PWM7 Output Enable"]
pub type PWM_ENABLE_PWM7EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM0 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm0en(&self) -> PWM_ENABLE_PWM0EN_R {
        PWM_ENABLE_PWM0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm1en(&self) -> PWM_ENABLE_PWM1EN_R {
        PWM_ENABLE_PWM1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm2en(&self) -> PWM_ENABLE_PWM2EN_R {
        PWM_ENABLE_PWM2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm3en(&self) -> PWM_ENABLE_PWM3EN_R {
        PWM_ENABLE_PWM3EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm4en(&self) -> PWM_ENABLE_PWM4EN_R {
        PWM_ENABLE_PWM4EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM5 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm5en(&self) -> PWM_ENABLE_PWM5EN_R {
        PWM_ENABLE_PWM5EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm6en(&self) -> PWM_ENABLE_PWM6EN_R {
        PWM_ENABLE_PWM6EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM7 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm7en(&self) -> PWM_ENABLE_PWM7EN_R {
        PWM_ENABLE_PWM7EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enable_pwm0en(&mut self) -> PWM_ENABLE_PWM0EN_W<ENABLE_SPEC, 0> {
        PWM_ENABLE_PWM0EN_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enable_pwm1en(&mut self) -> PWM_ENABLE_PWM1EN_W<ENABLE_SPEC, 1> {
        PWM_ENABLE_PWM1EN_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enable_pwm2en(&mut self) -> PWM_ENABLE_PWM2EN_W<ENABLE_SPEC, 2> {
        PWM_ENABLE_PWM2EN_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enable_pwm3en(&mut self) -> PWM_ENABLE_PWM3EN_W<ENABLE_SPEC, 3> {
        PWM_ENABLE_PWM3EN_W::new(self)
    }
    #[doc = "Bit 4 - PWM4 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enable_pwm4en(&mut self) -> PWM_ENABLE_PWM4EN_W<ENABLE_SPEC, 4> {
        PWM_ENABLE_PWM4EN_W::new(self)
    }
    #[doc = "Bit 5 - PWM5 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enable_pwm5en(&mut self) -> PWM_ENABLE_PWM5EN_W<ENABLE_SPEC, 5> {
        PWM_ENABLE_PWM5EN_W::new(self)
    }
    #[doc = "Bit 6 - PWM6 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enable_pwm6en(&mut self) -> PWM_ENABLE_PWM6EN_W<ENABLE_SPEC, 6> {
        PWM_ENABLE_PWM6EN_W::new(self)
    }
    #[doc = "Bit 7 - PWM7 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enable_pwm7en(&mut self) -> PWM_ENABLE_PWM7EN_W<ENABLE_SPEC, 7> {
        PWM_ENABLE_PWM7EN_W::new(self)
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
#[doc = "PWM Output Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
