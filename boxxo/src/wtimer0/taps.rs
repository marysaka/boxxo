#[doc = "Register `TAPS` reader"]
pub type R = crate::R<TAPS_SPEC>;
#[doc = "Register `TAPS` writer"]
pub type W = crate::W<TAPS_SPEC>;
#[doc = "Field `TIMER_TAPS_PSS` reader - GPTM Timer A Prescaler Snapshot"]
pub type TIMER_TAPS_PSS_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_TAPS_PSS` writer - GPTM Timer A Prescaler Snapshot"]
pub type TIMER_TAPS_PSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Snapshot"]
    #[inline(always)]
    pub fn timer_taps_pss(&self) -> TIMER_TAPS_PSS_R {
        TIMER_TAPS_PSS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Snapshot"]
    #[inline(always)]
    #[must_use]
    pub fn timer_taps_pss(&mut self) -> TIMER_TAPS_PSS_W<TAPS_SPEC, 0> {
        TIMER_TAPS_PSS_W::new(self)
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
#[doc = "GPTM Timer A Prescale Snapshot\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAPS_SPEC;
impl crate::RegisterSpec for TAPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`taps::R`](R) reader structure"]
impl crate::Readable for TAPS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`taps::W`](W) writer structure"]
impl crate::Writable for TAPS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAPS to value 0"]
impl crate::Resettable for TAPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
