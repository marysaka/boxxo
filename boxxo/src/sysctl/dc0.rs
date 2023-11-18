#[doc = "Register `DC0` reader"]
pub type R = crate::R<DC0_SPEC>;
#[doc = "Register `DC0` writer"]
pub type W = crate::W<DC0_SPEC>;
#[doc = "Field `SYSCTL_DC0_FLASHSZ` reader - Flash Size"]
pub type SYSCTL_DC0_FLASHSZ_R = crate::FieldReader<SYSCTL_DC0_FLASHSZ_A>;
#[doc = "Flash Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SYSCTL_DC0_FLASHSZ_A {
    #[doc = "3: 8 KB of Flash"]
    SYSCTL_DC0_FLASHSZ_8KB = 3,
    #[doc = "7: 16 KB of Flash"]
    SYSCTL_DC0_FLASHSZ_16KB = 7,
    #[doc = "15: 32 KB of Flash"]
    SYSCTL_DC0_FLASHSZ_32KB = 15,
    #[doc = "31: 64 KB of Flash"]
    SYSCTL_DC0_FLASHSZ_64KB = 31,
    #[doc = "47: 96 KB of Flash"]
    SYSCTL_DC0_FLASHSZ_96KB = 47,
    #[doc = "63: 128 KB of Flash"]
    SYSCTL_DC0_FLASHSZ_128K = 63,
    #[doc = "95: 192 KB of Flash"]
    SYSCTL_DC0_FLASHSZ_192K = 95,
    #[doc = "127: 256 KB of Flash"]
    SYSCTL_DC0_FLASHSZ_256K = 127,
}
impl From<SYSCTL_DC0_FLASHSZ_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSCTL_DC0_FLASHSZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DC0_FLASHSZ_A {
    type Ux = u16;
}
impl SYSCTL_DC0_FLASHSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DC0_FLASHSZ_A> {
        match self.bits {
            3 => Some(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_8KB),
            7 => Some(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_16KB),
            15 => Some(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_32KB),
            31 => Some(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_64KB),
            47 => Some(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_96KB),
            63 => Some(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_128K),
            95 => Some(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_192K),
            127 => Some(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_256K),
            _ => None,
        }
    }
    #[doc = "8 KB of Flash"]
    #[inline(always)]
    pub fn is_sysctl_dc0_flashsz_8kb(&self) -> bool {
        *self == SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_8KB
    }
    #[doc = "16 KB of Flash"]
    #[inline(always)]
    pub fn is_sysctl_dc0_flashsz_16kb(&self) -> bool {
        *self == SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_16KB
    }
    #[doc = "32 KB of Flash"]
    #[inline(always)]
    pub fn is_sysctl_dc0_flashsz_32kb(&self) -> bool {
        *self == SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_32KB
    }
    #[doc = "64 KB of Flash"]
    #[inline(always)]
    pub fn is_sysctl_dc0_flashsz_64kb(&self) -> bool {
        *self == SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_64KB
    }
    #[doc = "96 KB of Flash"]
    #[inline(always)]
    pub fn is_sysctl_dc0_flashsz_96kb(&self) -> bool {
        *self == SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_96KB
    }
    #[doc = "128 KB of Flash"]
    #[inline(always)]
    pub fn is_sysctl_dc0_flashsz_128k(&self) -> bool {
        *self == SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_128K
    }
    #[doc = "192 KB of Flash"]
    #[inline(always)]
    pub fn is_sysctl_dc0_flashsz_192k(&self) -> bool {
        *self == SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_192K
    }
    #[doc = "256 KB of Flash"]
    #[inline(always)]
    pub fn is_sysctl_dc0_flashsz_256k(&self) -> bool {
        *self == SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_256K
    }
}
#[doc = "Field `SYSCTL_DC0_FLASHSZ` writer - Flash Size"]
pub type SYSCTL_DC0_FLASHSZ_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, SYSCTL_DC0_FLASHSZ_A>;
impl<'a, REG, const O: u8> SYSCTL_DC0_FLASHSZ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "8 KB of Flash"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz_8kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_8KB)
    }
    #[doc = "16 KB of Flash"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz_16kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_16KB)
    }
    #[doc = "32 KB of Flash"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz_32kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_32KB)
    }
    #[doc = "64 KB of Flash"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz_64kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_64KB)
    }
    #[doc = "96 KB of Flash"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz_96kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_96KB)
    }
    #[doc = "128 KB of Flash"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz_128k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_128K)
    }
    #[doc = "192 KB of Flash"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz_192k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_192K)
    }
    #[doc = "256 KB of Flash"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz_256k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_FLASHSZ_A::SYSCTL_DC0_FLASHSZ_256K)
    }
}
#[doc = "Field `SYSCTL_DC0_SRAMSZ` reader - SRAM Size"]
pub type SYSCTL_DC0_SRAMSZ_R = crate::FieldReader<SYSCTL_DC0_SRAMSZ_A>;
#[doc = "SRAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SYSCTL_DC0_SRAMSZ_A {
    #[doc = "7: 2 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_2KB = 7,
    #[doc = "15: 4 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_4KB = 15,
    #[doc = "23: 6 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_6KB = 23,
    #[doc = "31: 8 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_8KB = 31,
    #[doc = "47: 12 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_12KB = 47,
    #[doc = "63: 16 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_16KB = 63,
    #[doc = "79: 20 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_20KB = 79,
    #[doc = "95: 24 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_24KB = 95,
    #[doc = "127: 32 KB of SRAM"]
    SYSCTL_DC0_SRAMSZ_32KB = 127,
}
impl From<SYSCTL_DC0_SRAMSZ_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSCTL_DC0_SRAMSZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DC0_SRAMSZ_A {
    type Ux = u16;
}
impl SYSCTL_DC0_SRAMSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DC0_SRAMSZ_A> {
        match self.bits {
            7 => Some(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_2KB),
            15 => Some(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_4KB),
            23 => Some(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_6KB),
            31 => Some(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_8KB),
            47 => Some(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_12KB),
            63 => Some(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_16KB),
            79 => Some(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_20KB),
            95 => Some(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_24KB),
            127 => Some(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_32KB),
            _ => None,
        }
    }
    #[doc = "2 KB of SRAM"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_2kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_2KB
    }
    #[doc = "4 KB of SRAM"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_4kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_4KB
    }
    #[doc = "6 KB of SRAM"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_6kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_6KB
    }
    #[doc = "8 KB of SRAM"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_8kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_8KB
    }
    #[doc = "12 KB of SRAM"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_12kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_12KB
    }
    #[doc = "16 KB of SRAM"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_16kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_16KB
    }
    #[doc = "20 KB of SRAM"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_20kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_20KB
    }
    #[doc = "24 KB of SRAM"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_24kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_24KB
    }
    #[doc = "32 KB of SRAM"]
    #[inline(always)]
    pub fn is_sysctl_dc0_sramsz_32kb(&self) -> bool {
        *self == SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_32KB
    }
}
#[doc = "Field `SYSCTL_DC0_SRAMSZ` writer - SRAM Size"]
pub type SYSCTL_DC0_SRAMSZ_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, SYSCTL_DC0_SRAMSZ_A>;
impl<'a, REG, const O: u8> SYSCTL_DC0_SRAMSZ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "2 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_2kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_2KB)
    }
    #[doc = "4 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_4kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_4KB)
    }
    #[doc = "6 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_6kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_6KB)
    }
    #[doc = "8 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_8kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_8KB)
    }
    #[doc = "12 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_12kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_12KB)
    }
    #[doc = "16 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_16kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_16KB)
    }
    #[doc = "20 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_20kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_20KB)
    }
    #[doc = "24 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_24kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_24KB)
    }
    #[doc = "32 KB of SRAM"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz_32kb(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC0_SRAMSZ_A::SYSCTL_DC0_SRAMSZ_32KB)
    }
}
impl R {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn sysctl_dc0_flashsz(&self) -> SYSCTL_DC0_FLASHSZ_R {
        SYSCTL_DC0_FLASHSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM Size"]
    #[inline(always)]
    pub fn sysctl_dc0_sramsz(&self) -> SYSCTL_DC0_SRAMSZ_R {
        SYSCTL_DC0_SRAMSZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc0_flashsz(&mut self) -> SYSCTL_DC0_FLASHSZ_W<DC0_SPEC, 0> {
        SYSCTL_DC0_FLASHSZ_W::new(self)
    }
    #[doc = "Bits 16:31 - SRAM Size"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc0_sramsz(&mut self) -> SYSCTL_DC0_SRAMSZ_W<DC0_SPEC, 16> {
        SYSCTL_DC0_SRAMSZ_W::new(self)
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
#[doc = "Device Capabilities 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC0_SPEC;
impl crate::RegisterSpec for DC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc0::R`](R) reader structure"]
impl crate::Readable for DC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc0::W`](W) writer structure"]
impl crate::Writable for DC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC0 to value 0"]
impl crate::Resettable for DC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
