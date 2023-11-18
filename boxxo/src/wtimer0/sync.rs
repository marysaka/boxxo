#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SYNC_SPEC>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SYNC_SPEC>;
#[doc = "Field `TIMER_SYNC_SYNCT0` reader - Synchronize GPTM 16/32-Bit Timer 0"]
pub type TIMER_SYNC_SYNCT0_R = crate::FieldReader<TIMER_SYNC_SYNCT0_A>;
#[doc = "Synchronize GPTM 16/32-Bit Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT0_A {
    #[doc = "0: GPTM 16/32-Bit Timer 0 is not affected"]
    TIMER_SYNC_SYNCT0_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 16/32-Bit Timer 0 is triggered"]
    TIMER_SYNC_SYNCT0_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 16/32-Bit Timer 0 is triggered"]
    TIMER_SYNC_SYNCT0_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 0 is triggered"]
    TIMER_SYNC_SYNCT0_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT0_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCT0_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCT0_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_NONE,
            1 => TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TA,
            2 => TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TB,
            3 => TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 16/32-Bit Timer 0 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT0` writer - Synchronize GPTM 16/32-Bit Timer 0"]
pub type TIMER_SYNC_SYNCT0_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCT0_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCT0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 16/32-Bit Timer 0 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct0_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct0_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct0_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct0_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT1` reader - Synchronize GPTM 16/32-Bit Timer 1"]
pub type TIMER_SYNC_SYNCT1_R = crate::FieldReader<TIMER_SYNC_SYNCT1_A>;
#[doc = "Synchronize GPTM 16/32-Bit Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT1_A {
    #[doc = "0: GPTM 16/32-Bit Timer 1 is not affected"]
    TIMER_SYNC_SYNCT1_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 16/32-Bit Timer 1 is triggered"]
    TIMER_SYNC_SYNCT1_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 16/32-Bit Timer 1 is triggered"]
    TIMER_SYNC_SYNCT1_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 1 is triggered"]
    TIMER_SYNC_SYNCT1_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT1_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCT1_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCT1_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_NONE,
            1 => TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TA,
            2 => TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TB,
            3 => TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 16/32-Bit Timer 1 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT1` writer - Synchronize GPTM 16/32-Bit Timer 1"]
pub type TIMER_SYNC_SYNCT1_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCT1_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCT1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 16/32-Bit Timer 1 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct1_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct1_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct1_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct1_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT2` reader - Synchronize GPTM 16/32-Bit Timer 2"]
pub type TIMER_SYNC_SYNCT2_R = crate::FieldReader<TIMER_SYNC_SYNCT2_A>;
#[doc = "Synchronize GPTM 16/32-Bit Timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT2_A {
    #[doc = "0: GPTM 16/32-Bit Timer 2 is not affected"]
    TIMER_SYNC_SYNCT2_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 16/32-Bit Timer 2 is triggered"]
    TIMER_SYNC_SYNCT2_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 16/32-Bit Timer 2 is triggered"]
    TIMER_SYNC_SYNCT2_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 2 is triggered"]
    TIMER_SYNC_SYNCT2_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT2_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCT2_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCT2_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_NONE,
            1 => TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TA,
            2 => TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TB,
            3 => TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 16/32-Bit Timer 2 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT2` writer - Synchronize GPTM 16/32-Bit Timer 2"]
pub type TIMER_SYNC_SYNCT2_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCT2_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCT2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 16/32-Bit Timer 2 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct2_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct2_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct2_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct2_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT3` reader - Synchronize GPTM 16/32-Bit Timer 3"]
pub type TIMER_SYNC_SYNCT3_R = crate::FieldReader<TIMER_SYNC_SYNCT3_A>;
#[doc = "Synchronize GPTM 16/32-Bit Timer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT3_A {
    #[doc = "0: GPTM 16/32-Bit Timer 3 is not affected"]
    TIMER_SYNC_SYNCT3_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 16/32-Bit Timer 3 is triggered"]
    TIMER_SYNC_SYNCT3_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 16/32-Bit Timer 3 is triggered"]
    TIMER_SYNC_SYNCT3_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 3 is triggered"]
    TIMER_SYNC_SYNCT3_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT3_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCT3_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCT3_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_NONE,
            1 => TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TA,
            2 => TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TB,
            3 => TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 16/32-Bit Timer 3 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT3` writer - Synchronize GPTM 16/32-Bit Timer 3"]
pub type TIMER_SYNC_SYNCT3_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCT3_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCT3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 16/32-Bit Timer 3 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct3_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct3_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct3_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct3_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT4` reader - Synchronize GPTM 16/32-Bit Timer 4"]
pub type TIMER_SYNC_SYNCT4_R = crate::FieldReader<TIMER_SYNC_SYNCT4_A>;
#[doc = "Synchronize GPTM 16/32-Bit Timer 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT4_A {
    #[doc = "0: GPTM 16/32-Bit Timer 4 is not affected"]
    TIMER_SYNC_SYNCT4_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 16/32-Bit Timer 4 is triggered"]
    TIMER_SYNC_SYNCT4_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 16/32-Bit Timer 4 is triggered"]
    TIMER_SYNC_SYNCT4_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 4 is triggered"]
    TIMER_SYNC_SYNCT4_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT4_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCT4_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCT4_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_NONE,
            1 => TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TA,
            2 => TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TB,
            3 => TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 16/32-Bit Timer 4 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT4` writer - Synchronize GPTM 16/32-Bit Timer 4"]
pub type TIMER_SYNC_SYNCT4_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCT4_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCT4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 16/32-Bit Timer 4 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct4_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct4_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct4_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct4_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT5` reader - Synchronize GPTM 16/32-Bit Timer 5"]
pub type TIMER_SYNC_SYNCT5_R = crate::FieldReader<TIMER_SYNC_SYNCT5_A>;
#[doc = "Synchronize GPTM 16/32-Bit Timer 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT5_A {
    #[doc = "0: GPTM 16/32-Bit Timer 5 is not affected"]
    TIMER_SYNC_SYNCT5_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 16/32-Bit Timer 5 is triggered"]
    TIMER_SYNC_SYNCT5_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 16/32-Bit Timer 5 is triggered"]
    TIMER_SYNC_SYNCT5_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 5 is triggered"]
    TIMER_SYNC_SYNCT5_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT5_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCT5_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCT5_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_NONE,
            1 => TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TA,
            2 => TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TB,
            3 => TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 16/32-Bit Timer 5 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT5` writer - Synchronize GPTM 16/32-Bit Timer 5"]
pub type TIMER_SYNC_SYNCT5_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCT5_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCT5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 16/32-Bit Timer 5 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct5_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 16/32-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct5_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 16/32-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct5_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct5_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT0` reader - Synchronize GPTM 32/64-Bit Timer 0"]
pub type TIMER_SYNC_SYNCWT0_R = crate::FieldReader<TIMER_SYNC_SYNCWT0_A>;
#[doc = "Synchronize GPTM 32/64-Bit Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCWT0_A {
    #[doc = "0: GPTM 32/64-Bit Timer 0 is not affected"]
    TIMER_SYNC_SYNCWT0_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 0 is triggered"]
    TIMER_SYNC_SYNCWT0_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    TIMER_SYNC_SYNCWT0_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    TIMER_SYNC_SYNCWT0_TATB = 3,
}
impl From<TIMER_SYNC_SYNCWT0_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCWT0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCWT0_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCWT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCWT0_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_NONE,
            1 => TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_TA,
            2 => TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_TB,
            3 => TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 0 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt0_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt0_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt0_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt0_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT0` writer - Synchronize GPTM 32/64-Bit Timer 0"]
pub type TIMER_SYNC_SYNCWT0_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCWT0_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCWT0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 32/64-Bit Timer 0 is not affected"]
    #[inline(always)]
    pub fn timer_sync_syncwt0_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt0_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt0_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt0_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT0_A::TIMER_SYNC_SYNCWT0_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT1` reader - Synchronize GPTM 32/64-Bit Timer 1"]
