#[doc = "Register `ENUPD` reader"]
pub type R = crate::R<ENUPD_SPEC>;
#[doc = "Register `ENUPD` writer"]
pub type W = crate::W<ENUPD_SPEC>;
#[doc = "Field `PWM_ENUPD_ENUPD0` reader - PWM0 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD0_R = crate::FieldReader<PWM_ENUPD_ENUPD0_A>;
#[doc = "PWM0 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD0_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD0_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD0_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD0_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD0_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_ENUPD_ENUPD0_A {
    type Ux = u8;
}
impl PWM_ENUPD_ENUPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_ENUPD_ENUPD0_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_IMM),
            2 => Some(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_GSYNC),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd0_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_IMM
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd0_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_LSYNC
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd0_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD0` writer - PWM0 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_ENUPD_ENUPD0_A>;
impl<'a, REG, const O: u8> PWM_ENUPD_ENUPD0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0_imm(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0_lsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0_gsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_GSYNC)
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD1` reader - PWM1 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD1_R = crate::FieldReader<PWM_ENUPD_ENUPD1_A>;
#[doc = "PWM1 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD1_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD1_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD1_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD1_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD1_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_ENUPD_ENUPD1_A {
    type Ux = u8;
}
impl PWM_ENUPD_ENUPD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_ENUPD_ENUPD1_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_IMM),
            2 => Some(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_GSYNC),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd1_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_IMM
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd1_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_LSYNC
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd1_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD1` writer - PWM1 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD1_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_ENUPD_ENUPD1_A>;
impl<'a, REG, const O: u8> PWM_ENUPD_ENUPD1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1_imm(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1_lsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1_gsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_GSYNC)
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD2` reader - PWM2 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD2_R = crate::FieldReader<PWM_ENUPD_ENUPD2_A>;
#[doc = "PWM2 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD2_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD2_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD2_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD2_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD2_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_ENUPD_ENUPD2_A {
    type Ux = u8;
}
impl PWM_ENUPD_ENUPD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_ENUPD_ENUPD2_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_IMM),
            2 => Some(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_GSYNC),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd2_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_IMM
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd2_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_LSYNC
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd2_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD2` writer - PWM2 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD2_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_ENUPD_ENUPD2_A>;
impl<'a, REG, const O: u8> PWM_ENUPD_ENUPD2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2_imm(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2_lsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2_gsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_GSYNC)
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD3` reader - PWM3 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD3_R = crate::FieldReader<PWM_ENUPD_ENUPD3_A>;
#[doc = "PWM3 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD3_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD3_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD3_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD3_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD3_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_ENUPD_ENUPD3_A {
    type Ux = u8;
}
impl PWM_ENUPD_ENUPD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_ENUPD_ENUPD3_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_IMM),
            2 => Some(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_GSYNC),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd3_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_IMM
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd3_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_LSYNC
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd3_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD3` writer - PWM3 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD3_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_ENUPD_ENUPD3_A>;
impl<'a, REG, const O: u8> PWM_ENUPD_ENUPD3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3_imm(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3_lsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3_gsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_GSYNC)
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD4` reader - PWM4 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD4_R = crate::FieldReader<PWM_ENUPD_ENUPD4_A>;
#[doc = "PWM4 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD4_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD4_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD4_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD4_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD4_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_ENUPD_ENUPD4_A {
    type Ux = u8;
}
impl PWM_ENUPD_ENUPD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_ENUPD_ENUPD4_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_IMM),
            2 => Some(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_GSYNC),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd4_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_IMM
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd4_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_LSYNC
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd4_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD4` writer - PWM4 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD4_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_ENUPD_ENUPD4_A>;
impl<'a, REG, const O: u8> PWM_ENUPD_ENUPD4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4_imm(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4_lsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4_gsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_GSYNC)
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD5` reader - PWM5 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD5_R = crate::FieldReader<PWM_ENUPD_ENUPD5_A>;
#[doc = "PWM5 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD5_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD5_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD5_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD5_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD5_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_ENUPD_ENUPD5_A {
    type Ux = u8;
}
impl PWM_ENUPD_ENUPD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_ENUPD_ENUPD5_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_IMM),
            2 => Some(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_GSYNC),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd5_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_IMM
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd5_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_LSYNC
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd5_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD5` writer - PWM5 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD5_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_ENUPD_ENUPD5_A>;
impl<'a, REG, const O: u8> PWM_ENUPD_ENUPD5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5_imm(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5_lsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5_gsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_GSYNC)
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD6` reader - PWM6 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD6_R = crate::FieldReader<PWM_ENUPD_ENUPD6_A>;
#[doc = "PWM6 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD6_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD6_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD6_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD6_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD6_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_ENUPD_ENUPD6_A {
    type Ux = u8;
}
impl PWM_ENUPD_ENUPD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_ENUPD_ENUPD6_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_IMM),
            2 => Some(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_GSYNC),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd6_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_IMM
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd6_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_LSYNC
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd6_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD6` writer - PWM6 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD6_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_ENUPD_ENUPD6_A>;
impl<'a, REG, const O: u8> PWM_ENUPD_ENUPD6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6_imm(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6_lsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6_gsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_GSYNC)
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD7` reader - PWM7 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD7_R = crate::FieldReader<PWM_ENUPD_ENUPD7_A>;
#[doc = "PWM7 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD7_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD7_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD7_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD7_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD7_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_ENUPD_ENUPD7_A {
    type Ux = u8;
}
impl PWM_ENUPD_ENUPD7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_ENUPD_ENUPD7_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_IMM),
            2 => Some(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_GSYNC),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd7_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_IMM
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd7_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_LSYNC
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd7_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD7` writer - PWM7 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD7_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_ENUPD_ENUPD7_A>;
impl<'a, REG, const O: u8> PWM_ENUPD_ENUPD7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7_imm(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7_lsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7_gsync(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_GSYNC)
    }
}
impl R {
    #[doc = "Bits 0:1 - PWM0 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0(&self) -> PWM_ENUPD_ENUPD0_R {
        PWM_ENUPD_ENUPD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PWM1 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1(&self) -> PWM_ENUPD_ENUPD1_R {
        PWM_ENUPD_ENUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PWM2 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2(&self) -> PWM_ENUPD_ENUPD2_R {
        PWM_ENUPD_ENUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PWM3 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3(&self) -> PWM_ENUPD_ENUPD3_R {
        PWM_ENUPD_ENUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PWM4 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4(&self) -> PWM_ENUPD_ENUPD4_R {
        PWM_ENUPD_ENUPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PWM5 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5(&self) -> PWM_ENUPD_ENUPD5_R {
        PWM_ENUPD_ENUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PWM6 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6(&self) -> PWM_ENUPD_ENUPD6_R {
        PWM_ENUPD_ENUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PWM7 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7(&self) -> PWM_ENUPD_ENUPD7_R {
        PWM_ENUPD_ENUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM0 Enable Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enupd_enupd0(&mut self) -> PWM_ENUPD_ENUPD0_W<ENUPD_SPEC, 0> {
        PWM_ENUPD_ENUPD0_W::new(self)
    }
    #[doc = "Bits 2:3 - PWM1 Enable Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enupd_enupd1(&mut self) -> PWM_ENUPD_ENUPD1_W<ENUPD_SPEC, 2> {
        PWM_ENUPD_ENUPD1_W::new(self)
    }
    #[doc = "Bits 4:5 - PWM2 Enable Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enupd_enupd2(&mut self) -> PWM_ENUPD_ENUPD2_W<ENUPD_SPEC, 4> {
        PWM_ENUPD_ENUPD2_W::new(self)
    }
    #[doc = "Bits 6:7 - PWM3 Enable Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enupd_enupd3(&mut self) -> PWM_ENUPD_ENUPD3_W<ENUPD_SPEC, 6> {
        PWM_ENUPD_ENUPD3_W::new(self)
    }
    #[doc = "Bits 8:9 - PWM4 Enable Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enupd_enupd4(&mut self) -> PWM_ENUPD_ENUPD4_W<ENUPD_SPEC, 8> {
        PWM_ENUPD_ENUPD4_W::new(self)
    }
    #[doc = "Bits 10:11 - PWM5 Enable Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enupd_enupd5(&mut self) -> PWM_ENUPD_ENUPD5_W<ENUPD_SPEC, 10> {
        PWM_ENUPD_ENUPD5_W::new(self)
    }
    #[doc = "Bits 12:13 - PWM6 Enable Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enupd_enupd6(&mut self) -> PWM_ENUPD_ENUPD6_W<ENUPD_SPEC, 12> {
        PWM_ENUPD_ENUPD6_W::new(self)
    }
    #[doc = "Bits 14:15 - PWM7 Enable Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enupd_enupd7(&mut self) -> PWM_ENUPD_ENUPD7_W<ENUPD_SPEC, 14> {
        PWM_ENUPD_ENUPD7_W::new(self)
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
#[doc = "PWM Enable Update\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enupd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enupd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENUPD_SPEC;
impl crate::RegisterSpec for ENUPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enupd::R`](R) reader structure"]
impl crate::Readable for ENUPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enupd::W`](W) writer structure"]
impl crate::Writable for ENUPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENUPD to value 0"]
impl crate::Resettable for ENUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
