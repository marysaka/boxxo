#[doc = "Register `CC` reader"]
pub type R = crate::R<CC_SPEC>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CC_SPEC>;
#[doc = "Field `ADC_CC_CS` reader - ADC Clock Source"]
pub type ADC_CC_CS_R = crate::FieldReader<ADC_CC_CS_A>;
#[doc = "ADC Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_CC_CS_A {
    #[doc = "0: Either the system clock (if the PLL bypass is in effect) or the 16 MHz clock derived from PLL / 25 (default)"]
    ADC_CC_CS_SYSPLL = 0,
    #[doc = "1: PIOSC"]
    ADC_CC_CS_PIOSC = 1,
}
impl From<ADC_CC_CS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_CC_CS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_CC_CS_A {
    type Ux = u8;
}
impl ADC_CC_CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_CC_CS_A> {
        match self.bits {
            0 => Some(ADC_CC_CS_A::ADC_CC_CS_SYSPLL),
            1 => Some(ADC_CC_CS_A::ADC_CC_CS_PIOSC),
            _ => None,
        }
    }
    #[doc = "Either the system clock (if the PLL bypass is in effect) or the 16 MHz clock derived from PLL / 25 (default)"]
    #[inline(always)]
    pub fn is_adc_cc_cs_syspll(&self) -> bool {
        *self == ADC_CC_CS_A::ADC_CC_CS_SYSPLL
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn is_adc_cc_cs_piosc(&self) -> bool {
        *self == ADC_CC_CS_A::ADC_CC_CS_PIOSC
    }
}
#[doc = "Field `ADC_CC_CS` writer - ADC Clock Source"]
pub type ADC_CC_CS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADC_CC_CS_A>;
impl<'a, REG, const O: u8> ADC_CC_CS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Either the system clock (if the PLL bypass is in effect) or the 16 MHz clock derived from PLL / 25 (default)"]
    #[inline(always)]
    pub fn adc_cc_cs_syspll(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CC_CS_A::ADC_CC_CS_SYSPLL)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn adc_cc_cs_piosc(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_CC_CS_A::ADC_CC_CS_PIOSC)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC Clock Source"]
    #[inline(always)]
    pub fn adc_cc_cs(&self) -> ADC_CC_CS_R {
        ADC_CC_CS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn adc_cc_cs(&mut self) -> ADC_CC_CS_W<CC_SPEC, 0> {
        ADC_CC_CS_W::new(self)
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
#[doc = "ADC Clock Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
