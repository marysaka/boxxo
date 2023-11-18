#[doc = "Register `_1_CMPB` reader"]
pub type R = crate::R<_1_CMPB_SPEC>;
#[doc = "Register `_1_CMPB` writer"]
pub type W = crate::W<_1_CMPB_SPEC>;
#[doc = "Field `PWM_1_CMPB_COMPB` reader - Comparator B Value"]
pub type PWM_1_CMPB_COMPB_R = crate::FieldReader<u16>;
#[doc = "Field `PWM_1_CMPB_COMPB` writer - Comparator B Value"]
pub type PWM_1_CMPB_COMPB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Comparator B Value"]
    #[inline(always)]
    pub fn pwm_1_cmpb_compb(&self) -> PWM_1_CMPB_COMPB_R {
        PWM_1_CMPB_COMPB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator B Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_1_cmpb_compb(&mut self) -> PWM_1_CMPB_COMPB_W<_1_CMPB_SPEC, 0> {
        PWM_1_CMPB_COMPB_W::new(self)
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
#[doc = "PWM1 Compare B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_cmpb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1_cmpb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1_CMPB_SPEC;
impl crate::RegisterSpec for _1_CMPB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1_cmpb::R`](R) reader structure"]
impl crate::Readable for _1_CMPB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_1_cmpb::W`](W) writer structure"]
impl crate::Writable for _1_CMPB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _1_CMPB to value 0"]
impl crate::Resettable for _1_CMPB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
