#[doc = "Register `EMUX` reader"]
pub type R = crate::R<EMUX_SPEC>;
#[doc = "Register `EMUX` writer"]
pub type W = crate::W<EMUX_SPEC>;
#[doc = "Field `ADC_EMUX_EM0` reader - SS0 Trigger Select"]
pub type ADC_EMUX_EM0_R = crate::FieldReader<ADC_EMUX_EM0_A>;
#[doc = "SS0 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_EMUX_EM0_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM0_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM0_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM0_COMP1 = 2,
    #[doc = "4: External (GPIO PB4)"]
    ADC_EMUX_EM0_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM0_TIMER = 5,
    #[doc = "6: PWM0"]
    ADC_EMUX_EM0_PWM0 = 6,
    #[doc = "7: PWM1"]
    ADC_EMUX_EM0_PWM1 = 7,
    #[doc = "8: PWM2"]
    ADC_EMUX_EM0_PWM2 = 8,
    #[doc = "9: PWM3"]
    ADC_EMUX_EM0_PWM3 = 9,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM0_ALWAYS = 15,
}
impl From<ADC_EMUX_EM0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_EMUX_EM0_A {
    type Ux = u8;
}
impl ADC_EMUX_EM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_EMUX_EM0_A> {
        match self.bits {
            0 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PROCESSOR),
            1 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP0),
            2 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP1),
            4 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_EXTERNAL),
            5 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_TIMER),
            6 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM0),
            7 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM1),
            8 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM2),
            9 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM3),
            15 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_ALWAYS),
            _ => None,
        }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn is_adc_emux_em0_processor(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PROCESSOR
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp0(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp1(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP1
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn is_adc_emux_em0_external(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_EXTERNAL
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn is_adc_emux_em0_timer(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_TIMER
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM0
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM1
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM2
    }
    #[doc = "PWM3"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM3
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn is_adc_emux_em0_always(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_ALWAYS
    }
}
#[doc = "Field `ADC_EMUX_EM0` writer - SS0 Trigger Select"]
pub type ADC_EMUX_EM0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADC_EMUX_EM0_A>;
impl<'a, REG, const O: u8> ADC_EMUX_EM0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em0_processor(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em0_comp0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em0_comp1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP1)
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn adc_emux_em0_external(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em0_timer(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_TIMER)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM0)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM1)
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM2)
    }
    #[doc = "PWM3"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em0_always(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_ALWAYS)
    }
}
#[doc = "Field `ADC_EMUX_EM1` reader - SS1 Trigger Select"]
pub type ADC_EMUX_EM1_R = crate::FieldReader<ADC_EMUX_EM1_A>;
#[doc = "SS1 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_EMUX_EM1_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM1_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM1_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM1_COMP1 = 2,
    #[doc = "4: External (GPIO PB4)"]
    ADC_EMUX_EM1_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM1_TIMER = 5,
    #[doc = "6: PWM0"]
    ADC_EMUX_EM1_PWM0 = 6,
    #[doc = "7: PWM1"]
    ADC_EMUX_EM1_PWM1 = 7,
    #[doc = "8: PWM2"]
    ADC_EMUX_EM1_PWM2 = 8,
    #[doc = "9: PWM3"]
    ADC_EMUX_EM1_PWM3 = 9,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM1_ALWAYS = 15,
}
impl From<ADC_EMUX_EM1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_EMUX_EM1_A {
    type Ux = u8;
}
impl ADC_EMUX_EM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_EMUX_EM1_A> {
        match self.bits {
            0 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PROCESSOR),
            1 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP0),
            2 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP1),
            4 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_EXTERNAL),
            5 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_TIMER),
            6 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM0),
            7 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM1),
            8 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM2),
            9 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM3),
            15 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_ALWAYS),
            _ => None,
        }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn is_adc_emux_em1_processor(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PROCESSOR
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp0(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp1(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP1
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn is_adc_emux_em1_external(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_EXTERNAL
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn is_adc_emux_em1_timer(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_TIMER
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM0
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM1
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM2
    }
    #[doc = "PWM3"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM3
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn is_adc_emux_em1_always(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_ALWAYS
    }
}
#[doc = "Field `ADC_EMUX_EM1` writer - SS1 Trigger Select"]
pub type ADC_EMUX_EM1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADC_EMUX_EM1_A>;
impl<'a, REG, const O: u8> ADC_EMUX_EM1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em1_processor(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em1_comp0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em1_comp1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP1)
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn adc_emux_em1_external(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em1_timer(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_TIMER)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM0)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM1)
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM2)
    }
    #[doc = "PWM3"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em1_always(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_ALWAYS)
    }
}
#[doc = "Field `ADC_EMUX_EM2` reader - SS2 Trigger Select"]
pub type ADC_EMUX_EM2_R = crate::FieldReader<ADC_EMUX_EM2_A>;
#[doc = "SS2 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_EMUX_EM2_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM2_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM2_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM2_COMP1 = 2,
    #[doc = "4: External (GPIO PB4)"]
    ADC_EMUX_EM2_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM2_TIMER = 5,
    #[doc = "6: PWM0"]
    ADC_EMUX_EM2_PWM0 = 6,
    #[doc = "7: PWM1"]
    ADC_EMUX_EM2_PWM1 = 7,
    #[doc = "8: PWM2"]
    ADC_EMUX_EM2_PWM2 = 8,
    #[doc = "9: PWM3"]
    ADC_EMUX_EM2_PWM3 = 9,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM2_ALWAYS = 15,
}
impl From<ADC_EMUX_EM2_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_EMUX_EM2_A {
    type Ux = u8;
}
impl ADC_EMUX_EM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_EMUX_EM2_A> {
        match self.bits {
            0 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PROCESSOR),
            1 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP0),
            2 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP1),
            4 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_EXTERNAL),
            5 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_TIMER),
            6 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM0),
            7 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM1),
            8 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM2),
            9 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM3),
            15 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_ALWAYS),
            _ => None,
        }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn is_adc_emux_em2_processor(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PROCESSOR
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp0(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp1(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP1
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn is_adc_emux_em2_external(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_EXTERNAL
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn is_adc_emux_em2_timer(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_TIMER
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM0
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM1
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM2
    }
    #[doc = "PWM3"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM3
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn is_adc_emux_em2_always(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_ALWAYS
    }
}
#[doc = "Field `ADC_EMUX_EM2` writer - SS2 Trigger Select"]
pub type ADC_EMUX_EM2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADC_EMUX_EM2_A>;
impl<'a, REG, const O: u8> ADC_EMUX_EM2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em2_processor(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em2_comp0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em2_comp1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP1)
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn adc_emux_em2_external(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em2_timer(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_TIMER)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM0)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM1)
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM2)
    }
    #[doc = "PWM3"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em2_always(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_ALWAYS)
    }
}
#[doc = "Field `ADC_EMUX_EM3` reader - SS3 Trigger Select"]
pub type ADC_EMUX_EM3_R = crate::FieldReader<ADC_EMUX_EM3_A>;
#[doc = "SS3 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_EMUX_EM3_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM3_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM3_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM3_COMP1 = 2,
    #[doc = "4: External (GPIO PB4)"]
    ADC_EMUX_EM3_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM3_TIMER = 5,
    #[doc = "6: PWM0"]
    ADC_EMUX_EM3_PWM0 = 6,
    #[doc = "7: PWM1"]
    ADC_EMUX_EM3_PWM1 = 7,
    #[doc = "8: PWM2"]
    ADC_EMUX_EM3_PWM2 = 8,
    #[doc = "9: PWM3"]
    ADC_EMUX_EM3_PWM3 = 9,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM3_ALWAYS = 15,
}
impl From<ADC_EMUX_EM3_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_EMUX_EM3_A {
    type Ux = u8;
}
impl ADC_EMUX_EM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC_EMUX_EM3_A> {
        match self.bits {
            0 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PROCESSOR),
            1 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP0),
            2 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP1),
            4 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_EXTERNAL),
            5 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_TIMER),
            6 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM0),
            7 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM1),
            8 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM2),
            9 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM3),
            15 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_ALWAYS),
            _ => None,
        }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn is_adc_emux_em3_processor(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PROCESSOR
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp0(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp1(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP1
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn is_adc_emux_em3_external(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_EXTERNAL
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn is_adc_emux_em3_timer(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_TIMER
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM0
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM1
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM2
    }
    #[doc = "PWM3"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM3
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn is_adc_emux_em3_always(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_ALWAYS
    }
}
#[doc = "Field `ADC_EMUX_EM3` writer - SS3 Trigger Select"]
pub type ADC_EMUX_EM3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADC_EMUX_EM3_A>;
impl<'a, REG, const O: u8> ADC_EMUX_EM3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em3_processor(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em3_comp0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em3_comp1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP1)
    }
    #[doc = "External (GPIO PB4)"]
    #[inline(always)]
    pub fn adc_emux_em3_external(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em3_timer(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_TIMER)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM0)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM1)
    }
    #[doc = "PWM2"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM2)
    }
    #[doc = "PWM3"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM3)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em3_always(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_ALWAYS)
    }
}
impl R {
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em0(&self) -> ADC_EMUX_EM0_R {
        ADC_EMUX_EM0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em1(&self) -> ADC_EMUX_EM1_R {
        ADC_EMUX_EM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em2(&self) -> ADC_EMUX_EM2_R {
        ADC_EMUX_EM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em3(&self) -> ADC_EMUX_EM3_R {
        ADC_EMUX_EM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_emux_em0(&mut self) -> ADC_EMUX_EM0_W<EMUX_SPEC, 0> {
        ADC_EMUX_EM0_W::new(self)
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_emux_em1(&mut self) -> ADC_EMUX_EM1_W<EMUX_SPEC, 4> {
        ADC_EMUX_EM1_W::new(self)
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_emux_em2(&mut self) -> ADC_EMUX_EM2_W<EMUX_SPEC, 8> {
        ADC_EMUX_EM2_W::new(self)
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_emux_em3(&mut self) -> ADC_EMUX_EM3_W<EMUX_SPEC, 12> {
        ADC_EMUX_EM3_W::new(self)
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
#[doc = "ADC Event Multiplexer Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMUX_SPEC;
impl crate::RegisterSpec for EMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emux::R`](R) reader structure"]
impl crate::Readable for EMUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emux::W`](W) writer structure"]
impl crate::Writable for EMUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMUX to value 0"]
impl crate::Resettable for EMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
