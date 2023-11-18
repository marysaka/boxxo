#[doc = "Register `TBPS` reader"]
pub type R = crate::R<TBPS_SPEC>;
#[doc = "Register `TBPS` writer"]
pub type W = crate::W<TBPS_SPEC>;
#[doc = "Field `TIMER_TBPS_PSS` reader - GPTM Timer A Prescaler Value"]
pub type TIMER_TBPS_PSS_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_TBPS_PSS` writer - GPTM Timer A Prescaler Value"]
pub type TIMER_TBPS_PSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Value"]
    #[inline(always)]
    pub fn timer_tbps_pss(&self) -> TIMER_TBPS_PSS_R {
        TIMER_TBPS_PSS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbps_pss(&mut self) -> TIMER_TBPS_PSS_W<TBPS_SPEC, 0> {
        TIMER_TBPS_PSS_W::new(self)
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
#[doc = "GPTM Timer B Prescale Snapshot\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBPS_SPEC;
impl crate::RegisterSpec for TBPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbps::R`](R) reader structure"]
impl crate::Readable for TBPS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbps::W`](W) writer structure"]
impl crate::Writable for TBPS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBPS to value 0"]
impl crate::Resettable for TBPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
