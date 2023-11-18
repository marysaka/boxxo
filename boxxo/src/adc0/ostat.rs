#[doc = "Register `OSTAT` reader"]
pub type R = crate::R<OSTAT_SPEC>;
#[doc = "Register `OSTAT` writer"]
pub type W = crate::W<OSTAT_SPEC>;
#[doc = "Field `ADC_OSTAT_OV0` reader - SS0 FIFO Overflow"]
pub type ADC_OSTAT_OV0_R = crate::BitReader;
#[doc = "Field `ADC_OSTAT_OV0` writer - SS0 FIFO Overflow"]
pub type ADC_OSTAT_OV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_OSTAT_OV1` reader - SS1 FIFO Overflow"]
pub type ADC_OSTAT_OV1_R = crate::BitReader;
#[doc = "Field `ADC_OSTAT_OV1` writer - SS1 FIFO Overflow"]
pub type ADC_OSTAT_OV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_OSTAT_OV2` reader - SS2 FIFO Overflow"]
pub type ADC_OSTAT_OV2_R = crate::BitReader;
#[doc = "Field `ADC_OSTAT_OV2` writer - SS2 FIFO Overflow"]
pub type ADC_OSTAT_OV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_OSTAT_OV3` reader - SS3 FIFO Overflow"]
pub type ADC_OSTAT_OV3_R = crate::BitReader;
#[doc = "Field `ADC_OSTAT_OV3` writer - SS3 FIFO Overflow"]
pub type ADC_OSTAT_OV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SS0 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov0(&self) -> ADC_OSTAT_OV0_R {
        ADC_OSTAT_OV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov1(&self) -> ADC_OSTAT_OV1_R {
        ADC_OSTAT_OV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov2(&self) -> ADC_OSTAT_OV2_R {
        ADC_OSTAT_OV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov3(&self) -> ADC_OSTAT_OV3_R {
        ADC_OSTAT_OV3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 FIFO Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ostat_ov0(&mut self) -> ADC_OSTAT_OV0_W<OSTAT_SPEC, 0> {
        ADC_OSTAT_OV0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 FIFO Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ostat_ov1(&mut self) -> ADC_OSTAT_OV1_W<OSTAT_SPEC, 1> {
        ADC_OSTAT_OV1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 FIFO Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ostat_ov2(&mut self) -> ADC_OSTAT_OV2_W<OSTAT_SPEC, 2> {
        ADC_OSTAT_OV2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 FIFO Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ostat_ov3(&mut self) -> ADC_OSTAT_OV3_W<OSTAT_SPEC, 3> {
        ADC_OSTAT_OV3_W::new(self)
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
#[doc = "ADC Overflow Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ostat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ostat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSTAT_SPEC;
impl crate::RegisterSpec for OSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ostat::R`](R) reader structure"]
impl crate::Readable for OSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ostat::W`](W) writer structure"]
impl crate::Writable for OSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSTAT to value 0"]
impl crate::Resettable for OSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
