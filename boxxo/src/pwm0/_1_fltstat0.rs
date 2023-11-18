#[doc = "Register `_1_FLTSTAT0` reader"]
pub type R = crate::R<_1_FLTSTAT0_SPEC>;
#[doc = "Field `PWM_1_FLTSTAT0_FAULT0` reader - Fault Input 0"]
pub type PWM_1_FLTSTAT0_FAULT0_R = crate::BitReader;
#[doc = "Field `PWM_1_FLTSTAT0_FAULT1` reader - Fault Input 1"]
pub type PWM_1_FLTSTAT0_FAULT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Fault Input 0"]
    #[inline(always)]
    pub fn pwm_1_fltstat0_fault0(&self) -> PWM_1_FLTSTAT0_FAULT0_R {
        PWM_1_FLTSTAT0_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1"]
    #[inline(always)]
    pub fn pwm_1_fltstat0_fault1(&self) -> PWM_1_FLTSTAT0_FAULT1_R {
        PWM_1_FLTSTAT0_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PWM1 Fault Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_fltstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1_FLTSTAT0_SPEC;
impl crate::RegisterSpec for _1_FLTSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1_fltstat0::R`](R) reader structure"]
impl crate::Readable for _1_FLTSTAT0_SPEC {}
#[doc = "`reset()` method sets _1_FLTSTAT0 to value 0"]
impl crate::Resettable for _1_FLTSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
