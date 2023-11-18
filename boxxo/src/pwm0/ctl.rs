#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `PWM_CTL_GLOBALSYNC0` reader - Update PWM Generator 0"]
pub type PWM_CTL_GLOBALSYNC0_R = crate::BitReader;
#[doc = "Field `PWM_CTL_GLOBALSYNC0` writer - Update PWM Generator 0"]
pub type PWM_CTL_GLOBALSYNC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_CTL_GLOBALSYNC1` reader - Update PWM Generator 1"]
pub type PWM_CTL_GLOBALSYNC1_R = crate::BitReader;
#[doc = "Field `PWM_CTL_GLOBALSYNC1` writer - Update PWM Generator 1"]
pub type PWM_CTL_GLOBALSYNC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_CTL_GLOBALSYNC2` reader - Update PWM Generator 2"]
pub type PWM_CTL_GLOBALSYNC2_R = crate::BitReader;
#[doc = "Field `PWM_CTL_GLOBALSYNC2` writer - Update PWM Generator 2"]
pub type PWM_CTL_GLOBALSYNC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_CTL_GLOBALSYNC3` reader - Update PWM Generator 3"]
pub type PWM_CTL_GLOBALSYNC3_R = crate::BitReader;
#[doc = "Field `PWM_CTL_GLOBALSYNC3` writer - Update PWM Generator 3"]
pub type PWM_CTL_GLOBALSYNC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Update PWM Generator 0"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync0(&self) -> PWM_CTL_GLOBALSYNC0_R {
        PWM_CTL_GLOBALSYNC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update PWM Generator 1"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync1(&self) -> PWM_CTL_GLOBALSYNC1_R {
        PWM_CTL_GLOBALSYNC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update PWM Generator 2"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync2(&self) -> PWM_CTL_GLOBALSYNC2_R {
        PWM_CTL_GLOBALSYNC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update PWM Generator 3"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync3(&self) -> PWM_CTL_GLOBALSYNC3_R {
        PWM_CTL_GLOBALSYNC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update PWM Generator 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ctl_globalsync0(&mut self) -> PWM_CTL_GLOBALSYNC0_W<CTL_SPEC, 0> {
        PWM_CTL_GLOBALSYNC0_W::new(self)
    }
    #[doc = "Bit 1 - Update PWM Generator 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ctl_globalsync1(&mut self) -> PWM_CTL_GLOBALSYNC1_W<CTL_SPEC, 1> {
        PWM_CTL_GLOBALSYNC1_W::new(self)
    }
    #[doc = "Bit 2 - Update PWM Generator 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ctl_globalsync2(&mut self) -> PWM_CTL_GLOBALSYNC2_W<CTL_SPEC, 2> {
        PWM_CTL_GLOBALSYNC2_W::new(self)
    }
    #[doc = "Bit 3 - Update PWM Generator 3"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_ctl_globalsync3(&mut self) -> PWM_CTL_GLOBALSYNC3_W<CTL_SPEC, 3> {
        PWM_CTL_GLOBALSYNC3_W::new(self)
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
#[doc = "PWM Master Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
