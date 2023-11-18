#[doc = "Register `_3_FLTSTAT0` reader"]
pub type R = crate::R<_3_FLTSTAT0_SPEC>;
#[doc = "Field `PWM_3_FLTSTAT0_FAULT0` reader - Fault Input 0"]
pub type PWM_3_FLTSTAT0_FAULT0_R = crate::BitReader;
#[doc = "Field `PWM_3_FLTSTAT0_FAULT1` reader - Fault Input 1"]
pub type PWM_3_FLTSTAT0_FAULT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Fault Input 0"]
    #[inline(always)]
    pub fn pwm_3_fltstat0_fault0(&self) -> PWM_3_FLTSTAT0_FAULT0_R {
        PWM_3_FLTSTAT0_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1"]
    #[inline(always)]
    pub fn pwm_3_fltstat0_fault1(&self) -> PWM_3_FLTSTAT0_FAULT1_R {
        PWM_3_FLTSTAT0_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PWM3 Fault Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_fltstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_FLTSTAT0_SPEC;
impl crate::RegisterSpec for _3_FLTSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_fltstat0::R`](R) reader structure"]
impl crate::Readable for _3_FLTSTAT0_SPEC {}
#[doc = "`reset()` method sets _3_FLTSTAT0 to value 0"]
impl crate::Resettable for _3_FLTSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
