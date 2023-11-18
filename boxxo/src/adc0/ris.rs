#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `ADC_RIS_INR0` reader - SS0 Raw Interrupt Status"]
pub type ADC_RIS_INR0_R = crate::BitReader;
#[doc = "Field `ADC_RIS_INR0` writer - SS0 Raw Interrupt Status"]
pub type ADC_RIS_INR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_RIS_INR1` reader - SS1 Raw Interrupt Status"]
pub type ADC_RIS_INR1_R = crate::BitReader;
#[doc = "Field `ADC_RIS_INR1` writer - SS1 Raw Interrupt Status"]
pub type ADC_RIS_INR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_RIS_INR2` reader - SS2 Raw Interrupt Status"]
pub type ADC_RIS_INR2_R = crate::BitReader;
#[doc = "Field `ADC_RIS_INR2` writer - SS2 Raw Interrupt Status"]
pub type ADC_RIS_INR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_RIS_INR3` reader - SS3 Raw Interrupt Status"]
pub type ADC_RIS_INR3_R = crate::BitReader;
#[doc = "Field `ADC_RIS_INR3` writer - SS3 Raw Interrupt Status"]
pub type ADC_RIS_INR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_RIS_INRDC` reader - Digital Comparator Raw Interrupt Status"]
pub type ADC_RIS_INRDC_R = crate::BitReader;
#[doc = "Field `ADC_RIS_INRDC` writer - Digital Comparator Raw Interrupt Status"]
pub type ADC_RIS_INRDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SS0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr0(&self) -> ADC_RIS_INR0_R {
        ADC_RIS_INR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr1(&self) -> ADC_RIS_INR1_R {
        ADC_RIS_INR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr2(&self) -> ADC_RIS_INR2_R {
        ADC_RIS_INR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr3(&self) -> ADC_RIS_INR3_R {
        ADC_RIS_INR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Digital Comparator Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inrdc(&self) -> ADC_RIS_INRDC_R {
        ADC_RIS_INRDC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ris_inr0(&mut self) -> ADC_RIS_INR0_W<RIS_SPEC, 0> {
        ADC_RIS_INR0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ris_inr1(&mut self) -> ADC_RIS_INR1_W<RIS_SPEC, 1> {
        ADC_RIS_INR1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ris_inr2(&mut self) -> ADC_RIS_INR2_W<RIS_SPEC, 2> {
        ADC_RIS_INR2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ris_inr3(&mut self) -> ADC_RIS_INR3_W<RIS_SPEC, 3> {
        ADC_RIS_INR3_W::new(self)
    }
    #[doc = "Bit 16 - Digital Comparator Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ris_inrdc(&mut self) -> ADC_RIS_INRDC_W<RIS_SPEC, 16> {
        ADC_RIS_INRDC_W::new(self)
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
#[doc = "ADC Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
