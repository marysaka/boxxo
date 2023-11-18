#[doc = "Register `TSSEL` reader"]
pub type R = crate::R<TSSEL_SPEC>;
#[doc = "Register `TSSEL` writer"]
pub type W = crate::W<TSSEL_SPEC>;
#[doc = "Field `ADC_TSSEL_PS0` reader - PWM Unit Select"]
pub type ADC_TSSEL_PS0_R = crate::FieldReader<ADC_TSSEL_PS0_A>;
#[doc = "PWM Unit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_TSSEL_PS0_A {
    #[doc = "0: PWM Unit 0"]
    ADC_TSSEL_PS0_0 = 0,
    #[doc = "1: PWM Unit 1"]
    ADC_TSSEL_PS0_1 = 1,
}
impl From<ADC_TSSEL_PS0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_TSSEL_PS0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_TSSEL_PS0_A {
    type Ux = u8;
}
impl ADC_TSSEL_PS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_TSSEL_PS0_A> {
        match self.bits {
            0 => Some(ADC_TSSEL_PS0_A::ADC_TSSEL_PS0_0),
            1 => Some(ADC_TSSEL_PS0_A::ADC_TSSEL_PS0_1),
            _ => None,
        }
    }
    #[doc = "PWM Unit 0"]
    #[inline(always)]
    pub fn is_adc_tssel_ps0_0(&self) -> bool {
        *self == ADC_TSSEL_PS0_A::ADC_TSSEL_PS0_0
    }
    #[doc = "PWM Unit 1"]
    #[inline(always)]
    pub fn is_adc_tssel_ps0_1(&self) -> bool {
        *self == ADC_TSSEL_PS0_A::ADC_TSSEL_PS0_1
    }
}
#[doc = "Field `ADC_TSSEL_PS0` writer - PWM Unit Select"]
pub type ADC_TSSEL_PS0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ADC_TSSEL_PS0_A>;
impl<'a, REG, const O: u8> ADC_TSSEL_PS0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWM Unit 0"]
    #[inline(always)]
    pub fn adc_tssel_ps0_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_TSSEL_PS0_A::ADC_TSSEL_PS0_0)
    }
    #[doc = "PWM Unit 1"]
    #[inline(always)]
    pub fn adc_tssel_ps0_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_TSSEL_PS0_A::ADC_TSSEL_PS0_1)
    }
}
#[doc = "Field `ADC_TSSEL_PS1` reader - PWM Unit Select"]
pub type ADC_TSSEL_PS1_R = crate::FieldReader<ADC_TSSEL_PS1_A>;
#[doc = "PWM Unit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_TSSEL_PS1_A {
    #[doc = "0: PWM Unit 0"]
    ADC_TSSEL_PS1_0 = 0,
    #[doc = "1: PWM Unit 1"]
    ADC_TSSEL_PS1_1 = 1,
}
impl From<ADC_TSSEL_PS1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_TSSEL_PS1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_TSSEL_PS1_A {
    type Ux = u8;
}
impl ADC_TSSEL_PS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_TSSEL_PS1_A> {
        match self.bits {
            0 => Some(ADC_TSSEL_PS1_A::ADC_TSSEL_PS1_0),
            1 => Some(ADC_TSSEL_PS1_A::ADC_TSSEL_PS1_1),
            _ => None,
        }
    }
    #[doc = "PWM Unit 0"]
    #[inline(always)]
    pub fn is_adc_tssel_ps1_0(&self) -> bool {
        *self == ADC_TSSEL_PS1_A::ADC_TSSEL_PS1_0
    }
    #[doc = "PWM Unit 1"]
    #[inline(always)]
    pub fn is_adc_tssel_ps1_1(&self) -> bool {
        *self == ADC_TSSEL_PS1_A::ADC_TSSEL_PS1_1
    }
}
#[doc = "Field `ADC_TSSEL_PS1` writer - PWM Unit Select"]
pub type ADC_TSSEL_PS1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ADC_TSSEL_PS1_A>;
impl<'a, REG, const O: u8> ADC_TSSEL_PS1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWM Unit 0"]
    #[inline(always)]
    pub fn adc_tssel_ps1_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_TSSEL_PS1_A::ADC_TSSEL_PS1_0)
    }
    #[doc = "PWM Unit 1"]
    #[inline(always)]
    pub fn adc_tssel_ps1_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_TSSEL_PS1_A::ADC_TSSEL_PS1_1)
    }
}
#[doc = "Field `ADC_TSSEL_PS2` reader - PWM Unit Select"]
pub type ADC_TSSEL_PS2_R = crate::FieldReader<ADC_TSSEL_PS2_A>;
#[doc = "PWM Unit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_TSSEL_PS2_A {
    #[doc = "0: PWM Unit 0"]
    ADC_TSSEL_PS2_0 = 0,
    #[doc = "1: PWM Unit 1"]
    ADC_TSSEL_PS2_1 = 1,
}
impl From<ADC_TSSEL_PS2_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_TSSEL_PS2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_TSSEL_PS2_A {
    type Ux = u8;
}
impl ADC_TSSEL_PS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_TSSEL_PS2_A> {
        match self.bits {
            0 => Some(ADC_TSSEL_PS2_A::ADC_TSSEL_PS2_0),
            1 => Some(ADC_TSSEL_PS2_A::ADC_TSSEL_PS2_1),
            _ => None,
        }
    }
    #[doc = "PWM Unit 0"]
    #[inline(always)]
    pub fn is_adc_tssel_ps2_0(&self) -> bool {
        *self == ADC_TSSEL_PS2_A::ADC_TSSEL_PS2_0
    }
    #[doc = "PWM Unit 1"]
    #[inline(always)]
    pub fn is_adc_tssel_ps2_1(&self) -> bool {
        *self == ADC_TSSEL_PS2_A::ADC_TSSEL_PS2_1
    }
}
#[doc = "Field `ADC_TSSEL_PS2` writer - PWM Unit Select"]
pub type ADC_TSSEL_PS2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ADC_TSSEL_PS2_A>;
impl<'a, REG, const O: u8> ADC_TSSEL_PS2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWM Unit 0"]
    #[inline(always)]
    pub fn adc_tssel_ps2_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_TSSEL_PS2_A::ADC_TSSEL_PS2_0)
    }
    #[doc = "PWM Unit 1"]
    #[inline(always)]
    pub fn adc_tssel_ps2_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_TSSEL_PS2_A::ADC_TSSEL_PS2_1)
    }
}
#[doc = "Field `ADC_TSSEL_PS3` reader - PWM Unit Select"]
pub type ADC_TSSEL_PS3_R = crate::FieldReader<ADC_TSSEL_PS3_A>;
#[doc = "PWM Unit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_TSSEL_PS3_A {
    #[doc = "0: PWM Unit 0"]
    ADC_TSSEL_PS3_0 = 0,
    #[doc = "1: PWM Unit 1"]
    ADC_TSSEL_PS3_1 = 1,
}
impl From<ADC_TSSEL_PS3_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_TSSEL_PS3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_TSSEL_PS3_A {
    type Ux = u8;
}
impl ADC_TSSEL_PS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_TSSEL_PS3_A> {
        match self.bits {
            0 => Some(ADC_TSSEL_PS3_A::ADC_TSSEL_PS3_0),
            1 => Some(ADC_TSSEL_PS3_A::ADC_TSSEL_PS3_1),
            _ => None,
        }
    }
    #[doc = "PWM Unit 0"]
    #[inline(always)]
    pub fn is_adc_tssel_ps3_0(&self) -> bool {
        *self == ADC_TSSEL_PS3_A::ADC_TSSEL_PS3_0
    }
    #[doc = "PWM Unit 1"]
    #[inline(always)]
    pub fn is_adc_tssel_ps3_1(&self) -> bool {
        *self == ADC_TSSEL_PS3_A::ADC_TSSEL_PS3_1
    }
}
#[doc = "Field `ADC_TSSEL_PS3` writer - PWM Unit Select"]
pub type ADC_TSSEL_PS3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ADC_TSSEL_PS3_A>;
impl<'a, REG, const O: u8> ADC_TSSEL_PS3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWM Unit 0"]
    #[inline(always)]
    pub fn adc_tssel_ps3_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_TSSEL_PS3_A::ADC_TSSEL_PS3_0)
    }
    #[doc = "PWM Unit 1"]
    #[inline(always)]
    pub fn adc_tssel_ps3_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_TSSEL_PS3_A::ADC_TSSEL_PS3_1)
    }
}
impl R {
    #[doc = "Bits 4:5 - PWM Unit Select"]
    #[inline(always)]
    pub fn adc_tssel_ps0(&self) -> ADC_TSSEL_PS0_R {
        ADC_TSSEL_PS0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PWM Unit Select"]
    #[inline(always)]
    pub fn adc_tssel_ps1(&self) -> ADC_TSSEL_PS1_R {
        ADC_TSSEL_PS1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PWM Unit Select"]
    #[inline(always)]
    pub fn adc_tssel_ps2(&self) -> ADC_TSSEL_PS2_R {
        ADC_TSSEL_PS2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PWM Unit Select"]
    #[inline(always)]
    pub fn adc_tssel_ps3(&self) -> ADC_TSSEL_PS3_R {
        ADC_TSSEL_PS3_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - PWM Unit Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_tssel_ps0(&mut self) -> ADC_TSSEL_PS0_W<TSSEL_SPEC, 4> {
        ADC_TSSEL_PS0_W::new(self)
    }
    #[doc = "Bits 12:13 - PWM Unit Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_tssel_ps1(&mut self) -> ADC_TSSEL_PS1_W<TSSEL_SPEC, 12> {
        ADC_TSSEL_PS1_W::new(self)
    }
    #[doc = "Bits 20:21 - PWM Unit Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_tssel_ps2(&mut self) -> ADC_TSSEL_PS2_W<TSSEL_SPEC, 20> {
        ADC_TSSEL_PS2_W::new(self)
    }
    #[doc = "Bits 28:29 - PWM Unit Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_tssel_ps3(&mut self) -> ADC_TSSEL_PS3_W<TSSEL_SPEC, 28> {
        ADC_TSSEL_PS3_W::new(self)
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
#[doc = "ADC Trigger Source Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSSEL_SPEC;
impl crate::RegisterSpec for TSSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tssel::R`](R) reader structure"]
impl crate::Readable for TSSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tssel::W`](W) writer structure"]
impl crate::Writable for TSSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSSEL to value 0"]
impl crate::Resettable for TSSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
