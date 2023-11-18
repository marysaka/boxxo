#[doc = "Register `PP` reader"]
pub type R = crate::R<PP_SPEC>;
#[doc = "Register `PP` writer"]
pub type W = crate::W<PP_SPEC>;
#[doc = "Field `PWM_PP_GCNT` reader - Generators"]
pub type PWM_PP_GCNT_R = crate::FieldReader;
#[doc = "Field `PWM_PP_GCNT` writer - Generators"]
pub type PWM_PP_GCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PWM_PP_FCNT` reader - Fault Inputs"]
pub type PWM_PP_FCNT_R = crate::FieldReader;
#[doc = "Field `PWM_PP_FCNT` writer - Fault Inputs"]
pub type PWM_PP_FCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PWM_PP_ESYNC` reader - Extended Synchronization"]
pub type PWM_PP_ESYNC_R = crate::BitReader;
#[doc = "Field `PWM_PP_ESYNC` writer - Extended Synchronization"]
pub type PWM_PP_ESYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_PP_EFAULT` reader - Extended Fault"]
pub type PWM_PP_EFAULT_R = crate::BitReader;
#[doc = "Field `PWM_PP_EFAULT` writer - Extended Fault"]
pub type PWM_PP_EFAULT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_PP_ONE` reader - One-Shot Mode"]
pub type PWM_PP_ONE_R = crate::BitReader;
#[doc = "Field `PWM_PP_ONE` writer - One-Shot Mode"]
pub type PWM_PP_ONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Generators"]
    #[inline(always)]
    pub fn pwm_pp_gcnt(&self) -> PWM_PP_GCNT_R {
        PWM_PP_GCNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Fault Inputs"]
    #[inline(always)]
    pub fn pwm_pp_fcnt(&self) -> PWM_PP_FCNT_R {
        PWM_PP_FCNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Extended Synchronization"]
    #[inline(always)]
    pub fn pwm_pp_esync(&self) -> PWM_PP_ESYNC_R {
        PWM_PP_ESYNC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Extended Fault"]
    #[inline(always)]
    pub fn pwm_pp_efault(&self) -> PWM_PP_EFAULT_R {
        PWM_PP_EFAULT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - One-Shot Mode"]
    #[inline(always)]
    pub fn pwm_pp_one(&self) -> PWM_PP_ONE_R {
        PWM_PP_ONE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generators"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_pp_gcnt(&mut self) -> PWM_PP_GCNT_W<PP_SPEC, 0> {
        PWM_PP_GCNT_W::new(self)
    }
    #[doc = "Bits 4:7 - Fault Inputs"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_pp_fcnt(&mut self) -> PWM_PP_FCNT_W<PP_SPEC, 4> {
        PWM_PP_FCNT_W::new(self)
    }
    #[doc = "Bit 8 - Extended Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_pp_esync(&mut self) -> PWM_PP_ESYNC_W<PP_SPEC, 8> {
        PWM_PP_ESYNC_W::new(self)
    }
    #[doc = "Bit 9 - Extended Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_pp_efault(&mut self) -> PWM_PP_EFAULT_W<PP_SPEC, 9> {
        PWM_PP_EFAULT_W::new(self)
    }
    #[doc = "Bit 10 - One-Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_pp_one(&mut self) -> PWM_PP_ONE_W<PP_SPEC, 10> {
        PWM_PP_ONE_W::new(self)
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
#[doc = "PWM Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp::R`](R) reader structure"]
impl crate::Readable for PP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pp::W`](W) writer structure"]
impl crate::Writable for PP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
