#[doc = "Register `SPC` reader"]
pub type R = crate::R<SPC_SPEC>;
#[doc = "Register `SPC` writer"]
pub type W = crate::W<SPC_SPEC>;
#[doc = "Field `ADC_SPC_PHASE` reader - Phase Difference"]
pub type ADC_SPC_PHASE_R = crate::FieldReader<ADC_SPC_PHASE_A>;
#[doc = "Phase Difference\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_SPC_PHASE_A {
    #[doc = "0: ADC sample lags by 0.0"]
    ADC_SPC_PHASE_0 = 0,
    #[doc = "1: ADC sample lags by 22.5"]
    ADC_SPC_PHASE_22_5 = 1,
    #[doc = "2: ADC sample lags by 45.0"]
    ADC_SPC_PHASE_45 = 2,
    #[doc = "3: ADC sample lags by 67.5"]
    ADC_SPC_PHASE_67_5 = 3,
    #[doc = "4: ADC sample lags by 90.0"]
    ADC_SPC_PHASE_90 = 4,
    #[doc = "5: ADC sample lags by 112.5"]
    ADC_SPC_PHASE_112_5 = 5,
    #[doc = "6: ADC sample lags by 135.0"]
    ADC_SPC_PHASE_135 = 6,
    #[doc = "7: ADC sample lags by 157.5"]
    ADC_SPC_PHASE_157_5 = 7,
    #[doc = "8: ADC sample lags by 180.0"]
    ADC_SPC_PHASE_180 = 8,
    #[doc = "9: ADC sample lags by 202.5"]
    ADC_SPC_PHASE_202_5 = 9,
    #[doc = "10: ADC sample lags by 225.0"]
    ADC_SPC_PHASE_225 = 10,
    #[doc = "11: ADC sample lags by 247.5"]
    ADC_SPC_PHASE_247_5 = 11,
    #[doc = "12: ADC sample lags by 270.0"]
    ADC_SPC_PHASE_270 = 12,
    #[doc = "13: ADC sample lags by 292.5"]
    ADC_SPC_PHASE_292_5 = 13,
    #[doc = "14: ADC sample lags by 315.0"]
    ADC_SPC_PHASE_315 = 14,
    #[doc = "15: ADC sample lags by 337.5"]
    ADC_SPC_PHASE_337_5 = 15,
}
impl From<ADC_SPC_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SPC_PHASE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_SPC_PHASE_A {
    type Ux = u8;
}
impl ADC_SPC_PHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_SPC_PHASE_A {
        match self.bits {
            0 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_0,
            1 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_22_5,
            2 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_45,
            3 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_67_5,
            4 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_90,
            5 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_112_5,
            6 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_135,
            7 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_157_5,
            8 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_180,
            9 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_202_5,
            10 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_225,
            11 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_247_5,
            12 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_270,
            13 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_292_5,
            14 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_315,
            15 => ADC_SPC_PHASE_A::ADC_SPC_PHASE_337_5,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC sample lags by 0.0"]
    #[inline(always)]
    pub fn is_adc_spc_phase_0(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_0
    }
    #[doc = "ADC sample lags by 22.5"]
    #[inline(always)]
    pub fn is_adc_spc_phase_22_5(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_22_5
    }
    #[doc = "ADC sample lags by 45.0"]
    #[inline(always)]
    pub fn is_adc_spc_phase_45(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_45
    }
    #[doc = "ADC sample lags by 67.5"]
    #[inline(always)]
    pub fn is_adc_spc_phase_67_5(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_67_5
    }
    #[doc = "ADC sample lags by 90.0"]
    #[inline(always)]
    pub fn is_adc_spc_phase_90(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_90
    }
    #[doc = "ADC sample lags by 112.5"]
    #[inline(always)]
    pub fn is_adc_spc_phase_112_5(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_112_5
    }
    #[doc = "ADC sample lags by 135.0"]
    #[inline(always)]
    pub fn is_adc_spc_phase_135(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_135
    }
    #[doc = "ADC sample lags by 157.5"]
    #[inline(always)]
    pub fn is_adc_spc_phase_157_5(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_157_5
    }
    #[doc = "ADC sample lags by 180.0"]
    #[inline(always)]
    pub fn is_adc_spc_phase_180(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_180
    }
    #[doc = "ADC sample lags by 202.5"]
    #[inline(always)]
    pub fn is_adc_spc_phase_202_5(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_202_5
    }
    #[doc = "ADC sample lags by 225.0"]
    #[inline(always)]
    pub fn is_adc_spc_phase_225(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_225
    }
    #[doc = "ADC sample lags by 247.5"]
    #[inline(always)]
    pub fn is_adc_spc_phase_247_5(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_247_5
    }
    #[doc = "ADC sample lags by 270.0"]
    #[inline(always)]
    pub fn is_adc_spc_phase_270(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_270
    }
    #[doc = "ADC sample lags by 292.5"]
    #[inline(always)]
    pub fn is_adc_spc_phase_292_5(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_292_5
    }
    #[doc = "ADC sample lags by 315.0"]
    #[inline(always)]
    pub fn is_adc_spc_phase_315(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_315
    }
    #[doc = "ADC sample lags by 337.5"]
    #[inline(always)]
    pub fn is_adc_spc_phase_337_5(&self) -> bool {
        *self == ADC_SPC_PHASE_A::ADC_SPC_PHASE_337_5
    }
}
#[doc = "Field `ADC_SPC_PHASE` writer - Phase Difference"]
pub type ADC_SPC_PHASE_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 4, O, ADC_SPC_PHASE_A>;
impl<'a, REG, const O: u8> ADC_SPC_PHASE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC sample lags by 0.0"]
    #[inline(always)]
    pub fn adc_spc_phase_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_0)
    }
    #[doc = "ADC sample lags by 22.5"]
    #[inline(always)]
    pub fn adc_spc_phase_22_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_22_5)
    }
    #[doc = "ADC sample lags by 45.0"]
    #[inline(always)]
    pub fn adc_spc_phase_45(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_45)
    }
    #[doc = "ADC sample lags by 67.5"]
    #[inline(always)]
    pub fn adc_spc_phase_67_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_67_5)
    }
    #[doc = "ADC sample lags by 90.0"]
    #[inline(always)]
    pub fn adc_spc_phase_90(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_90)
    }
    #[doc = "ADC sample lags by 112.5"]
    #[inline(always)]
    pub fn adc_spc_phase_112_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_112_5)
    }
    #[doc = "ADC sample lags by 135.0"]
    #[inline(always)]
    pub fn adc_spc_phase_135(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_135)
    }
    #[doc = "ADC sample lags by 157.5"]
    #[inline(always)]
    pub fn adc_spc_phase_157_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_157_5)
    }
    #[doc = "ADC sample lags by 180.0"]
    #[inline(always)]
    pub fn adc_spc_phase_180(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_180)
    }
    #[doc = "ADC sample lags by 202.5"]
    #[inline(always)]
    pub fn adc_spc_phase_202_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_202_5)
    }
    #[doc = "ADC sample lags by 225.0"]
    #[inline(always)]
    pub fn adc_spc_phase_225(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_225)
    }
    #[doc = "ADC sample lags by 247.5"]
    #[inline(always)]
    pub fn adc_spc_phase_247_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_247_5)
    }
    #[doc = "ADC sample lags by 270.0"]
    #[inline(always)]
    pub fn adc_spc_phase_270(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_270)
    }
    #[doc = "ADC sample lags by 292.5"]
    #[inline(always)]
    pub fn adc_spc_phase_292_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_292_5)
    }
    #[doc = "ADC sample lags by 315.0"]
    #[inline(always)]
    pub fn adc_spc_phase_315(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_315)
    }
    #[doc = "ADC sample lags by 337.5"]
    #[inline(always)]
    pub fn adc_spc_phase_337_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_SPC_PHASE_A::ADC_SPC_PHASE_337_5)
    }
}
impl R {
    #[doc = "Bits 0:3 - Phase Difference"]
    #[inline(always)]
    pub fn adc_spc_phase(&self) -> ADC_SPC_PHASE_R {
        ADC_SPC_PHASE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Phase Difference"]
    #[inline(always)]
    #[must_use]
    pub fn adc_spc_phase(&mut self) -> ADC_SPC_PHASE_W<SPC_SPEC, 0> {
        ADC_SPC_PHASE_W::new(self)
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
#[doc = "ADC Sample Phase Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPC_SPEC;
impl crate::RegisterSpec for SPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spc::R`](R) reader structure"]
impl crate::Readable for SPC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spc::W`](W) writer structure"]
impl crate::Writable for SPC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPC to value 0"]
impl crate::Resettable for SPC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
