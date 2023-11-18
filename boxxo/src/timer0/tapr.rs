#[doc = "Register `TAPR` reader"]
pub type R = crate::R<TAPR_SPEC>;
#[doc = "Register `TAPR` writer"]
pub type W = crate::W<TAPR_SPEC>;
#[doc = "Field `TIMER_TAPR_TAPSR` reader - GPTM Timer A Prescale"]
pub type TIMER_TAPR_TAPSR_R = crate::FieldReader;
#[doc = "Field `TIMER_TAPR_TAPSR` writer - GPTM Timer A Prescale"]
pub type TIMER_TAPR_TAPSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TIMER_TAPR_TAPSRH` reader - GPTM Timer A Prescale High Byte"]
pub type TIMER_TAPR_TAPSRH_R = crate::FieldReader;
#[doc = "Field `TIMER_TAPR_TAPSRH` writer - GPTM Timer A Prescale High Byte"]
pub type TIMER_TAPR_TAPSRH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer A Prescale"]
    #[inline(always)]
    pub fn timer_tapr_tapsr(&self) -> TIMER_TAPR_TAPSR_R {
        TIMER_TAPR_TAPSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPTM Timer A Prescale High Byte"]
    #[inline(always)]
    pub fn timer_tapr_tapsrh(&self) -> TIMER_TAPR_TAPSRH_R {
        TIMER_TAPR_TAPSRH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer A Prescale"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tapr_tapsr(&mut self) -> TIMER_TAPR_TAPSR_W<TAPR_SPEC, 0> {
        TIMER_TAPR_TAPSR_W::new(self)
    }
    #[doc = "Bits 8:15 - GPTM Timer A Prescale High Byte"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tapr_tapsrh(&mut self) -> TIMER_TAPR_TAPSRH_W<TAPR_SPEC, 8> {
        TIMER_TAPR_TAPSRH_W::new(self)
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
#[doc = "GPTM Timer A Prescale\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAPR_SPEC;
impl crate::RegisterSpec for TAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tapr::R`](R) reader structure"]
impl crate::Readable for TAPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tapr::W`](W) writer structure"]
impl crate::Writable for TAPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAPR to value 0"]
impl crate::Resettable for TAPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
