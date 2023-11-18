#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `PWM_STATUS_FAULT0` reader - Generator 0 Fault Status"]
pub type PWM_STATUS_FAULT0_R = crate::BitReader;
#[doc = "Field `PWM_STATUS_FAULT0` writer - Generator 0 Fault Status"]
pub type PWM_STATUS_FAULT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_STATUS_FAULT1` reader - Generator 1 Fault Status"]
pub type PWM_STATUS_FAULT1_R = crate::BitReader;
#[doc = "Field `PWM_STATUS_FAULT1` writer - Generator 1 Fault Status"]
pub type PWM_STATUS_FAULT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Generator 0 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault0(&self) -> PWM_STATUS_FAULT0_R {
        PWM_STATUS_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generator 1 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault1(&self) -> PWM_STATUS_FAULT1_R {
        PWM_STATUS_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generator 0 Fault Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_status_fault0(&mut self) -> PWM_STATUS_FAULT0_W<STATUS_SPEC, 0> {
        PWM_STATUS_FAULT0_W::new(self)
    }
    #[doc = "Bit 1 - Generator 1 Fault Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_status_fault1(&mut self) -> PWM_STATUS_FAULT1_W<STATUS_SPEC, 1> {
        PWM_STATUS_FAULT1_W::new(self)
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
#[doc = "PWM Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
