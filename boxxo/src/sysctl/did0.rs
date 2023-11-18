#[doc = "Register `DID0` reader"]
pub type R = crate::R<DID0_SPEC>;
#[doc = "Register `DID0` writer"]
pub type W = crate::W<DID0_SPEC>;
#[doc = "Field `SYSCTL_DID0_MIN` reader - Minor Revision"]
pub type SYSCTL_DID0_MIN_R = crate::FieldReader<SYSCTL_DID0_MIN_A>;
#[doc = "Minor Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DID0_MIN_A {
    #[doc = "0: Initial device, or a major revision update"]
    SYSCTL_DID0_MIN_0 = 0,
    #[doc = "1: First metal layer change"]
    SYSCTL_DID0_MIN_1 = 1,
    #[doc = "2: Second metal layer change"]
    SYSCTL_DID0_MIN_2 = 2,
}
impl From<SYSCTL_DID0_MIN_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_MIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DID0_MIN_A {
    type Ux = u8;
}
impl SYSCTL_DID0_MIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DID0_MIN_A> {
        match self.bits {
            0 => Some(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_0),
            1 => Some(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_1),
            2 => Some(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_2),
            _ => None,
        }
    }
    #[doc = "Initial device, or a major revision update"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_0(&self) -> bool {
        *self == SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_0
    }
    #[doc = "First metal layer change"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_1(&self) -> bool {
        *self == SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_1
    }
    #[doc = "Second metal layer change"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_2(&self) -> bool {
        *self == SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_2
    }
}
#[doc = "Field `SYSCTL_DID0_MIN` writer - Minor Revision"]
pub type SYSCTL_DID0_MIN_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 8, O, SYSCTL_DID0_MIN_A>;
impl<'a, REG, const O: u8> SYSCTL_DID0_MIN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Initial device, or a major revision update"]
    #[inline(always)]
    pub fn sysctl_did0_min_0(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_0)
    }
    #[doc = "First metal layer change"]
    #[inline(always)]
    pub fn sysctl_did0_min_1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_1)
    }
    #[doc = "Second metal layer change"]
    #[inline(always)]
    pub fn sysctl_did0_min_2(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_2)
    }
}
#[doc = "Field `SYSCTL_DID0_MAJ` reader - Major Revision"]
pub type SYSCTL_DID0_MAJ_R = crate::FieldReader<SYSCTL_DID0_MAJ_A>;
#[doc = "Major Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DID0_MAJ_A {
    #[doc = "0: Revision A (initial device)"]
    SYSCTL_DID0_MAJ_REVA = 0,
    #[doc = "1: Revision B (first base layer revision)"]
    SYSCTL_DID0_MAJ_REVB = 1,
    #[doc = "2: Revision C (second base layer revision)"]
    SYSCTL_DID0_MAJ_REVC = 2,
}
impl From<SYSCTL_DID0_MAJ_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_MAJ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DID0_MAJ_A {
    type Ux = u8;
}
impl SYSCTL_DID0_MAJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DID0_MAJ_A> {
        match self.bits {
            0 => Some(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVA),
            1 => Some(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVB),
            2 => Some(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVC),
            _ => None,
        }
    }
    #[doc = "Revision A (initial device)"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_reva(&self) -> bool {
        *self == SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVA
    }
    #[doc = "Revision B (first base layer revision)"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_revb(&self) -> bool {
        *self == SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVB
    }
    #[doc = "Revision C (second base layer revision)"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_revc(&self) -> bool {
        *self == SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVC
    }
}
#[doc = "Field `SYSCTL_DID0_MAJ` writer - Major Revision"]
pub type SYSCTL_DID0_MAJ_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 8, O, SYSCTL_DID0_MAJ_A>;
impl<'a, REG, const O: u8> SYSCTL_DID0_MAJ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Revision A (initial device)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_reva(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVA)
    }
    #[doc = "Revision B (first base layer revision)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_revb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVB)
    }
    #[doc = "Revision C (second base layer revision)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_revc(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVC)
    }
}
#[doc = "Field `SYSCTL_DID0_CLASS` reader - Device Class"]
pub type SYSCTL_DID0_CLASS_R = crate::FieldReader<SYSCTL_DID0_CLASS_A>;
#[doc = "Device Class\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DID0_CLASS_A {
    #[doc = "5: Tiva(TM) C Series Blizzard-class microcontrollers"]
    SYSCTL_DID0_CLASS_BLIZZARD = 5,
}
impl From<SYSCTL_DID0_CLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_CLASS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DID0_CLASS_A {
    type Ux = u8;
}
impl SYSCTL_DID0_CLASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DID0_CLASS_A> {
        match self.bits {
            5 => Some(SYSCTL_DID0_CLASS_A::SYSCTL_DID0_CLASS_BLIZZARD),
            _ => None,
        }
    }
    #[doc = "Tiva(TM) C Series Blizzard-class microcontrollers"]
    #[inline(always)]
    pub fn is_sysctl_did0_class_blizzard(&self) -> bool {
        *self == SYSCTL_DID0_CLASS_A::SYSCTL_DID0_CLASS_BLIZZARD
    }
}
#[doc = "Field `SYSCTL_DID0_CLASS` writer - Device Class"]
pub type SYSCTL_DID0_CLASS_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 8, O, SYSCTL_DID0_CLASS_A>;
impl<'a, REG, const O: u8> SYSCTL_DID0_CLASS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Tiva(TM) C Series Blizzard-class microcontrollers"]
    #[inline(always)]
    pub fn sysctl_did0_class_blizzard(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID0_CLASS_A::SYSCTL_DID0_CLASS_BLIZZARD)
    }
}
#[doc = "Field `SYSCTL_DID0_VER` reader - DID0 Version"]
pub type SYSCTL_DID0_VER_R = crate::FieldReader<SYSCTL_DID0_VER_A>;
#[doc = "DID0 Version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DID0_VER_A {
    #[doc = "1: Second version of the DID0 register format"]
    SYSCTL_DID0_VER_1 = 1,
}
impl From<SYSCTL_DID0_VER_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_VER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DID0_VER_A {
    type Ux = u8;
}
impl SYSCTL_DID0_VER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DID0_VER_A> {
        match self.bits {
            1 => Some(SYSCTL_DID0_VER_A::SYSCTL_DID0_VER_1),
            _ => None,
        }
    }
    #[doc = "Second version of the DID0 register format"]
    #[inline(always)]
    pub fn is_sysctl_did0_ver_1(&self) -> bool {
        *self == SYSCTL_DID0_VER_A::SYSCTL_DID0_VER_1
    }
}
#[doc = "Field `SYSCTL_DID0_VER` writer - DID0 Version"]
pub type SYSCTL_DID0_VER_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O, SYSCTL_DID0_VER_A>;
impl<'a, REG, const O: u8> SYSCTL_DID0_VER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Second version of the DID0 register format"]
    #[inline(always)]
    pub fn sysctl_did0_ver_1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DID0_VER_A::SYSCTL_DID0_VER_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn sysctl_did0_min(&self) -> SYSCTL_DID0_MIN_R {
        SYSCTL_DID0_MIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn sysctl_did0_maj(&self) -> SYSCTL_DID0_MAJ_R {
        SYSCTL_DID0_MAJ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline(always)]
    pub fn sysctl_did0_class(&self) -> SYSCTL_DID0_CLASS_R {
        SYSCTL_DID0_CLASS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline(always)]
    pub fn sysctl_did0_ver(&self) -> SYSCTL_DID0_VER_R {
        SYSCTL_DID0_VER_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did0_min(&mut self) -> SYSCTL_DID0_MIN_W<DID0_SPEC, 0> {
        SYSCTL_DID0_MIN_W::new(self)
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did0_maj(&mut self) -> SYSCTL_DID0_MAJ_W<DID0_SPEC, 8> {
        SYSCTL_DID0_MAJ_W::new(self)
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did0_class(&mut self) -> SYSCTL_DID0_CLASS_W<DID0_SPEC, 16> {
        SYSCTL_DID0_CLASS_W::new(self)
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_did0_ver(&mut self) -> SYSCTL_DID0_VER_W<DID0_SPEC, 28> {
        SYSCTL_DID0_VER_W::new(self)
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
#[doc = "Device Identification 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`did0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`did0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DID0_SPEC;
impl crate::RegisterSpec for DID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`did0::R`](R) reader structure"]
impl crate::Readable for DID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`did0::W`](W) writer structure"]
impl crate::Writable for DID0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DID0 to value 0"]
impl crate::Resettable for DID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
