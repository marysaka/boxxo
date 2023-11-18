#[doc = "Register `_2_RIS` reader"]
pub type R = crate::R<_2_RIS_SPEC>;
#[doc = "Register `_2_RIS` writer"]
pub type W = crate::W<_2_RIS_SPEC>;
#[doc = "Field `PWM_2_RIS_INTCNTZERO` reader - Counter=0 Interrupt Status"]
pub type PWM_2_RIS_INTCNTZERO_R = crate::BitReader;
#[doc = "Field `PWM_2_RIS_INTCNTZERO` writer - Counter=0 Interrupt Status"]
pub type PWM_2_RIS_INTCNTZERO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_2_RIS_INTCNTLOAD` reader - Counter=Load Interrupt Status"]
pub type PWM_2_RIS_INTCNTLOAD_R = crate::BitReader;
#[doc = "Field `PWM_2_RIS_INTCNTLOAD` writer - Counter=Load Interrupt Status"]
pub type PWM_2_RIS_INTCNTLOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_2_RIS_INTCMPAU` reader - Comparator A Up Interrupt Status"]
pub type PWM_2_RIS_INTCMPAU_R = crate::BitReader;
#[doc = "Field `PWM_2_RIS_INTCMPAU` writer - Comparator A Up Interrupt Status"]
pub type PWM_2_RIS_INTCMPAU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_2_RIS_INTCMPAD` reader - Comparator A Down Interrupt Status"]
pub type PWM_2_RIS_INTCMPAD_R = crate::BitReader;
#[doc = "Field `PWM_2_RIS_INTCMPAD` writer - Comparator A Down Interrupt Status"]
pub type PWM_2_RIS_INTCMPAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_2_RIS_INTCMPBU` reader - Comparator B Up Interrupt Status"]
pub type PWM_2_RIS_INTCMPBU_R = crate::BitReader;
#[doc = "Field `PWM_2_RIS_INTCMPBU` writer - Comparator B Up Interrupt Status"]
pub type PWM_2_RIS_INTCMPBU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_2_RIS_INTCMPBD` reader - Comparator B Down Interrupt Status"]
pub type PWM_2_RIS_INTCMPBD_R = crate::BitReader;
#[doc = "Field `PWM_2_RIS_INTCMPBD` writer - Comparator B Down Interrupt Status"]
pub type PWM_2_RIS_INTCMPBD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Counter=0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcntzero(&self) -> PWM_2_RIS_INTCNTZERO_R {
        PWM_2_RIS_INTCNTZERO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter=Load Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcntload(&self) -> PWM_2_RIS_INTCNTLOAD_R {
        PWM_2_RIS_INTCNTLOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpau(&self) -> PWM_2_RIS_INTCMPAU_R {
        PWM_2_RIS_INTCMPAU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpad(&self) -> PWM_2_RIS_INTCMPAD_R {
        PWM_2_RIS_INTCMPAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpbu(&self) -> PWM_2_RIS_INTCMPBU_R {
        PWM_2_RIS_INTCMPBU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpbd(&self) -> PWM_2_RIS_INTCMPBD_R {
        PWM_2_RIS_INTCMPBD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter=0 Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_2_ris_intcntzero(&mut self) -> PWM_2_RIS_INTCNTZERO_W<_2_RIS_SPEC, 0> {
        PWM_2_RIS_INTCNTZERO_W::new(self)
    }
    #[doc = "Bit 1 - Counter=Load Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_2_ris_intcntload(&mut self) -> PWM_2_RIS_INTCNTLOAD_W<_2_RIS_SPEC, 1> {
        PWM_2_RIS_INTCNTLOAD_W::new(self)
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_2_ris_intcmpau(&mut self) -> PWM_2_RIS_INTCMPAU_W<_2_RIS_SPEC, 2> {
        PWM_2_RIS_INTCMPAU_W::new(self)
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_2_ris_intcmpad(&mut self) -> PWM_2_RIS_INTCMPAD_W<_2_RIS_SPEC, 3> {
        PWM_2_RIS_INTCMPAD_W::new(self)
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_2_ris_intcmpbu(&mut self) -> PWM_2_RIS_INTCMPBU_W<_2_RIS_SPEC, 4> {
        PWM_2_RIS_INTCMPBU_W::new(self)
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_2_ris_intcmpbd(&mut self) -> PWM_2_RIS_INTCMPBD_W<_2_RIS_SPEC, 5> {
        PWM_2_RIS_INTCMPBD_W::new(self)
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
#[doc = "PWM2 Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _2_RIS_SPEC;
impl crate::RegisterSpec for _2_RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_2_ris::R`](R) reader structure"]
impl crate::Readable for _2_RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_2_ris::W`](W) writer structure"]
impl crate::Writable for _2_RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _2_RIS to value 0"]
impl crate::Resettable for _2_RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
