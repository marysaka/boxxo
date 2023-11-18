#[doc = "Register `SSFIFO2` reader"]
pub type R = crate::R<SSFIFO2_SPEC>;
#[doc = "Register `SSFIFO2` writer"]
pub type W = crate::W<SSFIFO2_SPEC>;
#[doc = "Field `ADC_SSFIFO2_DATA` reader - Conversion Result Data"]
pub type ADC_SSFIFO2_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `ADC_SSFIFO2_DATA` writer - Conversion Result Data"]
pub type ADC_SSFIFO2_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Conversion Result Data"]
    #[inline(always)]
    pub fn adc_ssfifo2_data(&self) -> ADC_SSFIFO2_DATA_R {
        ADC_SSFIFO2_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Conversion Result Data"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssfifo2_data(&mut self) -> ADC_SSFIFO2_DATA_W<SSFIFO2_SPEC, 0> {
        ADC_SSFIFO2_DATA_W::new(self)
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
#[doc = "ADC Sample Sequence Result FIFO 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfifo2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfifo2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSFIFO2_SPEC;
impl crate::RegisterSpec for SSFIFO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssfifo2::R`](R) reader structure"]
impl crate::Readable for SSFIFO2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssfifo2::W`](W) writer structure"]
impl crate::Writable for SSFIFO2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSFIFO2 to value 0"]
impl crate::Resettable for SSFIFO2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
