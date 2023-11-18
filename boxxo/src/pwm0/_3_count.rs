#[doc = "Register `_3_COUNT` reader"]
pub type R = crate::R<_3_COUNT_SPEC>;
#[doc = "Register `_3_COUNT` writer"]
pub type W = crate::W<_3_COUNT_SPEC>;
#[doc = "Field `PWM_3_COUNT_COUNT` reader - Counter Value"]
pub type PWM_3_COUNT_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `PWM_3_COUNT_COUNT` writer - Counter Value"]
pub type PWM_3_COUNT_COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    pub fn pwm_3_count_count(&self) -> PWM_3_COUNT_COUNT_R {
        PWM_3_COUNT_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_count_count(&mut self) -> PWM_3_COUNT_COUNT_W<_3_COUNT_SPEC, 0> {
        PWM_3_COUNT_COUNT_W::new(self)
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
#[doc = "PWM3 Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_COUNT_SPEC;
impl crate::RegisterSpec for _3_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_count::R`](R) reader structure"]
impl crate::Readable for _3_COUNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_3_count::W`](W) writer structure"]
impl crate::Writable for _3_COUNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _3_COUNT to value 0"]
impl crate::Resettable for _3_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}