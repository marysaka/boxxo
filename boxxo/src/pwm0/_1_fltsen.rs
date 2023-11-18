#[doc = "Register `_1_FLTSEN` reader"]
pub type R = crate::R<_1_FLTSEN_SPEC>;
#[doc = "Register `_1_FLTSEN` writer"]
pub type W = crate::W<_1_FLTSEN_SPEC>;
#[doc = "Field `PWM_1_FLTSEN_FAULT0` reader - Fault0 Sense"]
pub type PWM_1_FLTSEN_FAULT0_R = crate::BitReader;
#[doc = "Field `PWM_1_FLTSEN_FAULT0` writer - Fault0 Sense"]
pub type PWM_1_FLTSEN_FAULT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_1_FLTSEN_FAULT1` reader - Fault1 Sense"]
pub type PWM_1_FLTSEN_FAULT1_R = crate::BitReader;
#[doc = "Field `PWM_1_FLTSEN_FAULT1` writer - Fault1 Sense"]
pub type PWM_1_FLTSEN_FAULT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Fault0 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault0(&self) -> PWM_1_FLTSEN_FAULT0_R {
        PWM_1_FLTSEN_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault1 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault1(&self) -> PWM_1_FLTSEN_FAULT1_R {
        PWM_1_FLTSEN_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault0 Sense"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_1_fltsen_fault0(&mut self) -> PWM_1_FLTSEN_FAULT0_W<_1_FLTSEN_SPEC, 0> {
        PWM_1_FLTSEN_FAULT0_W::new(self)
    }
    #[doc = "Bit 1 - Fault1 Sense"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_1_fltsen_fault1(&mut self) -> PWM_1_FLTSEN_FAULT1_W<_1_FLTSEN_SPEC, 1> {
        PWM_1_FLTSEN_FAULT1_W::new(self)
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
#[doc = "PWM1 Fault Pin Logic Sense\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_fltsen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_fltsen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1_FLTSEN_SPEC;
impl crate::RegisterSpec for _1_FLTSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1_fltsen::R`](R) reader structure"]
impl crate::Readable for _1_FLTSEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_1_fltsen::W`](W) writer structure"]
impl crate::Writable for _1_FLTSEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _1_FLTSEN to value 0"]
impl crate::Resettable for _1_FLTSEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
