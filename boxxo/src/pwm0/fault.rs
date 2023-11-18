#[doc = "Register `FAULT` reader"]
pub type R = crate::R<FAULT_SPEC>;
#[doc = "Register `FAULT` writer"]
pub type W = crate::W<FAULT_SPEC>;
#[doc = "Field `PWM_FAULT_FAULT0` reader - PWM0 Fault"]
pub type PWM_FAULT_FAULT0_R = crate::BitReader;
#[doc = "Field `PWM_FAULT_FAULT0` writer - PWM0 Fault"]
pub type PWM_FAULT_FAULT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULT_FAULT1` reader - PWM1 Fault"]
pub type PWM_FAULT_FAULT1_R = crate::BitReader;
#[doc = "Field `PWM_FAULT_FAULT1` writer - PWM1 Fault"]
pub type PWM_FAULT_FAULT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULT_FAULT2` reader - PWM2 Fault"]
pub type PWM_FAULT_FAULT2_R = crate::BitReader;
#[doc = "Field `PWM_FAULT_FAULT2` writer - PWM2 Fault"]
pub type PWM_FAULT_FAULT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULT_FAULT3` reader - PWM3 Fault"]
pub type PWM_FAULT_FAULT3_R = crate::BitReader;
#[doc = "Field `PWM_FAULT_FAULT3` writer - PWM3 Fault"]
pub type PWM_FAULT_FAULT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULT_FAULT4` reader - PWM4 Fault"]
pub type PWM_FAULT_FAULT4_R = crate::BitReader;
#[doc = "Field `PWM_FAULT_FAULT4` writer - PWM4 Fault"]
pub type PWM_FAULT_FAULT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULT_FAULT5` reader - PWM5 Fault"]
pub type PWM_FAULT_FAULT5_R = crate::BitReader;
#[doc = "Field `PWM_FAULT_FAULT5` writer - PWM5 Fault"]
pub type PWM_FAULT_FAULT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULT_FAULT6` reader - PWM6 Fault"]
pub type PWM_FAULT_FAULT6_R = crate::BitReader;
#[doc = "Field `PWM_FAULT_FAULT6` writer - PWM6 Fault"]
pub type PWM_FAULT_FAULT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_FAULT_FAULT7` reader - PWM7 Fault"]
pub type PWM_FAULT_FAULT7_R = crate::BitReader;
#[doc = "Field `PWM_FAULT_FAULT7` writer - PWM7 Fault"]
pub type PWM_FAULT_FAULT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM0 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault0(&self) -> PWM_FAULT_FAULT0_R {
        PWM_FAULT_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault1(&self) -> PWM_FAULT_FAULT1_R {
        PWM_FAULT_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault2(&self) -> PWM_FAULT_FAULT2_R {
        PWM_FAULT_FAULT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault3(&self) -> PWM_FAULT_FAULT3_R {
        PWM_FAULT_FAULT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault4(&self) -> PWM_FAULT_FAULT4_R {
        PWM_FAULT_FAULT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM5 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault5(&self) -> PWM_FAULT_FAULT5_R {
        PWM_FAULT_FAULT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault6(&self) -> PWM_FAULT_FAULT6_R {
        PWM_FAULT_FAULT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM7 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault7(&self) -> PWM_FAULT_FAULT7_R {
        PWM_FAULT_FAULT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_fault_fault0(&mut self) -> PWM_FAULT_FAULT0_W<FAULT_SPEC, 0> {
        PWM_FAULT_FAULT0_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_fault_fault1(&mut self) -> PWM_FAULT_FAULT1_W<FAULT_SPEC, 1> {
        PWM_FAULT_FAULT1_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_fault_fault2(&mut self) -> PWM_FAULT_FAULT2_W<FAULT_SPEC, 2> {
        PWM_FAULT_FAULT2_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_fault_fault3(&mut self) -> PWM_FAULT_FAULT3_W<FAULT_SPEC, 3> {
        PWM_FAULT_FAULT3_W::new(self)
    }
    #[doc = "Bit 4 - PWM4 Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_fault_fault4(&mut self) -> PWM_FAULT_FAULT4_W<FAULT_SPEC, 4> {
        PWM_FAULT_FAULT4_W::new(self)
    }
    #[doc = "Bit 5 - PWM5 Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_fault_fault5(&mut self) -> PWM_FAULT_FAULT5_W<FAULT_SPEC, 5> {
        PWM_FAULT_FAULT5_W::new(self)
    }
    #[doc = "Bit 6 - PWM6 Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_fault_fault6(&mut self) -> PWM_FAULT_FAULT6_W<FAULT_SPEC, 6> {
        PWM_FAULT_FAULT6_W::new(self)
    }
    #[doc = "Bit 7 - PWM7 Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_fault_fault7(&mut self) -> PWM_FAULT_FAULT7_W<FAULT_SPEC, 7> {
        PWM_FAULT_FAULT7_W::new(self)
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
#[doc = "PWM Output Fault\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FAULT_SPEC;
impl crate::RegisterSpec for FAULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault::R`](R) reader structure"]
impl crate::Readable for FAULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fault::W`](W) writer structure"]
impl crate::Writable for FAULT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAULT to value 0"]
impl crate::Resettable for FAULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