pub type TIMER_SYNC_SYNCWT1_R = crate::FieldReader<TIMER_SYNC_SYNCWT1_A>;
#[doc = "Synchronize GPTM 32/64-Bit Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCWT1_A {
    #[doc = "0: GPTM 32/64-Bit Timer 1 is not affected"]
    TIMER_SYNC_SYNCWT1_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 1 is triggered"]
    TIMER_SYNC_SYNCWT1_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    TIMER_SYNC_SYNCWT1_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    TIMER_SYNC_SYNCWT1_TATB = 3,
}
impl From<TIMER_SYNC_SYNCWT1_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCWT1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCWT1_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCWT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCWT1_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_NONE,
            1 => TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_TA,
            2 => TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_TB,
            3 => TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 1 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt1_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt1_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt1_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt1_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT1` writer - Synchronize GPTM 32/64-Bit Timer 1"]
pub type TIMER_SYNC_SYNCWT1_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCWT1_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCWT1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 32/64-Bit Timer 1 is not affected"]
    #[inline(always)]
    pub fn timer_sync_syncwt1_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt1_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt1_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt1_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT1_A::TIMER_SYNC_SYNCWT1_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT2` reader - Synchronize GPTM 32/64-Bit Timer 2"]
pub type TIMER_SYNC_SYNCWT2_R = crate::FieldReader<TIMER_SYNC_SYNCWT2_A>;
#[doc = "Synchronize GPTM 32/64-Bit Timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCWT2_A {
    #[doc = "0: GPTM 32/64-Bit Timer 2 is not affected"]
    TIMER_SYNC_SYNCWT2_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 2 is triggered"]
    TIMER_SYNC_SYNCWT2_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    TIMER_SYNC_SYNCWT2_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    TIMER_SYNC_SYNCWT2_TATB = 3,
}
impl From<TIMER_SYNC_SYNCWT2_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCWT2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCWT2_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCWT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCWT2_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_NONE,
            1 => TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_TA,
            2 => TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_TB,
            3 => TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 2 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt2_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt2_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt2_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt2_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT2` writer - Synchronize GPTM 32/64-Bit Timer 2"]
pub type TIMER_SYNC_SYNCWT2_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCWT2_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCWT2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 32/64-Bit Timer 2 is not affected"]
    #[inline(always)]
    pub fn timer_sync_syncwt2_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt2_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt2_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt2_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT2_A::TIMER_SYNC_SYNCWT2_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT3` reader - Synchronize GPTM 32/64-Bit Timer 3"]
