#[doc = "Register `_3_FLTSRC0` reader"]
pub type R = crate::R<_3_FLTSRC0_SPEC>;
#[doc = "Register `_3_FLTSRC0` writer"]
pub type W = crate::W<_3_FLTSRC0_SPEC>;
#[doc = "Field `PWM_3_FLTSRC0_FAULT0` reader - Fault0"]
pub type PWM_3_FLTSRC0_FAULT0_R = crate::BitReader;
#[doc = "Field `PWM_3_FLTSRC0_FAULT0` writer - Fault0"]
pub type PWM_3_FLTSRC0_FAULT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_FLTSRC0_FAULT1` reader - Fault1"]
pub type PWM_3_FLTSRC0_FAULT1_R = crate::BitReader;
#[doc = "Field `PWM_3_FLTSRC0_FAULT1` writer - Fault1"]
pub type PWM_3_FLTSRC0_FAULT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Fault0"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault0(&self) -> PWM_3_FLTSRC0_FAULT0_R {
        PWM_3_FLTSRC0_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault1"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault1(&self) -> PWM_3_FLTSRC0_FAULT1_R {
        PWM_3_FLTSRC0_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault0"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_fltsrc0_fault0(&mut self) -> PWM_3_FLTSRC0_FAULT0_W<_3_FLTSRC0_SPEC, 0> {
        PWM_3_FLTSRC0_FAULT0_W::new(self)
    }
    #[doc = "Bit 1 - Fault1"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_fltsrc0_fault1(&mut self) -> PWM_3_FLTSRC0_FAULT1_W<_3_FLTSRC0_SPEC, 1> {
        PWM_3_FLTSRC0_FAULT1_W::new(self)
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
#[doc = "PWM3 Fault Source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_fltsrc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_fltsrc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_FLTSRC0_SPEC;
impl crate::RegisterSpec for _3_FLTSRC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_fltsrc0::R`](R) reader structure"]
impl crate::Readable for _3_FLTSRC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_3_fltsrc0::W`](W) writer structure"]
impl crate::Writable for _3_FLTSRC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _3_FLTSRC0 to value 0"]
impl crate::Resettable for _3_FLTSRC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
