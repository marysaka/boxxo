#[doc = "Register `SSMUX2` reader"]
pub type R = crate::R<SSMUX2_SPEC>;
#[doc = "Register `SSMUX2` writer"]
pub type W = crate::W<SSMUX2_SPEC>;
#[doc = "Field `ADC_SSMUX2_MUX0` reader - 1st Sample Input Select"]
pub type ADC_SSMUX2_MUX0_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX2_MUX0` writer - 1st Sample Input Select"]
pub type ADC_SSMUX2_MUX0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX2_MUX1` reader - 2nd Sample Input Select"]
pub type ADC_SSMUX2_MUX1_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX2_MUX1` writer - 2nd Sample Input Select"]
pub type ADC_SSMUX2_MUX1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX2_MUX2` reader - 3rd Sample Input Select"]
pub type ADC_SSMUX2_MUX2_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX2_MUX2` writer - 3rd Sample Input Select"]
pub type ADC_SSMUX2_MUX2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX2_MUX3` reader - 4th Sample Input Select"]
pub type ADC_SSMUX2_MUX3_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX2_MUX3` writer - 4th Sample Input Select"]
pub type ADC_SSMUX2_MUX3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux2_mux0(&self) -> ADC_SSMUX2_MUX0_R {
        ADC_SSMUX2_MUX0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux2_mux1(&self) -> ADC_SSMUX2_MUX1_R {
        ADC_SSMUX2_MUX1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux2_mux2(&self) -> ADC_SSMUX2_MUX2_R {
        ADC_SSMUX2_MUX2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux2_mux3(&self) -> ADC_SSMUX2_MUX3_R {
        ADC_SSMUX2_MUX3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux2_mux0(&mut self) -> ADC_SSMUX2_MUX0_W<SSMUX2_SPEC, 0> {
        ADC_SSMUX2_MUX0_W::new(self)
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux2_mux1(&mut self) -> ADC_SSMUX2_MUX1_W<SSMUX2_SPEC, 4> {
        ADC_SSMUX2_MUX1_W::new(self)
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux2_mux2(&mut self) -> ADC_SSMUX2_MUX2_W<SSMUX2_SPEC, 8> {
        ADC_SSMUX2_MUX2_W::new(self)
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux2_mux3(&mut self) -> ADC_SSMUX2_MUX3_W<SSMUX2_SPEC, 12> {
        ADC_SSMUX2_MUX3_W::new(self)
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
#[doc = "ADC Sample Sequence Input Multiplexer Select 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssmux2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssmux2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSMUX2_SPEC;
impl crate::RegisterSpec for SSMUX2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssmux2::R`](R) reader structure"]
impl crate::Readable for SSMUX2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssmux2::W`](W) writer structure"]
impl crate::Writable for SSMUX2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSMUX2 to value 0"]
impl crate::Resettable for SSMUX2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