pub type TIMER_SYNC_SYNCWT3_R = crate::FieldReader<TIMER_SYNC_SYNCWT3_A>;
#[doc = "Synchronize GPTM 32/64-Bit Timer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCWT3_A {
    #[doc = "0: GPTM 32/64-Bit Timer 3 is not affected"]
    TIMER_SYNC_SYNCWT3_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 3 is triggered"]
    TIMER_SYNC_SYNCWT3_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    TIMER_SYNC_SYNCWT3_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    TIMER_SYNC_SYNCWT3_TATB = 3,
}
impl From<TIMER_SYNC_SYNCWT3_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCWT3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCWT3_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCWT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCWT3_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_NONE,
            1 => TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_TA,
            2 => TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_TB,
            3 => TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 3 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt3_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt3_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt3_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt3_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT3` writer - Synchronize GPTM 32/64-Bit Timer 3"]
pub type TIMER_SYNC_SYNCWT3_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCWT3_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCWT3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 32/64-Bit Timer 3 is not affected"]
    #[inline(always)]
    pub fn timer_sync_syncwt3_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt3_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt3_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt3_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT3_A::TIMER_SYNC_SYNCWT3_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT4` reader - Synchronize GPTM 32/64-Bit Timer 4"]
pub type TIMER_SYNC_SYNCWT4_R = crate::FieldReader<TIMER_SYNC_SYNCWT4_A>;
#[doc = "Synchronize GPTM 32/64-Bit Timer 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCWT4_A {
    #[doc = "0: GPTM 32/64-Bit Timer 4 is not affected"]
    TIMER_SYNC_SYNCWT4_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 4 is triggered"]
    TIMER_SYNC_SYNCWT4_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    TIMER_SYNC_SYNCWT4_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    TIMER_SYNC_SYNCWT4_TATB = 3,
}
impl From<TIMER_SYNC_SYNCWT4_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCWT4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCWT4_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCWT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCWT4_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_NONE,
            1 => TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_TA,
            2 => TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_TB,
            3 => TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 4 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt4_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt4_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt4_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt4_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT4` writer - Synchronize GPTM 32/64-Bit Timer 4"]
pub type TIMER_SYNC_SYNCWT4_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCWT4_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCWT4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 32/64-Bit Timer 4 is not affected"]
    #[inline(always)]
    pub fn timer_sync_syncwt4_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt4_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt4_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt4_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT4_A::TIMER_SYNC_SYNCWT4_TATB)
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT5` reader - Synchronize GPTM 32/64-Bit Timer 5"]
pub type TIMER_SYNC_SYNCWT5_R = crate::FieldReader<TIMER_SYNC_SYNCWT5_A>;
#[doc = "Synchronize GPTM 32/64-Bit Timer 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCWT5_A {
    #[doc = "0: GPTM 32/64-Bit Timer 5 is not affected"]
    TIMER_SYNC_SYNCWT5_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM 32/64-Bit Timer 5 is triggered"]
    TIMER_SYNC_SYNCWT5_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    TIMER_SYNC_SYNCWT5_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    TIMER_SYNC_SYNCWT5_TATB = 3,
}
impl From<TIMER_SYNC_SYNCWT5_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCWT5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_SYNC_SYNCWT5_A {
    type Ux = u8;
}
impl TIMER_SYNC_SYNCWT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_SYNC_SYNCWT5_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_NONE,
            1 => TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_TA,
            2 => TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_TB,
            3 => TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "GPTM 32/64-Bit Timer 5 is not affected"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt5_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_NONE
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt5_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_TA
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt5_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_TB
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn is_timer_sync_syncwt5_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCWT5` writer - Synchronize GPTM 32/64-Bit Timer 5"]
pub type TIMER_SYNC_SYNCWT5_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TIMER_SYNC_SYNCWT5_A>;
impl<'a, REG, const O: u8> TIMER_SYNC_SYNCWT5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPTM 32/64-Bit Timer 5 is not affected"]
    #[inline(always)]
    pub fn timer_sync_syncwt5_none(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt5_ta(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt5_tb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_syncwt5_tatb(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_SYNC_SYNCWT5_A::TIMER_SYNC_SYNCWT5_TATB)
    }
}
impl R {
    #[doc = "Bits 0:1 - Synchronize GPTM 16/32-Bit Timer 0"]
    #[inline(always)]
    pub fn timer_sync_synct0(&self) -> TIMER_SYNC_SYNCT0_R {
        TIMER_SYNC_SYNCT0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM 16/32-Bit Timer 1"]
    #[inline(always)]
    pub fn timer_sync_synct1(&self) -> TIMER_SYNC_SYNCT1_R {
        TIMER_SYNC_SYNCT1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM 16/32-Bit Timer 2"]
    #[inline(always)]
    pub fn timer_sync_synct2(&self) -> TIMER_SYNC_SYNCT2_R {
        TIMER_SYNC_SYNCT2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM 16/32-Bit Timer 3"]
    #[inline(always)]
    pub fn timer_sync_synct3(&self) -> TIMER_SYNC_SYNCT3_R {
        TIMER_SYNC_SYNCT3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Synchronize GPTM 16/32-Bit Timer 4"]
    #[inline(always)]
    pub fn timer_sync_synct4(&self) -> TIMER_SYNC_SYNCT4_R {
        TIMER_SYNC_SYNCT4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Synchronize GPTM 16/32-Bit Timer 5"]
    #[inline(always)]
    pub fn timer_sync_synct5(&self) -> TIMER_SYNC_SYNCT5_R {
        TIMER_SYNC_SYNCT5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Synchronize GPTM 32/64-Bit Timer 0"]
    #[inline(always)]
    pub fn timer_sync_syncwt0(&self) -> TIMER_SYNC_SYNCWT0_R {
        TIMER_SYNC_SYNCWT0_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Synchronize GPTM 32/64-Bit Timer 1"]
    #[inline(always)]
    pub fn timer_sync_syncwt1(&self) -> TIMER_SYNC_SYNCWT1_R {
        TIMER_SYNC_SYNCWT1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Synchronize GPTM 32/64-Bit Timer 2"]
    #[inline(always)]
    pub fn timer_sync_syncwt2(&self) -> TIMER_SYNC_SYNCWT2_R {
        TIMER_SYNC_SYNCWT2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Synchronize GPTM 32/64-Bit Timer 3"]
    #[inline(always)]
    pub fn timer_sync_syncwt3(&self) -> TIMER_SYNC_SYNCWT3_R {
        TIMER_SYNC_SYNCWT3_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Synchronize GPTM 32/64-Bit Timer 4"]
    #[inline(always)]
    pub fn timer_sync_syncwt4(&self) -> TIMER_SYNC_SYNCWT4_R {
        TIMER_SYNC_SYNCWT4_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Synchronize GPTM 32/64-Bit Timer 5"]
    #[inline(always)]
    pub fn timer_sync_syncwt5(&self) -> TIMER_SYNC_SYNCWT5_R {
        TIMER_SYNC_SYNCWT5_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronize GPTM 16/32-Bit Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_synct0(&mut self) -> TIMER_SYNC_SYNCT0_W<SYNC_SPEC, 0> {
        TIMER_SYNC_SYNCT0_W::new(self)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM 16/32-Bit Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_synct1(&mut self) -> TIMER_SYNC_SYNCT1_W<SYNC_SPEC, 2> {
        TIMER_SYNC_SYNCT1_W::new(self)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM 16/32-Bit Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_synct2(&mut self) -> TIMER_SYNC_SYNCT2_W<SYNC_SPEC, 4> {
        TIMER_SYNC_SYNCT2_W::new(self)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM 16/32-Bit Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_synct3(&mut self) -> TIMER_SYNC_SYNCT3_W<SYNC_SPEC, 6> {
        TIMER_SYNC_SYNCT3_W::new(self)
    }
    #[doc = "Bits 8:9 - Synchronize GPTM 16/32-Bit Timer 4"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_synct4(&mut self) -> TIMER_SYNC_SYNCT4_W<SYNC_SPEC, 8> {
        TIMER_SYNC_SYNCT4_W::new(self)
    }
    #[doc = "Bits 10:11 - Synchronize GPTM 16/32-Bit Timer 5"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_synct5(&mut self) -> TIMER_SYNC_SYNCT5_W<SYNC_SPEC, 10> {
        TIMER_SYNC_SYNCT5_W::new(self)
    }
    #[doc = "Bits 12:13 - Synchronize GPTM 32/64-Bit Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_syncwt0(&mut self) -> TIMER_SYNC_SYNCWT0_W<SYNC_SPEC, 12> {
        TIMER_SYNC_SYNCWT0_W::new(self)
    }
    #[doc = "Bits 14:15 - Synchronize GPTM 32/64-Bit Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_syncwt1(&mut self) -> TIMER_SYNC_SYNCWT1_W<SYNC_SPEC, 14> {
        TIMER_SYNC_SYNCWT1_W::new(self)
    }
    #[doc = "Bits 16:17 - Synchronize GPTM 32/64-Bit Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_syncwt2(&mut self) -> TIMER_SYNC_SYNCWT2_W<SYNC_SPEC, 16> {
        TIMER_SYNC_SYNCWT2_W::new(self)
    }
    #[doc = "Bits 18:19 - Synchronize GPTM 32/64-Bit Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_syncwt3(&mut self) -> TIMER_SYNC_SYNCWT3_W<SYNC_SPEC, 18> {
        TIMER_SYNC_SYNCWT3_W::new(self)
    }
    #[doc = "Bits 20:21 - Synchronize GPTM 32/64-Bit Timer 4"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_syncwt4(&mut self) -> TIMER_SYNC_SYNCWT4_W<SYNC_SPEC, 20> {
        TIMER_SYNC_SYNCWT4_W::new(self)
    }
    #[doc = "Bits 22:23 - Synchronize GPTM 32/64-Bit Timer 5"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sync_syncwt5(&mut self) -> TIMER_SYNC_SYNCWT5_W<SYNC_SPEC, 22> {
        TIMER_SYNC_SYNCWT5_W::new(self)
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
#[doc = "GPTM Synchronize\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
