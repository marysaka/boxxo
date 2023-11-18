#[doc = "Register `DCCTL1` reader"]
pub type R = crate::R<DCCTL1_SPEC>;
#[doc = "Register `DCCTL1` writer"]
pub type W = crate::W<DCCTL1_SPEC>;
#[doc = "Field `ADC_DCCTL1_CIM` reader - Comparison Interrupt Mode"]
pub type ADC_DCCTL1_CIM_R = crate::FieldReader<ADC_DCCTL1_CIM_A>;
#[doc = "Comparison Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_DCCTL1_CIM_A {
    #[doc = "0: Always"]
    ADC_DCCTL1_CIM_ALWAYS = 0,
    #[doc = "1: Once"]
    ADC_DCCTL1_CIM_ONCE = 1,
    #[doc = "2: Hysteresis Always"]
    ADC_DCCTL1_CIM_HALWAYS = 2,
    #[doc = "3: Hysteresis Once"]
    ADC_DCCTL1_CIM_HONCE = 3,
}
impl From<ADC_DCCTL1_CIM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_DCCTL1_CIM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_DCCTL1_CIM_A {
    type Ux = u8;
}
impl ADC_DCCTL1_CIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_DCCTL1_CIM_A {
        match self.bits {
            0 => ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_ALWAYS,
            1 => ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_ONCE,
            2 => ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_HALWAYS,
            3 => ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Always"]
    #[inline(always)]
    pub fn is_adc_dcctl1_cim_always(&self) -> bool {
        *self == ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_ALWAYS
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn is_adc_dcctl1_cim_once(&self) -> bool {
        *self == ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_ONCE
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn is_adc_dcctl1_cim_halways(&self) -> bool {
        *self == ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_HALWAYS
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn is_adc_dcctl1_cim_honce(&self) -> bool {
        *self == ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_HONCE
    }
}
#[doc = "Field `ADC_DCCTL1_CIM` writer - Comparison Interrupt Mode"]
pub type ADC_DCCTL1_CIM_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, ADC_DCCTL1_CIM_A>;
impl<'a, REG, const O: u8> ADC_DCCTL1_CIM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Always"]
    #[inline(always)]
    pub fn adc_dcctl1_cim_always(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_ALWAYS)
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn adc_dcctl1_cim_once(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn adc_dcctl1_cim_halways(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn adc_dcctl1_cim_honce(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CIM_A::ADC_DCCTL1_CIM_HONCE)
    }
}
#[doc = "Field `ADC_DCCTL1_CIC` reader - Comparison Interrupt Condition"]
pub type ADC_DCCTL1_CIC_R = crate::FieldReader<ADC_DCCTL1_CIC_A>;
#[doc = "Comparison Interrupt Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_DCCTL1_CIC_A {
    #[doc = "0: Low Band"]
    ADC_DCCTL1_CIC_LOW = 0,
    #[doc = "1: Mid Band"]
    ADC_DCCTL1_CIC_MID = 1,
    #[doc = "3: High Band"]
    ADC_DCCTL1_CIC_HIGH = 3,
}
impl From<ADC_DCCTL1_CIC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_DCCTL1_CIC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_DCCTL1_CIC_A {
    type Ux = u8;
}
impl ADC_DCCTL1_CIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_DCCTL1_CIC_A> {
        match self.bits {
            0 => Some(ADC_DCCTL1_CIC_A::ADC_DCCTL1_CIC_LOW),
            1 => Some(ADC_DCCTL1_CIC_A::ADC_DCCTL1_CIC_MID),
            3 => Some(ADC_DCCTL1_CIC_A::ADC_DCCTL1_CIC_HIGH),
            _ => None,
        }
    }
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn is_adc_dcctl1_cic_low(&self) -> bool {
        *self == ADC_DCCTL1_CIC_A::ADC_DCCTL1_CIC_LOW
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn is_adc_dcctl1_cic_mid(&self) -> bool {
        *self == ADC_DCCTL1_CIC_A::ADC_DCCTL1_CIC_MID
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn is_adc_dcctl1_cic_high(&self) -> bool {
        *self == ADC_DCCTL1_CIC_A::ADC_DCCTL1_CIC_HIGH
    }
}
#[doc = "Field `ADC_DCCTL1_CIC` writer - Comparison Interrupt Condition"]
pub type ADC_DCCTL1_CIC_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, ADC_DCCTL1_CIC_A>;
impl<'a, REG, const O: u8> ADC_DCCTL1_CIC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn adc_dcctl1_cic_low(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CIC_A::ADC_DCCTL1_CIC_LOW)
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn adc_dcctl1_cic_mid(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CIC_A::ADC_DCCTL1_CIC_MID)
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn adc_dcctl1_cic_high(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CIC_A::ADC_DCCTL1_CIC_HIGH)
    }
}
#[doc = "Field `ADC_DCCTL1_CIE` reader - Comparison Interrupt Enable"]
pub type ADC_DCCTL1_CIE_R = crate::BitReader;
#[doc = "Field `ADC_DCCTL1_CIE` writer - Comparison Interrupt Enable"]
pub type ADC_DCCTL1_CIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCCTL1_CTM` reader - Comparison Trigger Mode"]
pub type ADC_DCCTL1_CTM_R = crate::FieldReader<ADC_DCCTL1_CTM_A>;
#[doc = "Comparison Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_DCCTL1_CTM_A {
    #[doc = "0: Always"]
    ADC_DCCTL1_CTM_ALWAYS = 0,
    #[doc = "1: Once"]
    ADC_DCCTL1_CTM_ONCE = 1,
    #[doc = "2: Hysteresis Always"]
    ADC_DCCTL1_CTM_HALWAYS = 2,
    #[doc = "3: Hysteresis Once"]
    ADC_DCCTL1_CTM_HONCE = 3,
}
impl From<ADC_DCCTL1_CTM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_DCCTL1_CTM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_DCCTL1_CTM_A {
    type Ux = u8;
}
impl ADC_DCCTL1_CTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_DCCTL1_CTM_A {
        match self.bits {
            0 => ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_ALWAYS,
            1 => ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_ONCE,
            2 => ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_HALWAYS,
            3 => ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Always"]
    #[inline(always)]
    pub fn is_adc_dcctl1_ctm_always(&self) -> bool {
        *self == ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_ALWAYS
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn is_adc_dcctl1_ctm_once(&self) -> bool {
        *self == ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_ONCE
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn is_adc_dcctl1_ctm_halways(&self) -> bool {
        *self == ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_HALWAYS
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn is_adc_dcctl1_ctm_honce(&self) -> bool {
        *self == ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_HONCE
    }
}
#[doc = "Field `ADC_DCCTL1_CTM` writer - Comparison Trigger Mode"]
pub type ADC_DCCTL1_CTM_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, ADC_DCCTL1_CTM_A>;
impl<'a, REG, const O: u8> ADC_DCCTL1_CTM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Always"]
    #[inline(always)]
    pub fn adc_dcctl1_ctm_always(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_ALWAYS)
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn adc_dcctl1_ctm_once(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn adc_dcctl1_ctm_halways(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn adc_dcctl1_ctm_honce(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CTM_A::ADC_DCCTL1_CTM_HONCE)
    }
}
#[doc = "Field `ADC_DCCTL1_CTC` reader - Comparison Trigger Condition"]
pub type ADC_DCCTL1_CTC_R = crate::FieldReader<ADC_DCCTL1_CTC_A>;
#[doc = "Comparison Trigger Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_DCCTL1_CTC_A {
    #[doc = "0: Low Band"]
    ADC_DCCTL1_CTC_LOW = 0,
    #[doc = "1: Mid Band"]
    ADC_DCCTL1_CTC_MID = 1,
    #[doc = "3: High Band"]
    ADC_DCCTL1_CTC_HIGH = 3,
}
impl From<ADC_DCCTL1_CTC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_DCCTL1_CTC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_DCCTL1_CTC_A {
    type Ux = u8;
}
impl ADC_DCCTL1_CTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_DCCTL1_CTC_A> {
        match self.bits {
            0 => Some(ADC_DCCTL1_CTC_A::ADC_DCCTL1_CTC_LOW),
            1 => Some(ADC_DCCTL1_CTC_A::ADC_DCCTL1_CTC_MID),
            3 => Some(ADC_DCCTL1_CTC_A::ADC_DCCTL1_CTC_HIGH),
            _ => None,
        }
    }
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn is_adc_dcctl1_ctc_low(&self) -> bool {
        *self == ADC_DCCTL1_CTC_A::ADC_DCCTL1_CTC_LOW
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn is_adc_dcctl1_ctc_mid(&self) -> bool {
        *self == ADC_DCCTL1_CTC_A::ADC_DCCTL1_CTC_MID
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn is_adc_dcctl1_ctc_high(&self) -> bool {
        *self == ADC_DCCTL1_CTC_A::ADC_DCCTL1_CTC_HIGH
    }
}
#[doc = "Field `ADC_DCCTL1_CTC` writer - Comparison Trigger Condition"]
pub type ADC_DCCTL1_CTC_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, ADC_DCCTL1_CTC_A>;
impl<'a, REG, const O: u8> ADC_DCCTL1_CTC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn adc_dcctl1_ctc_low(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CTC_A::ADC_DCCTL1_CTC_LOW)
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn adc_dcctl1_ctc_mid(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CTC_A::ADC_DCCTL1_CTC_MID)
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn adc_dcctl1_ctc_high(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DCCTL1_CTC_A::ADC_DCCTL1_CTC_HIGH)
    }
}
#[doc = "Field `ADC_DCCTL1_CTE` reader - Comparison Trigger Enable"]
pub type ADC_DCCTL1_CTE_R = crate::BitReader;
#[doc = "Field `ADC_DCCTL1_CTE` writer - Comparison Trigger Enable"]
pub type ADC_DCCTL1_CTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline(always)]
    pub fn adc_dcctl1_cim(&self) -> ADC_DCCTL1_CIM_R {
        ADC_DCCTL1_CIM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline(always)]
    pub fn adc_dcctl1_cic(&self) -> ADC_DCCTL1_CIC_R {
        ADC_DCCTL1_CIC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline(always)]
    pub fn adc_dcctl1_cie(&self) -> ADC_DCCTL1_CIE_R {
        ADC_DCCTL1_CIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline(always)]
    pub fn adc_dcctl1_ctm(&self) -> ADC_DCCTL1_CTM_R {
        ADC_DCCTL1_CTM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline(always)]
    pub fn adc_dcctl1_ctc(&self) -> ADC_DCCTL1_CTC_R {
        ADC_DCCTL1_CTC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline(always)]
    pub fn adc_dcctl1_cte(&self) -> ADC_DCCTL1_CTE_R {
        ADC_DCCTL1_CTE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcctl1_cim(&mut self) -> ADC_DCCTL1_CIM_W<DCCTL1_SPEC, 0> {
        ADC_DCCTL1_CIM_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcctl1_cic(&mut self) -> ADC_DCCTL1_CIC_W<DCCTL1_SPEC, 2> {
        ADC_DCCTL1_CIC_W::new(self)
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcctl1_cie(&mut self) -> ADC_DCCTL1_CIE_W<DCCTL1_SPEC, 4> {
        ADC_DCCTL1_CIE_W::new(self)
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcctl1_ctm(&mut self) -> ADC_DCCTL1_CTM_W<DCCTL1_SPEC, 8> {
        ADC_DCCTL1_CTM_W::new(self)
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcctl1_ctc(&mut self) -> ADC_DCCTL1_CTC_W<DCCTL1_SPEC, 10> {
        ADC_DCCTL1_CTC_W::new(self)
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcctl1_cte(&mut self) -> ADC_DCCTL1_CTE_W<DCCTL1_SPEC, 12> {
        ADC_DCCTL1_CTE_W::new(self)
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
#[doc = "ADC Digital Comparator Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCCTL1_SPEC;
impl crate::RegisterSpec for DCCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcctl1::R`](R) reader structure"]
impl crate::Readable for DCCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcctl1::W`](W) writer structure"]
impl crate::Writable for DCCTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCCTL1 to value 0"]
impl crate::Resettable for DCCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
