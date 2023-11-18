#[doc = "Register `_3_CTL` reader"]
pub type R = crate::R<_3_CTL_SPEC>;
#[doc = "Register `_3_CTL` writer"]
pub type W = crate::W<_3_CTL_SPEC>;
#[doc = "Field `PWM_3_CTL_ENABLE` reader - PWM Block Enable"]
pub type PWM_3_CTL_ENABLE_R = crate::BitReader;
#[doc = "Field `PWM_3_CTL_ENABLE` writer - PWM Block Enable"]
pub type PWM_3_CTL_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_CTL_MODE` reader - Counter Mode"]
pub type PWM_3_CTL_MODE_R = crate::BitReader;
#[doc = "Field `PWM_3_CTL_MODE` writer - Counter Mode"]
pub type PWM_3_CTL_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_CTL_DEBUG` reader - Debug Mode"]
pub type PWM_3_CTL_DEBUG_R = crate::BitReader;
#[doc = "Field `PWM_3_CTL_DEBUG` writer - Debug Mode"]
pub type PWM_3_CTL_DEBUG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_CTL_LOADUPD` reader - Load Register Update Mode"]
pub type PWM_3_CTL_LOADUPD_R = crate::BitReader;
#[doc = "Field `PWM_3_CTL_LOADUPD` writer - Load Register Update Mode"]
pub type PWM_3_CTL_LOADUPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_CTL_CMPAUPD` reader - Comparator A Update Mode"]
pub type PWM_3_CTL_CMPAUPD_R = crate::BitReader;
#[doc = "Field `PWM_3_CTL_CMPAUPD` writer - Comparator A Update Mode"]
pub type PWM_3_CTL_CMPAUPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_CTL_CMPBUPD` reader - Comparator B Update Mode"]
pub type PWM_3_CTL_CMPBUPD_R = crate::BitReader;
#[doc = "Field `PWM_3_CTL_CMPBUPD` writer - Comparator B Update Mode"]
pub type PWM_3_CTL_CMPBUPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_CTL_GENAUPD` reader - PWMnGENA Update Mode"]
pub type PWM_3_CTL_GENAUPD_R = crate::FieldReader<PWM_3_CTL_GENAUPD_A>;
#[doc = "PWMnGENA Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_3_CTL_GENAUPD_A {
    #[doc = "0: Immediate"]
    PWM_3_CTL_GENAUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_3_CTL_GENAUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_3_CTL_GENAUPD_GS = 3,
}
impl From<PWM_3_CTL_GENAUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_3_CTL_GENAUPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_3_CTL_GENAUPD_A {
    type Ux = u8;
}
impl PWM_3_CTL_GENAUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_3_CTL_GENAUPD_A> {
        match self.bits {
            0 => Some(PWM_3_CTL_GENAUPD_A::PWM_3_CTL_GENAUPD_I),
            2 => Some(PWM_3_CTL_GENAUPD_A::PWM_3_CTL_GENAUPD_LS),
            3 => Some(PWM_3_CTL_GENAUPD_A::PWM_3_CTL_GENAUPD_GS),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_genaupd_i(&self) -> bool {
        *self == PWM_3_CTL_GENAUPD_A::PWM_3_CTL_GENAUPD_I
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_genaupd_ls(&self) -> bool {
        *self == PWM_3_CTL_GENAUPD_A::PWM_3_CTL_GENAUPD_LS
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_genaupd_gs(&self) -> bool {
        *self == PWM_3_CTL_GENAUPD_A::PWM_3_CTL_GENAUPD_GS
    }
}
#[doc = "Field `PWM_3_CTL_GENAUPD` writer - PWMnGENA Update Mode"]
pub type PWM_3_CTL_GENAUPD_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_3_CTL_GENAUPD_A>;
impl<'a, REG, const O: u8> PWM_3_CTL_GENAUPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_3_ctl_genaupd_i(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_GENAUPD_A::PWM_3_CTL_GENAUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_genaupd_ls(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_GENAUPD_A::PWM_3_CTL_GENAUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_genaupd_gs(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_GENAUPD_A::PWM_3_CTL_GENAUPD_GS)
    }
}
#[doc = "Field `PWM_3_CTL_GENBUPD` reader - PWMnGENB Update Mode"]
pub type PWM_3_CTL_GENBUPD_R = crate::FieldReader<PWM_3_CTL_GENBUPD_A>;
#[doc = "PWMnGENB Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_3_CTL_GENBUPD_A {
    #[doc = "0: Immediate"]
    PWM_3_CTL_GENBUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_3_CTL_GENBUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_3_CTL_GENBUPD_GS = 3,
}
impl From<PWM_3_CTL_GENBUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_3_CTL_GENBUPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_3_CTL_GENBUPD_A {
    type Ux = u8;
}
impl PWM_3_CTL_GENBUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_3_CTL_GENBUPD_A> {
        match self.bits {
            0 => Some(PWM_3_CTL_GENBUPD_A::PWM_3_CTL_GENBUPD_I),
            2 => Some(PWM_3_CTL_GENBUPD_A::PWM_3_CTL_GENBUPD_LS),
            3 => Some(PWM_3_CTL_GENBUPD_A::PWM_3_CTL_GENBUPD_GS),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_genbupd_i(&self) -> bool {
        *self == PWM_3_CTL_GENBUPD_A::PWM_3_CTL_GENBUPD_I
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_genbupd_ls(&self) -> bool {
        *self == PWM_3_CTL_GENBUPD_A::PWM_3_CTL_GENBUPD_LS
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_genbupd_gs(&self) -> bool {
        *self == PWM_3_CTL_GENBUPD_A::PWM_3_CTL_GENBUPD_GS
    }
}
#[doc = "Field `PWM_3_CTL_GENBUPD` writer - PWMnGENB Update Mode"]
pub type PWM_3_CTL_GENBUPD_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_3_CTL_GENBUPD_A>;
impl<'a, REG, const O: u8> PWM_3_CTL_GENBUPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_3_ctl_genbupd_i(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_GENBUPD_A::PWM_3_CTL_GENBUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_genbupd_ls(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_GENBUPD_A::PWM_3_CTL_GENBUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_genbupd_gs(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_GENBUPD_A::PWM_3_CTL_GENBUPD_GS)
    }
}
#[doc = "Field `PWM_3_CTL_DBCTLUPD` reader - PWMnDBCTL Update Mode"]
pub type PWM_3_CTL_DBCTLUPD_R = crate::FieldReader<PWM_3_CTL_DBCTLUPD_A>;
#[doc = "PWMnDBCTL Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_3_CTL_DBCTLUPD_A {
    #[doc = "0: Immediate"]
    PWM_3_CTL_DBCTLUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_3_CTL_DBCTLUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_3_CTL_DBCTLUPD_GS = 3,
}
impl From<PWM_3_CTL_DBCTLUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_3_CTL_DBCTLUPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_3_CTL_DBCTLUPD_A {
    type Ux = u8;
}
impl PWM_3_CTL_DBCTLUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_3_CTL_DBCTLUPD_A> {
        match self.bits {
            0 => Some(PWM_3_CTL_DBCTLUPD_A::PWM_3_CTL_DBCTLUPD_I),
            2 => Some(PWM_3_CTL_DBCTLUPD_A::PWM_3_CTL_DBCTLUPD_LS),
            3 => Some(PWM_3_CTL_DBCTLUPD_A::PWM_3_CTL_DBCTLUPD_GS),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_dbctlupd_i(&self) -> bool {
        *self == PWM_3_CTL_DBCTLUPD_A::PWM_3_CTL_DBCTLUPD_I
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_dbctlupd_ls(&self) -> bool {
        *self == PWM_3_CTL_DBCTLUPD_A::PWM_3_CTL_DBCTLUPD_LS
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_dbctlupd_gs(&self) -> bool {
        *self == PWM_3_CTL_DBCTLUPD_A::PWM_3_CTL_DBCTLUPD_GS
    }
}
#[doc = "Field `PWM_3_CTL_DBCTLUPD` writer - PWMnDBCTL Update Mode"]
pub type PWM_3_CTL_DBCTLUPD_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_3_CTL_DBCTLUPD_A>;
impl<'a, REG, const O: u8> PWM_3_CTL_DBCTLUPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbctlupd_i(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_DBCTLUPD_A::PWM_3_CTL_DBCTLUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbctlupd_ls(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_DBCTLUPD_A::PWM_3_CTL_DBCTLUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbctlupd_gs(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_DBCTLUPD_A::PWM_3_CTL_DBCTLUPD_GS)
    }
}
#[doc = "Field `PWM_3_CTL_DBRISEUPD` reader - PWMnDBRISE Update Mode"]
pub type PWM_3_CTL_DBRISEUPD_R = crate::FieldReader<PWM_3_CTL_DBRISEUPD_A>;
#[doc = "PWMnDBRISE Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_3_CTL_DBRISEUPD_A {
    #[doc = "0: Immediate"]
    PWM_3_CTL_DBRISEUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_3_CTL_DBRISEUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_3_CTL_DBRISEUPD_GS = 3,
}
impl From<PWM_3_CTL_DBRISEUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_3_CTL_DBRISEUPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_3_CTL_DBRISEUPD_A {
    type Ux = u8;
}
impl PWM_3_CTL_DBRISEUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_3_CTL_DBRISEUPD_A> {
        match self.bits {
            0 => Some(PWM_3_CTL_DBRISEUPD_A::PWM_3_CTL_DBRISEUPD_I),
            2 => Some(PWM_3_CTL_DBRISEUPD_A::PWM_3_CTL_DBRISEUPD_LS),
            3 => Some(PWM_3_CTL_DBRISEUPD_A::PWM_3_CTL_DBRISEUPD_GS),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_dbriseupd_i(&self) -> bool {
        *self == PWM_3_CTL_DBRISEUPD_A::PWM_3_CTL_DBRISEUPD_I
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_dbriseupd_ls(&self) -> bool {
        *self == PWM_3_CTL_DBRISEUPD_A::PWM_3_CTL_DBRISEUPD_LS
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_dbriseupd_gs(&self) -> bool {
        *self == PWM_3_CTL_DBRISEUPD_A::PWM_3_CTL_DBRISEUPD_GS
    }
}
#[doc = "Field `PWM_3_CTL_DBRISEUPD` writer - PWMnDBRISE Update Mode"]
pub type PWM_3_CTL_DBRISEUPD_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_3_CTL_DBRISEUPD_A>;
impl<'a, REG, const O: u8> PWM_3_CTL_DBRISEUPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbriseupd_i(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_DBRISEUPD_A::PWM_3_CTL_DBRISEUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbriseupd_ls(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_DBRISEUPD_A::PWM_3_CTL_DBRISEUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbriseupd_gs(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_DBRISEUPD_A::PWM_3_CTL_DBRISEUPD_GS)
    }
}
#[doc = "Field `PWM_3_CTL_DBFALLUPD` reader - Specifies the update mode for the PWMnDBFALL register"]
pub type PWM_3_CTL_DBFALLUPD_R = crate::FieldReader<PWM_3_CTL_DBFALLUPD_A>;
#[doc = "Specifies the update mode for the PWMnDBFALL register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM_3_CTL_DBFALLUPD_A {
    #[doc = "0: Immediate"]
    PWM_3_CTL_DBFALLUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_3_CTL_DBFALLUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_3_CTL_DBFALLUPD_GS = 3,
}
impl From<PWM_3_CTL_DBFALLUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_3_CTL_DBFALLUPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM_3_CTL_DBFALLUPD_A {
    type Ux = u8;
}
impl PWM_3_CTL_DBFALLUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM_3_CTL_DBFALLUPD_A> {
        match self.bits {
            0 => Some(PWM_3_CTL_DBFALLUPD_A::PWM_3_CTL_DBFALLUPD_I),
            2 => Some(PWM_3_CTL_DBFALLUPD_A::PWM_3_CTL_DBFALLUPD_LS),
            3 => Some(PWM_3_CTL_DBFALLUPD_A::PWM_3_CTL_DBFALLUPD_GS),
            _ => None,
        }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_dbfallupd_i(&self) -> bool {
        *self == PWM_3_CTL_DBFALLUPD_A::PWM_3_CTL_DBFALLUPD_I
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_dbfallupd_ls(&self) -> bool {
        *self == PWM_3_CTL_DBFALLUPD_A::PWM_3_CTL_DBFALLUPD_LS
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn is_pwm_3_ctl_dbfallupd_gs(&self) -> bool {
        *self == PWM_3_CTL_DBFALLUPD_A::PWM_3_CTL_DBFALLUPD_GS
    }
}
#[doc = "Field `PWM_3_CTL_DBFALLUPD` writer - Specifies the update mode for the PWMnDBFALL register"]
pub type PWM_3_CTL_DBFALLUPD_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, PWM_3_CTL_DBFALLUPD_A>;
impl<'a, REG, const O: u8> PWM_3_CTL_DBFALLUPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbfallupd_i(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_DBFALLUPD_A::PWM_3_CTL_DBFALLUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbfallupd_ls(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_DBFALLUPD_A::PWM_3_CTL_DBFALLUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbfallupd_gs(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_3_CTL_DBFALLUPD_A::PWM_3_CTL_DBFALLUPD_GS)
    }
}
#[doc = "Field `PWM_3_CTL_FLTSRC` reader - Fault Condition Source"]
pub type PWM_3_CTL_FLTSRC_R = crate::BitReader;
#[doc = "Field `PWM_3_CTL_FLTSRC` writer - Fault Condition Source"]
pub type PWM_3_CTL_FLTSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_CTL_MINFLTPER` reader - Minimum Fault Period"]
pub type PWM_3_CTL_MINFLTPER_R = crate::BitReader;
#[doc = "Field `PWM_3_CTL_MINFLTPER` writer - Minimum Fault Period"]
pub type PWM_3_CTL_MINFLTPER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_3_CTL_LATCH` reader - Latch Fault Input"]
pub type PWM_3_CTL_LATCH_R = crate::BitReader;
#[doc = "Field `PWM_3_CTL_LATCH` writer - Latch Fault Input"]
pub type PWM_3_CTL_LATCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    pub fn pwm_3_ctl_enable(&self) -> PWM_3_CTL_ENABLE_R {
        PWM_3_CTL_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    pub fn pwm_3_ctl_mode(&self) -> PWM_3_CTL_MODE_R {
        PWM_3_CTL_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    pub fn pwm_3_ctl_debug(&self) -> PWM_3_CTL_DEBUG_R {
        PWM_3_CTL_DEBUG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    pub fn pwm_3_ctl_loadupd(&self) -> PWM_3_CTL_LOADUPD_R {
        PWM_3_CTL_LOADUPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    pub fn pwm_3_ctl_cmpaupd(&self) -> PWM_3_CTL_CMPAUPD_R {
        PWM_3_CTL_CMPAUPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    pub fn pwm_3_ctl_cmpbupd(&self) -> PWM_3_CTL_CMPBUPD_R {
        PWM_3_CTL_CMPBUPD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline(always)]
    pub fn pwm_3_ctl_genaupd(&self) -> PWM_3_CTL_GENAUPD_R {
        PWM_3_CTL_GENAUPD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline(always)]
    pub fn pwm_3_ctl_genbupd(&self) -> PWM_3_CTL_GENBUPD_R {
        PWM_3_CTL_GENBUPD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbctlupd(&self) -> PWM_3_CTL_DBCTLUPD_R {
        PWM_3_CTL_DBCTLUPD_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbriseupd(&self) -> PWM_3_CTL_DBRISEUPD_R {
        PWM_3_CTL_DBRISEUPD_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Specifies the update mode for the PWMnDBFALL register"]
    #[inline(always)]
    pub fn pwm_3_ctl_dbfallupd(&self) -> PWM_3_CTL_DBFALLUPD_R {
        PWM_3_CTL_DBFALLUPD_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline(always)]
    pub fn pwm_3_ctl_fltsrc(&self) -> PWM_3_CTL_FLTSRC_R {
        PWM_3_CTL_FLTSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline(always)]
    pub fn pwm_3_ctl_minfltper(&self) -> PWM_3_CTL_MINFLTPER_R {
        PWM_3_CTL_MINFLTPER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline(always)]
    pub fn pwm_3_ctl_latch(&self) -> PWM_3_CTL_LATCH_R {
        PWM_3_CTL_LATCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_enable(&mut self) -> PWM_3_CTL_ENABLE_W<_3_CTL_SPEC, 0> {
        PWM_3_CTL_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_mode(&mut self) -> PWM_3_CTL_MODE_W<_3_CTL_SPEC, 1> {
        PWM_3_CTL_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_debug(&mut self) -> PWM_3_CTL_DEBUG_W<_3_CTL_SPEC, 2> {
        PWM_3_CTL_DEBUG_W::new(self)
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_loadupd(&mut self) -> PWM_3_CTL_LOADUPD_W<_3_CTL_SPEC, 3> {
        PWM_3_CTL_LOADUPD_W::new(self)
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_cmpaupd(&mut self) -> PWM_3_CTL_CMPAUPD_W<_3_CTL_SPEC, 4> {
        PWM_3_CTL_CMPAUPD_W::new(self)
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_cmpbupd(&mut self) -> PWM_3_CTL_CMPBUPD_W<_3_CTL_SPEC, 5> {
        PWM_3_CTL_CMPBUPD_W::new(self)
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_genaupd(&mut self) -> PWM_3_CTL_GENAUPD_W<_3_CTL_SPEC, 6> {
        PWM_3_CTL_GENAUPD_W::new(self)
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_genbupd(&mut self) -> PWM_3_CTL_GENBUPD_W<_3_CTL_SPEC, 8> {
        PWM_3_CTL_GENBUPD_W::new(self)
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_dbctlupd(&mut self) -> PWM_3_CTL_DBCTLUPD_W<_3_CTL_SPEC, 10> {
        PWM_3_CTL_DBCTLUPD_W::new(self)
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_dbriseupd(&mut self) -> PWM_3_CTL_DBRISEUPD_W<_3_CTL_SPEC, 12> {
        PWM_3_CTL_DBRISEUPD_W::new(self)
    }
    #[doc = "Bits 14:15 - Specifies the update mode for the PWMnDBFALL register"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_dbfallupd(&mut self) -> PWM_3_CTL_DBFALLUPD_W<_3_CTL_SPEC, 14> {
        PWM_3_CTL_DBFALLUPD_W::new(self)
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_fltsrc(&mut self) -> PWM_3_CTL_FLTSRC_W<_3_CTL_SPEC, 16> {
        PWM_3_CTL_FLTSRC_W::new(self)
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_minfltper(&mut self) -> PWM_3_CTL_MINFLTPER_W<_3_CTL_SPEC, 17> {
        PWM_3_CTL_MINFLTPER_W::new(self)
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_3_ctl_latch(&mut self) -> PWM_3_CTL_LATCH_W<_3_CTL_SPEC, 18> {
        PWM_3_CTL_LATCH_W::new(self)
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
#[doc = "PWM3 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_3_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_3_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_CTL_SPEC;
impl crate::RegisterSpec for _3_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_ctl::R`](R) reader structure"]
impl crate::Readable for _3_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_3_ctl::W`](W) writer structure"]
impl crate::Writable for _3_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _3_CTL to value 0"]
impl crate::Resettable for _3_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
