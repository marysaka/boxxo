#[doc = "Register `DCRIC` writer"]
pub type W = crate::W<DCRIC_SPEC>;
#[doc = "Field `ADC_DCRIC_DCINT0` writer - Digital Comparator Interrupt 0"]
pub type ADC_DCRIC_DCINT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCINT1` writer - Digital Comparator Interrupt 1"]
pub type ADC_DCRIC_DCINT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCINT2` writer - Digital Comparator Interrupt 2"]
pub type ADC_DCRIC_DCINT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCINT3` writer - Digital Comparator Interrupt 3"]
pub type ADC_DCRIC_DCINT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCINT4` writer - Digital Comparator Interrupt 4"]
pub type ADC_DCRIC_DCINT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCINT5` writer - Digital Comparator Interrupt 5"]
pub type ADC_DCRIC_DCINT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCINT6` writer - Digital Comparator Interrupt 6"]
pub type ADC_DCRIC_DCINT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCINT7` writer - Digital Comparator Interrupt 7"]
pub type ADC_DCRIC_DCINT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCTRIG0` writer - Digital Comparator Trigger 0"]
pub type ADC_DCRIC_DCTRIG0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCTRIG1` writer - Digital Comparator Trigger 1"]
pub type ADC_DCRIC_DCTRIG1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCTRIG2` writer - Digital Comparator Trigger 2"]
pub type ADC_DCRIC_DCTRIG2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCTRIG3` writer - Digital Comparator Trigger 3"]
pub type ADC_DCRIC_DCTRIG3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCTRIG4` writer - Digital Comparator Trigger 4"]
pub type ADC_DCRIC_DCTRIG4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCTRIG5` writer - Digital Comparator Trigger 5"]
pub type ADC_DCRIC_DCTRIG5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCTRIG6` writer - Digital Comparator Trigger 6"]
pub type ADC_DCRIC_DCTRIG6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCRIC_DCTRIG7` writer - Digital Comparator Trigger 7"]
pub type ADC_DCRIC_DCTRIG7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Digital Comparator Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dcint0(&mut self) -> ADC_DCRIC_DCINT0_W<DCRIC_SPEC, 0> {
        ADC_DCRIC_DCINT0_W::new(self)
    }
    #[doc = "Bit 1 - Digital Comparator Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dcint1(&mut self) -> ADC_DCRIC_DCINT1_W<DCRIC_SPEC, 1> {
        ADC_DCRIC_DCINT1_W::new(self)
    }
    #[doc = "Bit 2 - Digital Comparator Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dcint2(&mut self) -> ADC_DCRIC_DCINT2_W<DCRIC_SPEC, 2> {
        ADC_DCRIC_DCINT2_W::new(self)
    }
    #[doc = "Bit 3 - Digital Comparator Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dcint3(&mut self) -> ADC_DCRIC_DCINT3_W<DCRIC_SPEC, 3> {
        ADC_DCRIC_DCINT3_W::new(self)
    }
    #[doc = "Bit 4 - Digital Comparator Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dcint4(&mut self) -> ADC_DCRIC_DCINT4_W<DCRIC_SPEC, 4> {
        ADC_DCRIC_DCINT4_W::new(self)
    }
    #[doc = "Bit 5 - Digital Comparator Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dcint5(&mut self) -> ADC_DCRIC_DCINT5_W<DCRIC_SPEC, 5> {
        ADC_DCRIC_DCINT5_W::new(self)
    }
    #[doc = "Bit 6 - Digital Comparator Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dcint6(&mut self) -> ADC_DCRIC_DCINT6_W<DCRIC_SPEC, 6> {
        ADC_DCRIC_DCINT6_W::new(self)
    }
    #[doc = "Bit 7 - Digital Comparator Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dcint7(&mut self) -> ADC_DCRIC_DCINT7_W<DCRIC_SPEC, 7> {
        ADC_DCRIC_DCINT7_W::new(self)
    }
    #[doc = "Bit 16 - Digital Comparator Trigger 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dctrig0(&mut self) -> ADC_DCRIC_DCTRIG0_W<DCRIC_SPEC, 16> {
        ADC_DCRIC_DCTRIG0_W::new(self)
    }
    #[doc = "Bit 17 - Digital Comparator Trigger 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dctrig1(&mut self) -> ADC_DCRIC_DCTRIG1_W<DCRIC_SPEC, 17> {
        ADC_DCRIC_DCTRIG1_W::new(self)
    }
    #[doc = "Bit 18 - Digital Comparator Trigger 2"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dctrig2(&mut self) -> ADC_DCRIC_DCTRIG2_W<DCRIC_SPEC, 18> {
        ADC_DCRIC_DCTRIG2_W::new(self)
    }
    #[doc = "Bit 19 - Digital Comparator Trigger 3"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dctrig3(&mut self) -> ADC_DCRIC_DCTRIG3_W<DCRIC_SPEC, 19> {
        ADC_DCRIC_DCTRIG3_W::new(self)
    }
    #[doc = "Bit 20 - Digital Comparator Trigger 4"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dctrig4(&mut self) -> ADC_DCRIC_DCTRIG4_W<DCRIC_SPEC, 20> {
        ADC_DCRIC_DCTRIG4_W::new(self)
    }
    #[doc = "Bit 21 - Digital Comparator Trigger 5"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dctrig5(&mut self) -> ADC_DCRIC_DCTRIG5_W<DCRIC_SPEC, 21> {
        ADC_DCRIC_DCTRIG5_W::new(self)
    }
    #[doc = "Bit 22 - Digital Comparator Trigger 6"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dctrig6(&mut self) -> ADC_DCRIC_DCTRIG6_W<DCRIC_SPEC, 22> {
        ADC_DCRIC_DCTRIG6_W::new(self)
    }
    #[doc = "Bit 23 - Digital Comparator Trigger 7"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcric_dctrig7(&mut self) -> ADC_DCRIC_DCTRIG7_W<DCRIC_SPEC, 23> {
        ADC_DCRIC_DCTRIG7_W::new(self)
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
#[doc = "ADC Digital Comparator Reset Initial Conditions\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcric::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCRIC_SPEC;
impl crate::RegisterSpec for DCRIC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dcric::W`](W) writer structure"]
impl crate::Writable for DCRIC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCRIC to value 0"]
impl crate::Resettable for DCRIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
