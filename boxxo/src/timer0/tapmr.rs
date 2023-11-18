#[doc = "Register `TAPMR` reader"]
pub type R = crate::R<TAPMR_SPEC>;
#[doc = "Register `TAPMR` writer"]
pub type W = crate::W<TAPMR_SPEC>;
#[doc = "Field `TIMER_TAPMR_TAPSMR` reader - GPTM TimerA Prescale Match"]
pub type TIMER_TAPMR_TAPSMR_R = crate::FieldReader;
#[doc = "Field `TIMER_TAPMR_TAPSMR` writer - GPTM TimerA Prescale Match"]
pub type TIMER_TAPMR_TAPSMR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TIMER_TAPMR_TAPSMRH` reader - GPTM Timer A Prescale Match High Byte"]
pub type TIMER_TAPMR_TAPSMRH_R = crate::FieldReader;
#[doc = "Field `TIMER_TAPMR_TAPSMRH` writer - GPTM Timer A Prescale Match High Byte"]
pub type TIMER_TAPMR_TAPSMRH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPTM TimerA Prescale Match"]
    #[inline(always)]
    pub fn timer_tapmr_tapsmr(&self) -> TIMER_TAPMR_TAPSMR_R {
        TIMER_TAPMR_TAPSMR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPTM Timer A Prescale Match High Byte"]
    #[inline(always)]
    pub fn timer_tapmr_tapsmrh(&self) -> TIMER_TAPMR_TAPSMRH_R {
        TIMER_TAPMR_TAPSMRH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM TimerA Prescale Match"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tapmr_tapsmr(&mut self) -> TIMER_TAPMR_TAPSMR_W<TAPMR_SPEC, 0> {
        TIMER_TAPMR_TAPSMR_W::new(self)
    }
    #[doc = "Bits 8:15 - GPTM Timer A Prescale Match High Byte"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tapmr_tapsmrh(&mut self) -> TIMER_TAPMR_TAPSMRH_W<TAPMR_SPEC, 8> {
        TIMER_TAPMR_TAPSMRH_W::new(self)
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
#[doc = "GPTM TimerA Prescale Match\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAPMR_SPEC;
impl crate::RegisterSpec for TAPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tapmr::R`](R) reader structure"]
impl crate::Readable for TAPMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tapmr::W`](W) writer structure"]
impl crate::Writable for TAPMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAPMR to value 0"]
impl crate::Resettable for TAPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
