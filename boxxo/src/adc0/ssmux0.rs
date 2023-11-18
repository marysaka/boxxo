#[doc = "Register `SSMUX0` reader"]
pub type R = crate::R<SSMUX0_SPEC>;
#[doc = "Register `SSMUX0` writer"]
pub type W = crate::W<SSMUX0_SPEC>;
#[doc = "Field `ADC_SSMUX0_MUX0` reader - 1st Sample Input Select"]
pub type ADC_SSMUX0_MUX0_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX0_MUX0` writer - 1st Sample Input Select"]
pub type ADC_SSMUX0_MUX0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX0_MUX1` reader - 2nd Sample Input Select"]
pub type ADC_SSMUX0_MUX1_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX0_MUX1` writer - 2nd Sample Input Select"]
pub type ADC_SSMUX0_MUX1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX0_MUX2` reader - 3rd Sample Input Select"]
pub type ADC_SSMUX0_MUX2_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX0_MUX2` writer - 3rd Sample Input Select"]
pub type ADC_SSMUX0_MUX2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX0_MUX3` reader - 4th Sample Input Select"]
pub type ADC_SSMUX0_MUX3_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX0_MUX3` writer - 4th Sample Input Select"]
pub type ADC_SSMUX0_MUX3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX0_MUX4` reader - 5th Sample Input Select"]
pub type ADC_SSMUX0_MUX4_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX0_MUX4` writer - 5th Sample Input Select"]
pub type ADC_SSMUX0_MUX4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX0_MUX5` reader - 6th Sample Input Select"]
pub type ADC_SSMUX0_MUX5_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX0_MUX5` writer - 6th Sample Input Select"]
pub type ADC_SSMUX0_MUX5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX0_MUX6` reader - 7th Sample Input Select"]
pub type ADC_SSMUX0_MUX6_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX0_MUX6` writer - 7th Sample Input Select"]
pub type ADC_SSMUX0_MUX6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADC_SSMUX0_MUX7` reader - 8th Sample Input Select"]
pub type ADC_SSMUX0_MUX7_R = crate::FieldReader;
#[doc = "Field `ADC_SSMUX0_MUX7` writer - 8th Sample Input Select"]
pub type ADC_SSMUX0_MUX7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux0(&self) -> ADC_SSMUX0_MUX0_R {
        ADC_SSMUX0_MUX0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux1(&self) -> ADC_SSMUX0_MUX1_R {
        ADC_SSMUX0_MUX1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux2(&self) -> ADC_SSMUX0_MUX2_R {
        ADC_SSMUX0_MUX2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux3(&self) -> ADC_SSMUX0_MUX3_R {
        ADC_SSMUX0_MUX3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux4(&self) -> ADC_SSMUX0_MUX4_R {
        ADC_SSMUX0_MUX4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux5(&self) -> ADC_SSMUX0_MUX5_R {
        ADC_SSMUX0_MUX5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux6(&self) -> ADC_SSMUX0_MUX6_R {
        ADC_SSMUX0_MUX6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux7(&self) -> ADC_SSMUX0_MUX7_R {
        ADC_SSMUX0_MUX7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux0_mux0(&mut self) -> ADC_SSMUX0_MUX0_W<SSMUX0_SPEC, 0> {
        ADC_SSMUX0_MUX0_W::new(self)
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux0_mux1(&mut self) -> ADC_SSMUX0_MUX1_W<SSMUX0_SPEC, 4> {
        ADC_SSMUX0_MUX1_W::new(self)
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux0_mux2(&mut self) -> ADC_SSMUX0_MUX2_W<SSMUX0_SPEC, 8> {
        ADC_SSMUX0_MUX2_W::new(self)
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux0_mux3(&mut self) -> ADC_SSMUX0_MUX3_W<SSMUX0_SPEC, 12> {
        ADC_SSMUX0_MUX3_W::new(self)
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux0_mux4(&mut self) -> ADC_SSMUX0_MUX4_W<SSMUX0_SPEC, 16> {
        ADC_SSMUX0_MUX4_W::new(self)
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux0_mux5(&mut self) -> ADC_SSMUX0_MUX5_W<SSMUX0_SPEC, 20> {
        ADC_SSMUX0_MUX5_W::new(self)
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux0_mux6(&mut self) -> ADC_SSMUX0_MUX6_W<SSMUX0_SPEC, 24> {
        ADC_SSMUX0_MUX6_W::new(self)
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssmux0_mux7(&mut self) -> ADC_SSMUX0_MUX7_W<SSMUX0_SPEC, 28> {
        ADC_SSMUX0_MUX7_W::new(self)
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
#[doc = "ADC Sample Sequence Input Multiplexer Select 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssmux0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssmux0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSMUX0_SPEC;
impl crate::RegisterSpec for SSMUX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssmux0::R`](R) reader structure"]
impl crate::Readable for SSMUX0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssmux0::W`](W) writer structure"]
impl crate::Writable for SSMUX0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSMUX0 to value 0"]
impl crate::Resettable for SSMUX0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
