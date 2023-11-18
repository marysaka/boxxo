#[doc = "Register `SSIZE` reader"]
pub type R = crate::R<SSIZE_SPEC>;
#[doc = "Register `SSIZE` writer"]
pub type W = crate::W<SSIZE_SPEC>;
#[doc = "Field `FLASH_SSIZE_SIZE` reader - SRAM Size"]
pub type FLASH_SSIZE_SIZE_R = crate::FieldReader<FLASH_SSIZE_SIZE_A>;
#[doc = "SRAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum FLASH_SSIZE_SIZE_A {
    #[doc = "7: 2 KB of SRAM"]
    FLASH_SSIZE_SIZE_2KB = 7,
    #[doc = "15: 4 KB of SRAM"]
    FLASH_SSIZE_SIZE_4KB = 15,
    #[doc = "23: 6 KB of SRAM"]
    FLASH_SSIZE_SIZE_6KB = 23,
    #[doc = "31: 8 KB of SRAM"]
    FLASH_SSIZE_SIZE_8KB = 31,
    #[doc = "47: 12 KB of SRAM"]
    FLASH_SSIZE_SIZE_12KB = 47,
    #[doc = "63: 16 KB of SRAM"]
    FLASH_SSIZE_SIZE_16KB = 63,
    #[doc = "79: 20 KB of SRAM"]
    FLASH_SSIZE_SIZE_20KB = 79,
    #[doc = "95: 24 KB of SRAM"]
    FLASH_SSIZE_SIZE_24KB = 95,
    #[doc = "127: 32 KB of SRAM"]
    FLASH_SSIZE_SIZE_32KB = 127,
}
impl From<FLASH_SSIZE_SIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: FLASH_SSIZE_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLASH_SSIZE_SIZE_A {
    type Ux = u16;
}
impl FLASH_SSIZE_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FLASH_SSIZE_SIZE_A> {
        match self.bits {
            7 => Some(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_2KB),
            15 => Some(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_4KB),
            23 => Some(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_6KB),
            31 => Some(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_8KB),
            47 => Some(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_12KB),
            63 => Some(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_16KB),
            79 => Some(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_20KB),
            95 => Some(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_24KB),
            127 => Some(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_32KB),
            _ => None,
        }
    }
    #[doc = "2 KB of SRAM"]
    #[inline(always)]
    pub fn is_flash_ssize_size_2kb(&self) -> bool {
        *self == FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_2KB
    }
    #[doc = "4 KB of SRAM"]
    #[inline(always)]
    pub fn is_flash_ssize_size_4kb(&self) -> bool {
        *self == FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_4KB
    }
    #[doc = "6 KB of SRAM"]
    #[inline(always)]
    pub fn is_flash_ssize_size_6kb(&self) -> bool {
        *self == FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_6KB
    }
    #[doc = "8 KB of SRAM"]
    #[inline(always)]
    pub fn is_flash_ssize_size_8kb(&self) -> bool {
        *self == FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_8KB
    }
    #[doc = "12 KB of SRAM"]
    #[inline(always)]
    pub fn is_flash_ssize_size_12kb(&self) -> bool {
        *self == FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_12KB
    }
    #[doc = "16 KB of SRAM"]
    #[inline(always)]
    pub fn is_flash_ssize_size_16kb(&self) -> bool {
        *self == FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_16KB
    }
    #[doc = "20 KB of SRAM"]
    #[inline(always)]
    pub fn is_flash_ssize_size_20kb(&self) -> bool {
        *self == FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_20KB
    }
    #[doc = "24 KB of SRAM"]
    #[inline(always)]
    pub fn is_flash_ssize_size_24kb(&self) -> bool {
        *self == FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_24KB
    }
    #[doc = "32 KB of SRAM"]
    #[inline(always)]
    pub fn is_flash_ssize_size_32kb(&self) -> bool {
        *self == FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_32KB
    }
}
#[doc = "Field `FLASH_SSIZE_SIZE` writer - SRAM Size"]
pub type FLASH_SSIZE_SIZE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, FLASH_SSIZE_SIZE_A>;
impl<'a, REG, const O: u8> FLASH_SSIZE_SIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "2 KB of SRAM"]
    #[inline(always)]
    pub fn flash_ssize_size_2kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_2KB)
    }
    #[doc = "4 KB of SRAM"]
    #[inline(always)]
    pub fn flash_ssize_size_4kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_4KB)
    }
    #[doc = "6 KB of SRAM"]
    #[inline(always)]
    pub fn flash_ssize_size_6kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_6KB)
    }
    #[doc = "8 KB of SRAM"]
    #[inline(always)]
    pub fn flash_ssize_size_8kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_8KB)
    }
    #[doc = "12 KB of SRAM"]
    #[inline(always)]
    pub fn flash_ssize_size_12kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_12KB)
    }
    #[doc = "16 KB of SRAM"]
    #[inline(always)]
    pub fn flash_ssize_size_16kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_16KB)
    }
    #[doc = "20 KB of SRAM"]
    #[inline(always)]
    pub fn flash_ssize_size_20kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_20KB)
    }
    #[doc = "24 KB of SRAM"]
    #[inline(always)]
    pub fn flash_ssize_size_24kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_24KB)
    }
    #[doc = "32 KB of SRAM"]
    #[inline(always)]
    pub fn flash_ssize_size_32kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_SSIZE_SIZE_A::FLASH_SSIZE_SIZE_32KB)
    }
}
impl R {
    #[doc = "Bits 0:15 - SRAM Size"]
    #[inline(always)]
    pub fn flash_ssize_size(&self) -> FLASH_SSIZE_SIZE_R {
        FLASH_SSIZE_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRAM Size"]
    #[inline(always)]
    #[must_use]
    pub fn flash_ssize_size(&mut self) -> FLASH_SSIZE_SIZE_W<SSIZE_SPEC, 0> {
        FLASH_SSIZE_SIZE_W::new(self)
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
#[doc = "SRAM Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSIZE_SPEC;
impl crate::RegisterSpec for SSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssize::R`](R) reader structure"]
impl crate::Readable for SSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssize::W`](W) writer structure"]
impl crate::Writable for SSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSIZE to value 0"]
impl crate::Resettable for SSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
