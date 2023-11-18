#[doc = "Register `SSMUX3` reader"]
pub type R = crate::R<SSMUX3_SPEC>;
#[doc = "Register `SSMUX3` writer"]
pub type W = crate::W<SSMUX3_SPEC>;
#[doc = "Field `ADC_SSMUX3_MUX0` reader - 1st Sample Input Select"]
pub type ADC_SSMUX3_MUX0_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX3_MUX0` writer - 1st Sample Input Select"]
pub type ADC_SSMUX3_MUX0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux3_mux0(&self) -> ADC_SSMUX3_MUX0_R {
        ADC_SSMUX3_MUX0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux3_mux0(&mut self) -> ADC_SSMUX3_MUX0_W<SSMUX3_SPEC, 0> {
        ADC_SSMUX3_MUX0_W::new(self)
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
#[doc = "ADC Sample Sequence Input Multiplexer Select 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssmux3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssmux3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSMUX3_SPEC;
impl crate::RegisterSpec for SSMUX3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssmux3::R`](R) reader structure"]
impl crate::Readable for SSMUX3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssmux3::W`](W) writer structure"]
impl crate::Writable for SSMUX3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSMUX3 to value 0"]
impl crate::Resettable for SSMUX3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
