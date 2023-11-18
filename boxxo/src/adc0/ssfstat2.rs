#[doc = "Register `SSFSTAT2` reader"]
pub type R = crate::R<SSFSTAT2_SPEC>;
#[doc = "Register `SSFSTAT2` writer"]
pub type W = crate::W<SSFSTAT2_SPEC>;
#[doc = "Field `ADC_SSFSTAT2_TPTR` reader - FIFO Tail Pointer"]
pub type ADC_SSFSTAT2_TPTR_R = crate::FieldReader;
#[doc = "Field `ADC_SSFSTAT2_TPTR` writer - FIFO Tail Pointer"]
pub type ADC_SSFSTAT2_TPTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSFSTAT2_HPTR` reader - FIFO Head Pointer"]
pub type ADC_SSFSTAT2_HPTR_R = crate::FieldReader;
#[doc = "Field `ADC_SSFSTAT2_HPTR` writer - FIFO Head Pointer"]
pub type ADC_SSFSTAT2_HPTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSFSTAT2_EMPTY` reader - FIFO Empty"]
pub type ADC_SSFSTAT2_EMPTY_R = crate::BitReader;
#[doc = "Field `ADC_SSFSTAT2_EMPTY` writer - FIFO Empty"]
pub type ADC_SSFSTAT2_EMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSFSTAT2_FULL` reader - FIFO Full"]
pub type ADC_SSFSTAT2_FULL_R = crate::BitReader;
#[doc = "Field `ADC_SSFSTAT2_FULL` writer - FIFO Full"]
pub type ADC_SSFSTAT2_FULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - FIFO Tail Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat2_tptr(&self) -> ADC_SSFSTAT2_TPTR_R {
        ADC_SSFSTAT2_TPTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFO Head Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat2_hptr(&self) -> ADC_SSFSTAT2_HPTR_R {
        ADC_SSFSTAT2_HPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - FIFO Empty"]
    #[inline(always)]
    pub fn adc_ssfstat2_empty(&self) -> ADC_SSFSTAT2_EMPTY_R {
        ADC_SSFSTAT2_EMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - FIFO Full"]
    #[inline(always)]
    pub fn adc_ssfstat2_full(&self) -> ADC_SSFSTAT2_FULL_R {
        ADC_SSFSTAT2_FULL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO Tail Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssfstat2_tptr(&mut self) -> ADC_SSFSTAT2_TPTR_W<SSFSTAT2_SPEC, 0> {
        ADC_SSFSTAT2_TPTR_W::new(self)
    }
    #[doc = "Bits 4:7 - FIFO Head Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssfstat2_hptr(&mut self) -> ADC_SSFSTAT2_HPTR_W<SSFSTAT2_SPEC, 4> {
        ADC_SSFSTAT2_HPTR_W::new(self)
    }
    #[doc = "Bit 8 - FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssfstat2_empty(&mut self) -> ADC_SSFSTAT2_EMPTY_W<SSFSTAT2_SPEC, 8> {
        ADC_SSFSTAT2_EMPTY_W::new(self)
    }
    #[doc = "Bit 12 - FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssfstat2_full(&mut self) -> ADC_SSFSTAT2_FULL_W<SSFSTAT2_SPEC, 12> {
        ADC_SSFSTAT2_FULL_W::new(self)
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
#[doc = "ADC Sample Sequence FIFO 2 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssfstat2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssfstat2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSFSTAT2_SPEC;
impl crate::RegisterSpec for SSFSTAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssfstat2::R`](R) reader structure"]
impl crate::Readable for SSFSTAT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssfstat2::W`](W) writer structure"]
impl crate::Writable for SSFSTAT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSFSTAT2 to value 0"]
impl crate::Resettable for SSFSTAT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
