#[doc = "Register `TAPV` reader"]
pub type R = crate::R<TAPV_SPEC>;
#[doc = "Register `TAPV` writer"]
pub type W = crate::W<TAPV_SPEC>;
#[doc = "Field `TIMER_TAPV_PSV` reader - GPTM Timer A Prescaler Value"]
pub type TIMER_TAPV_PSV_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_TAPV_PSV` writer - GPTM Timer A Prescaler Value"]
pub type TIMER_TAPV_PSV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Value"]
    #[inline(always)]
    pub fn timer_tapv_psv(&self) -> TIMER_TAPV_PSV_R {
        TIMER_TAPV_PSV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tapv_psv(&mut self) -> TIMER_TAPV_PSV_W<TAPV_SPEC, 0> {
        TIMER_TAPV_PSV_W::new(self)
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
#[doc = "GPTM Timer A Prescale Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAPV_SPEC;
impl crate::RegisterSpec for TAPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tapv::R`](R) reader structure"]
impl crate::Readable for TAPV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tapv::W`](W) writer structure"]
impl crate::Writable for TAPV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAPV to value 0"]
impl crate::Resettable for TAPV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
