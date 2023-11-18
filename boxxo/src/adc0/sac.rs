#[doc = "Register `SAC` reader"]
pub type R = crate::R<SAC_SPEC>;
#[doc = "Register `SAC` writer"]
pub type W = crate::W<SAC_SPEC>;
#[doc = "Field `ADC_SAC_AVG` reader - Hardware Averaging Control"]
pub type ADC_SAC_AVG_R = crate::FieldReader<ADC_SAC_AVG_A>;
#[doc = "Hardware Averaging Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_SAC_AVG_A {
    #[doc = "0: No hardware oversampling"]
    ADC_SAC_AVG_OFF = 0,
    #[doc = "1: 2x hardware oversampling"]
    ADC_SAC_AVG_2X = 1,
    #[doc = "2: 4x hardware oversampling"]
    ADC_SAC_AVG_4X = 2,
    #[doc = "3: 8x hardware oversampling"]
    ADC_SAC_AVG_8X = 3,
    #[doc = "4: 16x hardware oversampling"]
    ADC_SAC_AVG_16X = 4,
    #[doc = "5: 32x hardware oversampling"]
    ADC_SAC_AVG_32X = 5,
    #[doc = "6: 64x hardware oversampling"]
    ADC_SAC_AVG_64X = 6,
}
impl From<ADC_SAC_AVG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SAC_AVG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_SAC_AVG_A {
    type Ux = u8;
}
impl ADC_SAC_AVG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_SAC_AVG_A> {
        match self.bits {
            0 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_OFF),
            1 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_2X),
            2 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_4X),
            3 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_8X),
            4 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_16X),
            5 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_32X),
            6 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_64X),
            _ => None,
        }
    }
    #[doc = "No hardware oversampling"]
    #[inline(always)]
    pub fn is_adc_sac_avg_off(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_OFF
    }
    #[doc = "2x hardware oversampling"]
    #[inline(always)]
    pub fn is_adc_sac_avg_2x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_2X
    }
    #[doc = "4x hardware oversampling"]
    #[inline(always)]
    pub fn is_adc_sac_avg_4x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_4X
    }
    #[doc = "8x hardware oversampling"]
    #[inline(always)]
    pub fn is_adc_sac_avg_8x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_8X
    }
    #[doc = "16x hardware oversampling"]
    #[inline(always)]
    pub fn is_adc_sac_avg_16x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_16X
    }
    #[doc = "32x hardware oversampling"]
    #[inline(always)]
    pub fn is_adc_sac_avg_32x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_32X
    }
    #[doc = "64x hardware oversampling"]
    #[inline(always)]
    pub fn is_adc_sac_avg_64x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_64X
    }
}
#[doc = "Field `ADC_SAC_AVG` writer - Hardware Averaging Control"]
pub type ADC_SAC_AVG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, ADC_SAC_AVG_A>;
impl<'a, REG, const O: u8> ADC_SAC_AVG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_off(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_OFF)
    }
    #[doc = "2x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_2x(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_2X)
    }
    #[doc = "4x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_4x(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_4X)
    }
    #[doc = "8x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_8x(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_8X)
    }
    #[doc = "16x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_16x(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_16X)
    }
    #[doc = "32x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_32x(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_32X)
    }
    #[doc = "64x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_64x(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_64X)
    }
}
impl R {
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    pub fn adc_sac_avg(&self) -> ADC_SAC_AVG_R {
        ADC_SAC_AVG_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_sac_avg(&mut self) -> ADC_SAC_AVG_W<SAC_SPEC, 0> {
        ADC_SAC_AVG_W::new(self)
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
#[doc = "ADC Sample Averaging Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAC_SPEC;
impl crate::RegisterSpec for SAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sac::R`](R) reader structure"]
impl crate::Readable for SAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sac::W`](W) writer structure"]
impl crate::Writable for SAC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAC to value 0"]
impl crate::Resettable for SAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
