#[doc = "Register `SSDC3` reader"]
pub type R = crate::R<SSDC3_SPEC>;
#[doc = "Register `SSDC3` writer"]
pub type W = crate::W<SSDC3_SPEC>;
#[doc = "Field `ADC_SSDC3_S0DCSEL` reader - Sample 0 Digital Comparator Select"]
pub type ADC_SSDC3_S0DCSEL_R = crate::FieldReader;
#[doc = "Field `ADC_SSDC3_S0DCSEL` writer - Sample 0 Digital Comparator Select"]
pub type ADC_SSDC3_S0DCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc3_s0dcsel(&self) -> ADC_SSDC3_S0DCSEL_R {
        ADC_SSDC3_S0DCSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssdc3_s0dcsel(&mut self) -> ADC_SSDC3_S0DCSEL_W<SSDC3_SPEC, 0> {
        ADC_SSDC3_S0DCSEL_W::new(self)
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
#[doc = "ADC Sample Sequence 3 Digital Comparator Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssdc3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssdc3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSDC3_SPEC;
impl crate::RegisterSpec for SSDC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssdc3::R`](R) reader structure"]
impl crate::Readable for SSDC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssdc3::W`](W) writer structure"]
impl crate::Writable for SSDC3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSDC3 to value 0"]
impl crate::Resettable for SSDC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
