#[doc = "Register `TBPR` reader"]
pub type R = crate::R<TBPR_SPEC>;
#[doc = "Register `TBPR` writer"]
pub type W = crate::W<TBPR_SPEC>;
#[doc = "Field `TIMER_TBPR_TBPSR` reader - GPTM Timer B Prescale"]
pub type TIMER_TBPR_TBPSR_R = crate::FieldReader;
#[doc = "Field `TIMER_TBPR_TBPSR` writer - GPTM Timer B Prescale"]
pub type TIMER_TBPR_TBPSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TIMER_TBPR_TBPSRH` reader - GPTM Timer B Prescale High Byte"]
pub type TIMER_TBPR_TBPSRH_R = crate::FieldReader;
#[doc = "Field `TIMER_TBPR_TBPSRH` writer - GPTM Timer B Prescale High Byte"]
pub type TIMER_TBPR_TBPSRH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer B Prescale"]
    #[inline(always)]
    pub fn timer_tbpr_tbpsr(&self) -> TIMER_TBPR_TBPSR_R {
        TIMER_TBPR_TBPSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPTM Timer B Prescale High Byte"]
    #[inline(always)]
    pub fn timer_tbpr_tbpsrh(&self) -> TIMER_TBPR_TBPSRH_R {
        TIMER_TBPR_TBPSRH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer B Prescale"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbpr_tbpsr(&mut self) -> TIMER_TBPR_TBPSR_W<TBPR_SPEC, 0> {
        TIMER_TBPR_TBPSR_W::new(self)
    }
    #[doc = "Bits 8:15 - GPTM Timer B Prescale High Byte"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbpr_tbpsrh(&mut self) -> TIMER_TBPR_TBPSRH_W<TBPR_SPEC, 8> {
        TIMER_TBPR_TBPSRH_W::new(self)
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
#[doc = "GPTM Timer B Prescale\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBPR_SPEC;
impl crate::RegisterSpec for TBPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpr::R`](R) reader structure"]
impl crate::Readable for TBPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbpr::W`](W) writer structure"]
impl crate::Writable for TBPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBPR to value 0"]
impl crate::Resettable for TBPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
