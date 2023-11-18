#[doc = "Register `SSDC1` reader"]
pub type R = crate::R<SSDC1_SPEC>;
#[doc = "Register `SSDC1` writer"]
pub type W = crate::W<SSDC1_SPEC>;
#[doc = "Field `ADC_SSDC1_S0DCSEL` reader - Sample 0 Digital Comparator Select"]
pub type ADC_SSDC1_S0DCSEL_R = crate::FieldReader;
#[doc = "Field `ADC_SSDC1_S0DCSEL` writer - Sample 0 Digital Comparator Select"]
pub type ADC_SSDC1_S0DCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSDC1_S1DCSEL` reader - Sample 1 Digital Comparator Select"]
pub type ADC_SSDC1_S1DCSEL_R = crate::FieldReader;
#[doc = "Field `ADC_SSDC1_S1DCSEL` writer - Sample 1 Digital Comparator Select"]
pub type ADC_SSDC1_S1DCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSDC1_S2DCSEL` reader - Sample 2 Digital Comparator Select"]
pub type ADC_SSDC1_S2DCSEL_R = crate::FieldReader;
#[doc = "Field `ADC_SSDC1_S2DCSEL` writer - Sample 2 Digital Comparator Select"]
pub type ADC_SSDC1_S2DCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSDC1_S3DCSEL` reader - Sample 3 Digital Comparator Select"]
pub type ADC_SSDC1_S3DCSEL_R = crate::FieldReader;
#[doc = "Field `ADC_SSDC1_S3DCSEL` writer - Sample 3 Digital Comparator Select"]
pub type ADC_SSDC1_S3DCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc1_s0dcsel(&self) -> ADC_SSDC1_S0DCSEL_R {
        ADC_SSDC1_S0DCSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc1_s1dcsel(&self) -> ADC_SSDC1_S1DCSEL_R {
        ADC_SSDC1_S1DCSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc1_s2dcsel(&self) -> ADC_SSDC1_S2DCSEL_R {
        ADC_SSDC1_S2DCSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc1_s3dcsel(&self) -> ADC_SSDC1_S3DCSEL_R {
        ADC_SSDC1_S3DCSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssdc1_s0dcsel(&mut self) -> ADC_SSDC1_S0DCSEL_W<SSDC1_SPEC, 0> {
        ADC_SSDC1_S0DCSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssdc1_s1dcsel(&mut self) -> ADC_SSDC1_S1DCSEL_W<SSDC1_SPEC, 4> {
        ADC_SSDC1_S1DCSEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssdc1_s2dcsel(&mut self) -> ADC_SSDC1_S2DCSEL_W<SSDC1_SPEC, 8> {
        ADC_SSDC1_S2DCSEL_W::new(self)
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssdc1_s3dcsel(&mut self) -> ADC_SSDC1_S3DCSEL_W<SSDC1_SPEC, 12> {
        ADC_SSDC1_S3DCSEL_W::new(self)
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
#[doc = "ADC Sample Sequence 1 Digital Comparator Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssdc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssdc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSDC1_SPEC;
impl crate::RegisterSpec for SSDC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssdc1::R`](R) reader structure"]
impl crate::Readable for SSDC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssdc1::W`](W) writer structure"]
impl crate::Writable for SSDC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSDC1 to value 0"]
impl crate::Resettable for SSDC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
