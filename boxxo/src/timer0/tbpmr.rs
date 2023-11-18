#[doc = "Register `TBPMR` reader"]
pub type R = crate::R<TBPMR_SPEC>;
#[doc = "Register `TBPMR` writer"]
pub type W = crate::W<TBPMR_SPEC>;
#[doc = "Field `TIMER_TBPMR_TBPSMR` reader - GPTM TimerB Prescale Match"]
pub type TIMER_TBPMR_TBPSMR_R = crate::FieldReader;
#[doc = "Field `TIMER_TBPMR_TBPSMR` writer - GPTM TimerB Prescale Match"]
pub type TIMER_TBPMR_TBPSMR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TIMER_TBPMR_TBPSMRH` reader - GPTM Timer B Prescale Match High Byte"]
pub type TIMER_TBPMR_TBPSMRH_R = crate::FieldReader;
#[doc = "Field `TIMER_TBPMR_TBPSMRH` writer - GPTM Timer B Prescale Match High Byte"]
pub type TIMER_TBPMR_TBPSMRH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match"]
    #[inline(always)]
    pub fn timer_tbpmr_tbpsmr(&self) -> TIMER_TBPMR_TBPSMR_R {
        TIMER_TBPMR_TBPSMR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPTM Timer B Prescale Match High Byte"]
    #[inline(always)]
    pub fn timer_tbpmr_tbpsmrh(&self) -> TIMER_TBPMR_TBPSMRH_R {
        TIMER_TBPMR_TBPSMRH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbpmr_tbpsmr(&mut self) -> TIMER_TBPMR_TBPSMR_W<TBPMR_SPEC, 0> {
        TIMER_TBPMR_TBPSMR_W::new(self)
    }
    #[doc = "Bits 8:15 - GPTM Timer B Prescale Match High Byte"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbpmr_tbpsmrh(&mut self) -> TIMER_TBPMR_TBPSMRH_W<TBPMR_SPEC, 8> {
        TIMER_TBPMR_TBPSMRH_W::new(self)
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
#[doc = "GPTM TimerB Prescale Match\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBPMR_SPEC;
impl crate::RegisterSpec for TBPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpmr::R`](R) reader structure"]
impl crate::Readable for TBPMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbpmr::W`](W) writer structure"]
impl crate::Writable for TBPMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBPMR to value 0"]
impl crate::Resettable for TBPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
