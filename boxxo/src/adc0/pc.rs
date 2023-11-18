#[doc = "Register `PC` reader"]
pub type R = crate::R<PC_SPEC>;
#[doc = "Register `PC` writer"]
pub type W = crate::W<PC_SPEC>;
#[doc = "Field `ADC_PC_SR` reader - ADC Sample Rate"]
pub type ADC_PC_SR_R = crate::FieldReader<ADC_PC_SR_A>;
#[doc = "ADC Sample Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_PC_SR_A {
    #[doc = "1: 125 ksps"]
    ADC_PC_SR_125K = 1,
    #[doc = "3: 250 ksps"]
    ADC_PC_SR_250K = 3,
    #[doc = "5: 500 ksps"]
    ADC_PC_SR_500K = 5,
    #[doc = "7: 1 Msps"]
    ADC_PC_SR_1M = 7,
}
impl From<ADC_PC_SR_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_PC_SR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_PC_SR_A {
    type Ux = u8;
}
impl ADC_PC_SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_PC_SR_A> {
        match self.bits {
            1 => Some(ADC_PC_SR_A::ADC_PC_SR_125K),
            3 => Some(ADC_PC_SR_A::ADC_PC_SR_250K),
            5 => Some(ADC_PC_SR_A::ADC_PC_SR_500K),
            7 => Some(ADC_PC_SR_A::ADC_PC_SR_1M),
            _ => None,
        }
    }
    #[doc = "125 ksps"]
    #[inline(always)]
    pub fn is_adc_pc_sr_125k(&self) -> bool {
        *self == ADC_PC_SR_A::ADC_PC_SR_125K
    }
    #[doc = "250 ksps"]
    #[inline(always)]
    pub fn is_adc_pc_sr_250k(&self) -> bool {
        *self == ADC_PC_SR_A::ADC_PC_SR_250K
    }
    #[doc = "500 ksps"]
    #[inline(always)]
    pub fn is_adc_pc_sr_500k(&self) -> bool {
        *self == ADC_PC_SR_A::ADC_PC_SR_500K
    }
    #[doc = "1 Msps"]
    #[inline(always)]
    pub fn is_adc_pc_sr_1m(&self) -> bool {
        *self == ADC_PC_SR_A::ADC_PC_SR_1M
    }
}
#[doc = "Field `ADC_PC_SR` writer - ADC Sample Rate"]
pub type ADC_PC_SR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADC_PC_SR_A>;
impl<'a, REG, const O: u8> ADC_PC_SR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "125 ksps"]
    #[inline(always)]
    pub fn adc_pc_sr_125k(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_PC_SR_A::ADC_PC_SR_125K)
    }
    #[doc = "250 ksps"]
    #[inline(always)]
    pub fn adc_pc_sr_250k(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_PC_SR_A::ADC_PC_SR_250K)
    }
    #[doc = "500 ksps"]
    #[inline(always)]
    pub fn adc_pc_sr_500k(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_PC_SR_A::ADC_PC_SR_500K)
    }
    #[doc = "1 Msps"]
    #[inline(always)]
    pub fn adc_pc_sr_1m(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_PC_SR_A::ADC_PC_SR_1M)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC Sample Rate"]
    #[inline(always)]
    pub fn adc_pc_sr(&self) -> ADC_PC_SR_R {
        ADC_PC_SR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC Sample Rate"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pc_sr(&mut self) -> ADC_PC_SR_W<PC_SPEC, 0> {
        ADC_PC_SR_W::new(self)
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
#[doc = "ADC Peripheral Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc::R`](R) reader structure"]
impl crate::Readable for PC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc::W`](W) writer structure"]
impl crate::Writable for PC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
