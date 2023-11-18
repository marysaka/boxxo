#[doc = "Register `USTAT` reader"]
pub type R = crate::R<USTAT_SPEC>;
#[doc = "Register `USTAT` writer"]
pub type W = crate::W<USTAT_SPEC>;
#[doc = "Field `ADC_USTAT_UV0` reader - SS0 FIFO Underflow"]
pub type ADC_USTAT_UV0_R = crate::BitReader;
#[doc = "Field `ADC_USTAT_UV0` writer - SS0 FIFO Underflow"]
pub type ADC_USTAT_UV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_USTAT_UV1` reader - SS1 FIFO Underflow"]
pub type ADC_USTAT_UV1_R = crate::BitReader;
#[doc = "Field `ADC_USTAT_UV1` writer - SS1 FIFO Underflow"]
pub type ADC_USTAT_UV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_USTAT_UV2` reader - SS2 FIFO Underflow"]
pub type ADC_USTAT_UV2_R = crate::BitReader;
#[doc = "Field `ADC_USTAT_UV2` writer - SS2 FIFO Underflow"]
pub type ADC_USTAT_UV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_USTAT_UV3` reader - SS3 FIFO Underflow"]
pub type ADC_USTAT_UV3_R = crate::BitReader;
#[doc = "Field `ADC_USTAT_UV3` writer - SS3 FIFO Underflow"]
pub type ADC_USTAT_UV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SS0 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv0(&self) -> ADC_USTAT_UV0_R {
        ADC_USTAT_UV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv1(&self) -> ADC_USTAT_UV1_R {
        ADC_USTAT_UV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv2(&self) -> ADC_USTAT_UV2_R {
        ADC_USTAT_UV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv3(&self) -> ADC_USTAT_UV3_R {
        ADC_USTAT_UV3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 FIFO Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ustat_uv0(&mut self) -> ADC_USTAT_UV0_W<USTAT_SPEC, 0> {
        ADC_USTAT_UV0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 FIFO Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ustat_uv1(&mut self) -> ADC_USTAT_UV1_W<USTAT_SPEC, 1> {
        ADC_USTAT_UV1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 FIFO Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ustat_uv2(&mut self) -> ADC_USTAT_UV2_W<USTAT_SPEC, 2> {
        ADC_USTAT_UV2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 FIFO Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ustat_uv3(&mut self) -> ADC_USTAT_UV3_W<USTAT_SPEC, 3> {
        ADC_USTAT_UV3_W::new(self)
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
#[doc = "ADC Underflow Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ustat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ustat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USTAT_SPEC;
impl crate::RegisterSpec for USTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ustat::R`](R) reader structure"]
impl crate::Readable for USTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ustat::W`](W) writer structure"]
impl crate::Writable for USTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USTAT to value 0"]
impl crate::Resettable for USTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
