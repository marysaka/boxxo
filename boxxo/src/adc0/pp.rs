#[doc = "Register `PP` reader"]
pub type R = crate::R<PP_SPEC>;
#[doc = "Register `PP` writer"]
pub type W = crate::W<PP_SPEC>;
#[doc = "Field `ADC_PP_MSR` reader - Maximum ADC Sample Rate"]
pub type ADC_PP_MSR_R = crate::FieldReader<ADC_PP_MSR_A>;
#[doc = "Maximum ADC Sample Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_PP_MSR_A {
    #[doc = "1: 125 ksps"]
    ADC_PP_MSR_125K = 1,
    #[doc = "3: 250 ksps"]
    ADC_PP_MSR_250K = 3,
    #[doc = "5: 500 ksps"]
    ADC_PP_MSR_500K = 5,
    #[doc = "7: 1 Msps"]
    ADC_PP_MSR_1M = 7,
}
impl From<ADC_PP_MSR_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_PP_MSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_PP_MSR_A {
    type Ux = u8;
}
impl ADC_PP_MSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_PP_MSR_A> {
        match self.bits {
            1 => Some(ADC_PP_MSR_A::ADC_PP_MSR_125K),
            3 => Some(ADC_PP_MSR_A::ADC_PP_MSR_250K),
            5 => Some(ADC_PP_MSR_A::ADC_PP_MSR_500K),
            7 => Some(ADC_PP_MSR_A::ADC_PP_MSR_1M),
            _ => None,
        }
    }
    #[doc = "125 ksps"]
    #[inline(always)]
    pub fn is_adc_pp_msr_125k(&self) -> bool {
        *self == ADC_PP_MSR_A::ADC_PP_MSR_125K
    }
    #[doc = "250 ksps"]
    #[inline(always)]
    pub fn is_adc_pp_msr_250k(&self) -> bool {
        *self == ADC_PP_MSR_A::ADC_PP_MSR_250K
    }
    #[doc = "500 ksps"]
    #[inline(always)]
    pub fn is_adc_pp_msr_500k(&self) -> bool {
        *self == ADC_PP_MSR_A::ADC_PP_MSR_500K
    }
    #[doc = "1 Msps"]
    #[inline(always)]
    pub fn is_adc_pp_msr_1m(&self) -> bool {
        *self == ADC_PP_MSR_A::ADC_PP_MSR_1M
    }
}
#[doc = "Field `ADC_PP_MSR` writer - Maximum ADC Sample Rate"]
pub type ADC_PP_MSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADC_PP_MSR_A>;
impl<'a, REG, const O: u8> ADC_PP_MSR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "125 ksps"]
    #[inline(always)]
    pub fn adc_pp_msr_125k(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_PP_MSR_A::ADC_PP_MSR_125K)
    }
    #[doc = "250 ksps"]
    #[inline(always)]
    pub fn adc_pp_msr_250k(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_PP_MSR_A::ADC_PP_MSR_250K)
    }
    #[doc = "500 ksps"]
    #[inline(always)]
    pub fn adc_pp_msr_500k(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_PP_MSR_A::ADC_PP_MSR_500K)
    }
    #[doc = "1 Msps"]
    #[inline(always)]
    pub fn adc_pp_msr_1m(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_PP_MSR_A::ADC_PP_MSR_1M)
    }
}
#[doc = "Field `ADC_PP_CH` reader - ADC Channel Count"]
pub type ADC_PP_CH_R = crate::FieldReader;
#[doc = "Field `ADC_PP_CH` writer - ADC Channel Count"]
pub type ADC_PP_CH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `ADC_PP_DC` reader - Digital Comparator Count"]
pub type ADC_PP_DC_R = crate::FieldReader;
#[doc = "Field `ADC_PP_DC` writer - Digital Comparator Count"]
pub type ADC_PP_DC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `ADC_PP_TYPE` reader - ADC Architecture"]
pub type ADC_PP_TYPE_R = crate::FieldReader<ADC_PP_TYPE_A>;
#[doc = "ADC Architecture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_PP_TYPE_A {
    #[doc = "0: SAR"]
    ADC_PP_TYPE_SAR = 0,
}
impl From<ADC_PP_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_PP_TYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_PP_TYPE_A {
    type Ux = u8;
}
impl ADC_PP_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_PP_TYPE_A> {
        match self.bits {
            0 => Some(ADC_PP_TYPE_A::ADC_PP_TYPE_SAR),
            _ => None,
        }
    }
    #[doc = "SAR"]
    #[inline(always)]
    pub fn is_adc_pp_type_sar(&self) -> bool {
        *self == ADC_PP_TYPE_A::ADC_PP_TYPE_SAR
    }
}
#[doc = "Field `ADC_PP_TYPE` writer - ADC Architecture"]
pub type ADC_PP_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ADC_PP_TYPE_A>;
impl<'a, REG, const O: u8> ADC_PP_TYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SAR"]
    #[inline(always)]
    pub fn adc_pp_type_sar(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_PP_TYPE_A::ADC_PP_TYPE_SAR)
    }
}
#[doc = "Field `ADC_PP_RSL` reader - Resolution"]
pub type ADC_PP_RSL_R = crate::FieldReader;
#[doc = "Field `ADC_PP_RSL` writer - Resolution"]
pub type ADC_PP_RSL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `ADC_PP_TS` reader - Temperature Sensor"]
pub type ADC_PP_TS_R = crate::BitReader;
#[doc = "Field `ADC_PP_TS` writer - Temperature Sensor"]
pub type ADC_PP_TS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Maximum ADC Sample Rate"]
    #[inline(always)]
    pub fn adc_pp_msr(&self) -> ADC_PP_MSR_R {
        ADC_PP_MSR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - ADC Channel Count"]
    #[inline(always)]
    pub fn adc_pp_ch(&self) -> ADC_PP_CH_R {
        ADC_PP_CH_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - Digital Comparator Count"]
    #[inline(always)]
    pub fn adc_pp_dc(&self) -> ADC_PP_DC_R {
        ADC_PP_DC_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - ADC Architecture"]
    #[inline(always)]
    pub fn adc_pp_type(&self) -> ADC_PP_TYPE_R {
        ADC_PP_TYPE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Resolution"]
    #[inline(always)]
    pub fn adc_pp_rsl(&self) -> ADC_PP_RSL_R {
        ADC_PP_RSL_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Temperature Sensor"]
    #[inline(always)]
    pub fn adc_pp_ts(&self) -> ADC_PP_TS_R {
        ADC_PP_TS_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Maximum ADC Sample Rate"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pp_msr(&mut self) -> ADC_PP_MSR_W<PP_SPEC, 0> {
        ADC_PP_MSR_W::new(self)
    }
    #[doc = "Bits 4:9 - ADC Channel Count"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pp_ch(&mut self) -> ADC_PP_CH_W<PP_SPEC, 4> {
        ADC_PP_CH_W::new(self)
    }
    #[doc = "Bits 10:15 - Digital Comparator Count"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pp_dc(&mut self) -> ADC_PP_DC_W<PP_SPEC, 10> {
        ADC_PP_DC_W::new(self)
    }
    #[doc = "Bits 16:17 - ADC Architecture"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pp_type(&mut self) -> ADC_PP_TYPE_W<PP_SPEC, 16> {
        ADC_PP_TYPE_W::new(self)
    }
    #[doc = "Bits 18:22 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pp_rsl(&mut self) -> ADC_PP_RSL_W<PP_SPEC, 18> {
        ADC_PP_RSL_W::new(self)
    }
    #[doc = "Bit 23 - Temperature Sensor"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pp_ts(&mut self) -> ADC_PP_TS_W<PP_SPEC, 23> {
        ADC_PP_TS_W::new(self)
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
#[doc = "ADC Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp::R`](R) reader structure"]
impl crate::Readable for PP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pp::W`](W) writer structure"]
impl crate::Writable for PP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
