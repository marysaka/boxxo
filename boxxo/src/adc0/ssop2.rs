#[doc = "Register `SSOP2` reader"]
pub type R = crate::R<SSOP2_SPEC>;
#[doc = "Register `SSOP2` writer"]
pub type W = crate::W<SSOP2_SPEC>;
#[doc = "Field `ADC_SSOP2_S0DCOP` reader - Sample 0 Digital Comparator Operation"]
pub type ADC_SSOP2_S0DCOP_R = crate::BitReader;
#[doc = "Field `ADC_SSOP2_S0DCOP` writer - Sample 0 Digital Comparator Operation"]
pub type ADC_SSOP2_S0DCOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSOP2_S1DCOP` reader - Sample 1 Digital Comparator Operation"]
pub type ADC_SSOP2_S1DCOP_R = crate::BitReader;
#[doc = "Field `ADC_SSOP2_S1DCOP` writer - Sample 1 Digital Comparator Operation"]
pub type ADC_SSOP2_S1DCOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSOP2_S2DCOP` reader - Sample 2 Digital Comparator Operation"]
pub type ADC_SSOP2_S2DCOP_R = crate::BitReader;
#[doc = "Field `ADC_SSOP2_S2DCOP` writer - Sample 2 Digital Comparator Operation"]
pub type ADC_SSOP2_S2DCOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSOP2_S3DCOP` reader - Sample 3 Digital Comparator Operation"]
pub type ADC_SSOP2_S3DCOP_R = crate::BitReader;
#[doc = "Field `ADC_SSOP2_S3DCOP` writer - Sample 3 Digital Comparator Operation"]
pub type ADC_SSOP2_S3DCOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop2_s0dcop(&self) -> ADC_SSOP2_S0DCOP_R {
        ADC_SSOP2_S0DCOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Sample 1 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop2_s1dcop(&self) -> ADC_SSOP2_S1DCOP_R {
        ADC_SSOP2_S1DCOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Sample 2 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop2_s2dcop(&self) -> ADC_SSOP2_S2DCOP_R {
        ADC_SSOP2_S2DCOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Sample 3 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop2_s3dcop(&self) -> ADC_SSOP2_S3DCOP_R {
        ADC_SSOP2_S3DCOP_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssop2_s0dcop(&mut self) -> ADC_SSOP2_S0DCOP_W<SSOP2_SPEC, 0> {
        ADC_SSOP2_S0DCOP_W::new(self)
    }
    #[doc = "Bit 4 - Sample 1 Digital Comparator Operation"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssop2_s1dcop(&mut self) -> ADC_SSOP2_S1DCOP_W<SSOP2_SPEC, 4> {
        ADC_SSOP2_S1DCOP_W::new(self)
    }
    #[doc = "Bit 8 - Sample 2 Digital Comparator Operation"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssop2_s2dcop(&mut self) -> ADC_SSOP2_S2DCOP_W<SSOP2_SPEC, 8> {
        ADC_SSOP2_S2DCOP_W::new(self)
    }
    #[doc = "Bit 12 - Sample 3 Digital Comparator Operation"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssop2_s3dcop(&mut self) -> ADC_SSOP2_S3DCOP_W<SSOP2_SPEC, 12> {
        ADC_SSOP2_S3DCOP_W::new(self)
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
#[doc = "ADC Sample Sequence 2 Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssop2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssop2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSOP2_SPEC;
impl crate::RegisterSpec for SSOP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssop2::R`](R) reader structure"]
impl crate::Readable for SSOP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssop2::W`](W) writer structure"]
impl crate::Writable for SSOP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSOP2 to value 0"]
impl crate::Resettable for SSOP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
