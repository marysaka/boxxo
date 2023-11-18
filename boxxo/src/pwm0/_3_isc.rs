#[doc = "Register `_3_ISC` reader"]
pub type R = crate::R<_3_ISC_SPEC>;
#[doc = "Register `_3_ISC` writer"]
pub type W = crate::W<_3_ISC_SPEC>;
#[doc = "Field `PWM_3_ISC_INTCNTZERO` reader - Counter=0 Interrupt"]
pub type PWM_3_ISC_INTCNTZERO_R = crate::BitReader;
#[doc = "Field `PWM_3_ISC_INTCNTZERO` writer - Counter=0 Interrupt"]
pub type PWM_3_ISC_INTCNTZERO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_ISC_INTCNTLOAD` reader - Counter=Load Interrupt"]
pub type PWM_3_ISC_INTCNTLOAD_R = crate::BitReader;
#[doc = "Field `PWM_3_ISC_INTCNTLOAD` writer - Counter=Load Interrupt"]
pub type PWM_3_ISC_INTCNTLOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_ISC_INTCMPAU` reader - Comparator A Up Interrupt"]
pub type PWM_3_ISC_INTCMPAU_R = crate::BitReader;
#[doc = "Field `PWM_3_ISC_INTCMPAU` writer - Comparator A Up Interrupt"]
pub type PWM_3_ISC_INTCMPAU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_ISC_INTCMPAD` reader - Comparator A Down Interrupt"]
pub type PWM_3_ISC_INTCMPAD_R = crate::BitReader;
#[doc = "Field `PWM_3_ISC_INTCMPAD` writer - Comparator A Down Interrupt"]
pub type PWM_3_ISC_INTCMPAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_ISC_INTCMPBU` reader - Comparator B Up Interrupt"]
pub type PWM_3_ISC_INTCMPBU_R = crate::BitReader;
#[doc = "Field `PWM_3_ISC_INTCMPBU` writer - Comparator B Up Interrupt"]
pub type PWM_3_ISC_INTCMPBU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_ISC_INTCMPBD` reader - Comparator B Down Interrupt"]
pub type PWM_3_ISC_INTCMPBD_R = crate::BitReader;
#[doc = "Field `PWM_3_ISC_INTCMPBD` writer - Comparator B Down Interrupt"]
pub type PWM_3_ISC_INTCMPBD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Counter=0 Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcntzero(&self) -> PWM_3_ISC_INTCNTZERO_R {
        PWM_3_ISC_INTCNTZERO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter=Load Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcntload(&self) -> PWM_3_ISC_INTCNTLOAD_R {
        PWM_3_ISC_INTCNTLOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpau(&self) -> PWM_3_ISC_INTCMPAU_R {
        PWM_3_ISC_INTCMPAU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpad(&self) -> PWM_3_ISC_INTCMPAD_R {
        PWM_3_ISC_INTCMPAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpbu(&self) -> PWM_3_ISC_INTCMPBU_R {
        PWM_3_ISC_INTCMPBU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpbd(&self) -> PWM_3_ISC_INTCMPBD_R {
        PWM_3_ISC_INTCMPBD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter=0 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_isc_intcntzero(&mut self) -> PWM_3_ISC_INTCNTZERO_W<_3_ISC_SPEC, 0> {
        PWM_3_ISC_INTCNTZERO_W::new(self)
    }
    #[doc = "Bit 1 - Counter=Load Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_isc_intcntload(&mut self) -> PWM_3_ISC_INTCNTLOAD_W<_3_ISC_SPEC, 1> {
        PWM_3_ISC_INTCNTLOAD_W::new(self)
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_isc_intcmpau(&mut self) -> PWM_3_ISC_INTCMPAU_W<_3_ISC_SPEC, 2> {
        PWM_3_ISC_INTCMPAU_W::new(self)
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_isc_intcmpad(&mut self) -> PWM_3_ISC_INTCMPAD_W<_3_ISC_SPEC, 3> {
        PWM_3_ISC_INTCMPAD_W::new(self)
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_isc_intcmpbu(&mut self) -> PWM_3_ISC_INTCMPBU_W<_3_ISC_SPEC, 4> {
        PWM_3_ISC_INTCMPBU_W::new(self)
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_isc_intcmpbd(&mut self) -> PWM_3_ISC_INTCMPBD_W<_3_ISC_SPEC, 5> {
        PWM_3_ISC_INTCMPBD_W::new(self)
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
#[doc = "PWM3 Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_isc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_isc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_ISC_SPEC;
impl crate::RegisterSpec for _3_ISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_isc::R`](R) reader structure"]
impl crate::Readable for _3_ISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_3_isc::W`](W) writer structure"]
impl crate::Writable for _3_ISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _3_ISC to value 0"]
impl crate::Resettable for _3_ISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
