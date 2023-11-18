#[doc = "Register `SSPRI` reader"]
pub type R = crate::R<SSPRI_SPEC>;
#[doc = "Register `SSPRI` writer"]
pub type W = crate::W<SSPRI_SPEC>;
#[doc = "Field `ADC_SSPRI_SS0` reader - SS0 Priority"]
pub type ADC_SSPRI_SS0_R = crate::FieldReader;
#[doc = "Field `ADC_SSPRI_SS0` writer - SS0 Priority"]
pub type ADC_SSPRI_SS0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ADC_SSPRI_SS1` reader - SS1 Priority"]
pub type ADC_SSPRI_SS1_R = crate::FieldReader;
#[doc = "Field `ADC_SSPRI_SS1` writer - SS1 Priority"]
pub type ADC_SSPRI_SS1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ADC_SSPRI_SS2` reader - SS2 Priority"]
pub type ADC_SSPRI_SS2_R = crate::FieldReader;
#[doc = "Field `ADC_SSPRI_SS2` writer - SS2 Priority"]
pub type ADC_SSPRI_SS2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ADC_SSPRI_SS3` reader - SS3 Priority"]
pub type ADC_SSPRI_SS3_R = crate::FieldReader;
#[doc = "Field `ADC_SSPRI_SS3` writer - SS3 Priority"]
pub type ADC_SSPRI_SS3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0(&self) -> ADC_SSPRI_SS0_R {
        ADC_SSPRI_SS0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1(&self) -> ADC_SSPRI_SS1_R {
        ADC_SSPRI_SS1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2(&self) -> ADC_SSPRI_SS2_R {
        ADC_SSPRI_SS2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3(&self) -> ADC_SSPRI_SS3_R {
        ADC_SSPRI_SS3_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn adc_sspri_ss0(&mut self) -> ADC_SSPRI_SS0_W<SSPRI_SPEC, 0> {
        ADC_SSPRI_SS0_W::new(self)
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn adc_sspri_ss1(&mut self) -> ADC_SSPRI_SS1_W<SSPRI_SPEC, 4> {
        ADC_SSPRI_SS1_W::new(self)
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn adc_sspri_ss2(&mut self) -> ADC_SSPRI_SS2_W<SSPRI_SPEC, 8> {
        ADC_SSPRI_SS2_W::new(self)
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn adc_sspri_ss3(&mut self) -> ADC_SSPRI_SS3_W<SSPRI_SPEC, 12> {
        ADC_SSPRI_SS3_W::new(self)
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
#[doc = "ADC Sample Sequencer Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sspri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPRI_SPEC;
impl crate::RegisterSpec for SSPRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspri::R`](R) reader structure"]
impl crate::Readable for SSPRI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspri::W`](W) writer structure"]
impl crate::Writable for SSPRI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSPRI to value 0"]
impl crate::Resettable for SSPRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
