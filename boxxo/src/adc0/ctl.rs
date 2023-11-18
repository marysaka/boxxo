#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `ADC_CTL_VREF` reader - Voltage Reference Select"]
pub type ADC_CTL_VREF_R = crate::FieldReader<ADC_CTL_VREF_A>;
#[doc = "Voltage Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_CTL_VREF_A {
    #[doc = "0: The internal reference as the voltage reference"]
    ADC_CTL_VREF_INTERNAL = 0,
}
impl From<ADC_CTL_VREF_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_CTL_VREF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_CTL_VREF_A {
    type Ux = u8;
}
impl ADC_CTL_VREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_CTL_VREF_A> {
        match self.bits {
            0 => Some(ADC_CTL_VREF_A::ADC_CTL_VREF_INTERNAL),
            _ => None,
        }
    }
    #[doc = "The internal reference as the voltage reference"]
    #[inline(always)]
    pub fn is_adc_ctl_vref_internal(&self) -> bool {
        *self == ADC_CTL_VREF_A::ADC_CTL_VREF_INTERNAL
    }
}
#[doc = "Field `ADC_CTL_VREF` writer - Voltage Reference Select"]
pub type ADC_CTL_VREF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ADC_CTL_VREF_A>;
impl<'a, REG, const O: u8> ADC_CTL_VREF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The internal reference as the voltage reference"]
    #[inline(always)]
    pub fn adc_ctl_vref_internal(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CTL_VREF_A::ADC_CTL_VREF_INTERNAL)
    }
}
#[doc = "Field `ADC_CTL_DITHER` reader - Dither Mode Enable"]
pub type ADC_CTL_DITHER_R = crate::BitReader;
#[doc = "Field `ADC_CTL_DITHER` writer - Dither Mode Enable"]
pub type ADC_CTL_DITHER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Voltage Reference Select"]
    #[inline(always)]
    pub fn adc_ctl_vref(&self) -> ADC_CTL_VREF_R {
        ADC_CTL_VREF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 6 - Dither Mode Enable"]
    #[inline(always)]
    pub fn adc_ctl_dither(&self) -> ADC_CTL_DITHER_R {
        ADC_CTL_DITHER_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Voltage Reference Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ctl_vref(&mut self) -> ADC_CTL_VREF_W<CTL_SPEC, 0> {
        ADC_CTL_VREF_W::new(self)
    }
    #[doc = "Bit 6 - Dither Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ctl_dither(&mut self) -> ADC_CTL_DITHER_W<CTL_SPEC, 6> {
        ADC_CTL_DITHER_W::new(self)
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
#[doc = "ADC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
