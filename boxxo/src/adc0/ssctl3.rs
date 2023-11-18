#[doc = "Register `SSCTL3` reader"]
pub type R = crate::R<SSCTL3_SPEC>;
#[doc = "Register `SSCTL3` writer"]
pub type W = crate::W<SSCTL3_SPEC>;
#[doc = "Field `ADC_SSCTL3_D0` reader - 1st Sample Diff Input Select"]
pub type ADC_SSCTL3_D0_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL3_D0` writer - 1st Sample Diff Input Select"]
pub type ADC_SSCTL3_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL3_END0` reader - 1st Sample is End of Sequence"]
pub type ADC_SSCTL3_END0_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL3_END0` writer - 1st Sample is End of Sequence"]
pub type ADC_SSCTL3_END0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL3_IE0` reader - 1st Sample Interrupt Enable"]
pub type ADC_SSCTL3_IE0_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL3_IE0` writer - 1st Sample Interrupt Enable"]
pub type ADC_SSCTL3_IE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL3_TS0` reader - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL3_TS0_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL3_TS0` writer - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL3_TS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl3_d0(&self) -> ADC_SSCTL3_D0_R {
        ADC_SSCTL3_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl3_end0(&self) -> ADC_SSCTL3_END0_R {
        ADC_SSCTL3_END0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl3_ie0(&self) -> ADC_SSCTL3_IE0_R {
        ADC_SSCTL3_IE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl3_ts0(&self) -> ADC_SSCTL3_TS0_R {
        ADC_SSCTL3_TS0_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl3_d0(&mut self) -> ADC_SSCTL3_D0_W<SSCTL3_SPEC, 0> {
        ADC_SSCTL3_D0_W::new(self)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl3_end0(&mut self) -> ADC_SSCTL3_END0_W<SSCTL3_SPEC, 1> {
        ADC_SSCTL3_END0_W::new(self)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl3_ie0(&mut self) -> ADC_SSCTL3_IE0_W<SSCTL3_SPEC, 2> {
        ADC_SSCTL3_IE0_W::new(self)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl3_ts0(&mut self) -> ADC_SSCTL3_TS0_W<SSCTL3_SPEC, 3> {
        ADC_SSCTL3_TS0_W::new(self)
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
#[doc = "ADC Sample Sequence Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSCTL3_SPEC;
impl crate::RegisterSpec for SSCTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssctl3::R`](R) reader structure"]
impl crate::Readable for SSCTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssctl3::W`](W) writer structure"]
impl crate::Writable for SSCTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSCTL3 to value 0"]
impl crate::Resettable for SSCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
