#[doc = "Register `DID1` reader"]
pub type R = crate::R<DID1_SPEC>;
#[doc = "Register `DID1` writer"]
pub type W = crate::W<DID1_SPEC>;
#[doc = "Field `SYSCTL_DID1_QUAL` reader - Qualification Status"]
pub type SYSCTL_DID1_QUAL_R = crate::FieldReader<SYSCTL_DID1_QUAL_A>;
#[doc = "Qualification Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DID1_QUAL_A {
    #[doc = "0: Engineering Sample (unqualified)"]
    SYSCTL_DID1_QUAL_ES = 0,
    #[doc = "1: Pilot Production (unqualified)"]
    SYSCTL_DID1_QUAL_PP = 1,
    #[doc = "2: Fully Qualified"]
    SYSCTL_DID1_QUAL_FQ = 2,
}
impl From<SYSCTL_DID1_QUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_QUAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DID1_QUAL_A {
    type Ux = u8;
}
impl SYSCTL_DID1_QUAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DID1_QUAL_A> {
        match self.bits {
            0 => Some(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_ES),
            1 => Some(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_PP),
            2 => Some(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_FQ),
            _ => None,
        }
    }
    #[doc = "Engineering Sample (unqualified)"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_es(&self) -> bool {
        *self == SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_ES
    }
    #[doc = "Pilot Production (unqualified)"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_pp(&self) -> bool {
        *self == SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_PP
    }
    #[doc = "Fully Qualified"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_fq(&self) -> bool {
        *self == SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_FQ
    }
}
#[doc = "Field `SYSCTL_DID1_QUAL` writer - Qualification Status"]
pub type SYSCTL_DID1_QUAL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, SYSCTL_DID1_QUAL_A>;
impl<'a, REG, const O: u8> SYSCTL_DID1_QUAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Engineering Sample (unqualified)"]
    #[inline(always)]
    pub fn sysctl_did1_qual_es(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_ES)
    }
    #[doc = "Pilot Production (unqualified)"]
    #[inline(always)]
    pub fn sysctl_did1_qual_pp(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_PP)
    }
    #[doc = "Fully Qualified"]
    #[inline(always)]
    pub fn sysctl_did1_qual_fq(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_FQ)
    }
}
#[doc = "Field `SYSCTL_DID1_ROHS` reader - RoHS-Compliance"]
pub type SYSCTL_DID1_ROHS_R = crate::BitReader;
#[doc = "Field `SYSCTL_DID1_ROHS` writer - RoHS-Compliance"]
pub type SYSCTL_DID1_ROHS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DID1_PKG` reader - Package Type"]
pub type SYSCTL_DID1_PKG_R = crate::FieldReader<SYSCTL_DID1_PKG_A>;
#[doc = "Package Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DID1_PKG_A {
    #[doc = "0: SOIC package"]
    SYSCTL_DID1_PKG_SOIC = 0,
    #[doc = "1: LQFP package"]
    SYSCTL_DID1_PKG_QFP = 1,
    #[doc = "2: BGA package"]
    SYSCTL_DID1_PKG_BGA = 2,
}
impl From<SYSCTL_DID1_PKG_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_PKG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DID1_PKG_A {
    type Ux = u8;
}
impl SYSCTL_DID1_PKG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DID1_PKG_A> {
        match self.bits {
            0 => Some(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_SOIC),
            1 => Some(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_QFP),
            2 => Some(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_BGA),
            _ => None,
        }
    }
    #[doc = "SOIC package"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_soic(&self) -> bool {
        *self == SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_SOIC
    }
    #[doc = "LQFP package"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_qfp(&self) -> bool {
        *self == SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_QFP
    }
    #[doc = "BGA package"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_bga(&self) -> bool {
        *self == SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_BGA
    }
}
#[doc = "Field `SYSCTL_DID1_PKG` writer - Package Type"]
pub type SYSCTL_DID1_PKG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, SYSCTL_DID1_PKG_A>;
impl<'a, REG, const O: u8> SYSCTL_DID1_PKG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SOIC package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_soic(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_SOIC)
    }
    #[doc = "LQFP package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_qfp(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_QFP)
    }
    #[doc = "BGA package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_bga(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_BGA)
    }
}
#[doc = "Field `SYSCTL_DID1_TEMP` reader - Temperature Range"]
pub type SYSCTL_DID1_TEMP_R = crate::FieldReader<SYSCTL_DID1_TEMP_A>;
#[doc = "Temperature Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DID1_TEMP_A {
    #[doc = "0: Commercial temperature range (0C to 70C)"]
    SYSCTL_DID1_TEMP_C = 0,
    #[doc = "1: Industrial temperature range (-40C to 85C)"]
    SYSCTL_DID1_TEMP_I = 1,
    #[doc = "2: Extended temperature range (-40C to 105C)"]
    SYSCTL_DID1_TEMP_E = 2,
}
impl From<SYSCTL_DID1_TEMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_TEMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DID1_TEMP_A {
    type Ux = u8;
}
impl SYSCTL_DID1_TEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DID1_TEMP_A> {
        match self.bits {
            0 => Some(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_C),
            1 => Some(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_I),
            2 => Some(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_E),
            _ => None,
        }
    }
    #[doc = "Commercial temperature range (0C to 70C)"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_c(&self) -> bool {
        *self == SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_C
    }
    #[doc = "Industrial temperature range (-40C to 85C)"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_i(&self) -> bool {
        *self == SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_I
    }
    #[doc = "Extended temperature range (-40C to 105C)"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_e(&self) -> bool {
        *self == SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_E
    }
}
#[doc = "Field `SYSCTL_DID1_TEMP` writer - Temperature Range"]
pub type SYSCTL_DID1_TEMP_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O, SYSCTL_DID1_TEMP_A>;
impl<'a, REG, const O: u8> SYSCTL_DID1_TEMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Commercial temperature range (0C to 70C)"]
    #[inline(always)]
    pub fn sysctl_did1_temp_c(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_C)
    }
    #[doc = "Industrial temperature range (-40C to 85C)"]
    #[inline(always)]
    pub fn sysctl_did1_temp_i(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_I)
    }
    #[doc = "Extended temperature range (-40C to 105C)"]
    #[inline(always)]
    pub fn sysctl_did1_temp_e(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_E)
    }
}
#[doc = "Field `SYSCTL_DID1_PINCNT` reader - Package Pin Count"]
pub type SYSCTL_DID1_PINCNT_R = crate::FieldReader<SYSCTL_DID1_PINCNT_A>;
#[doc = "Package Pin Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DID1_PINCNT_A {
    #[doc = "0: 28-pin package"]
    SYSCTL_DID1_PINCNT_28 = 0,
    #[doc = "1: 48-pin package"]
    SYSCTL_DID1_PINCNT_48 = 1,
    #[doc = "2: 100-pin package"]
    SYSCTL_DID1_PINCNT_100 = 2,
    #[doc = "3: 64-pin package"]
    SYSCTL_DID1_PINCNT_64 = 3,
    #[doc = "4: 144-pin package"]
    SYSCTL_DID1_PINCNT_144 = 4,
    #[doc = "5: 157-pin package"]
    SYSCTL_DID1_PINCNT_157 = 5,
}
impl From<SYSCTL_DID1_PINCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_PINCNT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DID1_PINCNT_A {
    type Ux = u8;
}
impl SYSCTL_DID1_PINCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DID1_PINCNT_A> {
        match self.bits {
            0 => Some(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_28),
            1 => Some(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_48),
            2 => Some(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_100),
            3 => Some(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_64),
            4 => Some(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_144),
            5 => Some(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_157),
            _ => None,
        }
    }
    #[doc = "28-pin package"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_28(&self) -> bool {
        *self == SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_28
    }
    #[doc = "48-pin package"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_48(&self) -> bool {
        *self == SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_48
    }
    #[doc = "100-pin package"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_100(&self) -> bool {
        *self == SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_100
    }
    #[doc = "64-pin package"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_64(&self) -> bool {
        *self == SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_64
    }
    #[doc = "144-pin package"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_144(&self) -> bool {
        *self == SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_144
    }
    #[doc = "157-pin package"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_157(&self) -> bool {
        *self == SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_157
    }
}
#[doc = "Field `SYSCTL_DID1_PINCNT` writer - Package Pin Count"]
pub type SYSCTL_DID1_PINCNT_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O, SYSCTL_DID1_PINCNT_A>;
impl<'a, REG, const O: u8> SYSCTL_DID1_PINCNT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "28-pin package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_28(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_28)
    }
    #[doc = "48-pin package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_48(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_48)
    }
    #[doc = "100-pin package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_100(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_100)
    }
    #[doc = "64-pin package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_64(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_64)
    }
    #[doc = "144-pin package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_144(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_144)
    }
    #[doc = "157-pin package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_157(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_157)
    }
}
#[doc = "Field `SYSCTL_DID1_PRTNO` reader - Part Number"]
pub type SYSCTL_DID1_PRTNO_R = crate::FieldReader;
#[doc = "Field `SYSCTL_DID1_PRTNO` writer - Part Number"]
pub type SYSCTL_DID1_PRTNO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SYSCTL_DID1_FAM` reader - Family"]
pub type SYSCTL_DID1_FAM_R = crate::FieldReader;
#[doc = "Field `SYSCTL_DID1_FAM` writer - Family"]
pub type SYSCTL_DID1_FAM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SYSCTL_DID1_VER` reader - DID1 Version"]
pub type SYSCTL_DID1_VER_R = crate::FieldReader;
#[doc = "Field `SYSCTL_DID1_VER` writer - DID1 Version"]
pub type SYSCTL_DID1_VER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline(always)]
    pub fn sysctl_did1_qual(&self) -> SYSCTL_DID1_QUAL_R {
        SYSCTL_DID1_QUAL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline(always)]
    pub fn sysctl_did1_rohs(&self) -> SYSCTL_DID1_ROHS_R {
        SYSCTL_DID1_ROHS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline(always)]
    pub fn sysctl_did1_pkg(&self) -> SYSCTL_DID1_PKG_R {
        SYSCTL_DID1_PKG_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline(always)]
    pub fn sysctl_did1_temp(&self) -> SYSCTL_DID1_TEMP_R {
        SYSCTL_DID1_TEMP_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt(&self) -> SYSCTL_DID1_PINCNT_R {
        SYSCTL_DID1_PINCNT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline(always)]
    pub fn sysctl_did1_prtno(&self) -> SYSCTL_DID1_PRTNO_R {
        SYSCTL_DID1_PRTNO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline(always)]
    pub fn sysctl_did1_fam(&self) -> SYSCTL_DID1_FAM_R {
        SYSCTL_DID1_FAM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline(always)]
    pub fn sysctl_did1_ver(&self) -> SYSCTL_DID1_VER_R {
        SYSCTL_DID1_VER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did1_qual(&mut self) -> SYSCTL_DID1_QUAL_W<DID1_SPEC, 0> {
        SYSCTL_DID1_QUAL_W::new(self)
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did1_rohs(&mut self) -> SYSCTL_DID1_ROHS_W<DID1_SPEC, 2> {
        SYSCTL_DID1_ROHS_W::new(self)
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did1_pkg(&mut self) -> SYSCTL_DID1_PKG_W<DID1_SPEC, 3> {
        SYSCTL_DID1_PKG_W::new(self)
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did1_temp(&mut self) -> SYSCTL_DID1_TEMP_W<DID1_SPEC, 5> {
        SYSCTL_DID1_TEMP_W::new(self)
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did1_pincnt(&mut self) -> SYSCTL_DID1_PINCNT_W<DID1_SPEC, 13> {
        SYSCTL_DID1_PINCNT_W::new(self)
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did1_prtno(&mut self) -> SYSCTL_DID1_PRTNO_W<DID1_SPEC, 16> {
        SYSCTL_DID1_PRTNO_W::new(self)
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did1_fam(&mut self) -> SYSCTL_DID1_FAM_W<DID1_SPEC, 24> {
        SYSCTL_DID1_FAM_W::new(self)
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did1_ver(&mut self) -> SYSCTL_DID1_VER_W<DID1_SPEC, 28> {
        SYSCTL_DID1_VER_W::new(self)
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
#[doc = "Device Identification 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`did1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`did1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DID1_SPEC;
impl crate::RegisterSpec for DID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`did1::R`](R) reader structure"]
impl crate::Readable for DID1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`did1::W`](W) writer structure"]
impl crate::Writable for DID1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DID1 to value 0"]
impl crate::Resettable for DID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
