#[doc = "Register `FSIZE` reader"]
pub type R = crate::R<FSIZE_SPEC>;
#[doc = "Register `FSIZE` writer"]
pub type W = crate::W<FSIZE_SPEC>;
#[doc = "Field `FLASH_FSIZE_SIZE` reader - Flash Size"]
pub type FLASH_FSIZE_SIZE_R = crate::FieldReader<FLASH_FSIZE_SIZE_A>;
#[doc = "Flash Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum FLASH_FSIZE_SIZE_A {
    #[doc = "3: 8 KB of Flash"]
    FLASH_FSIZE_SIZE_8KB = 3,
    #[doc = "7: 16 KB of Flash"]
    FLASH_FSIZE_SIZE_16KB = 7,
    #[doc = "15: 32 KB of Flash"]
    FLASH_FSIZE_SIZE_32KB = 15,
    #[doc = "31: 64 KB of Flash"]
    FLASH_FSIZE_SIZE_64KB = 31,
    #[doc = "47: 96 KB of Flash"]
    FLASH_FSIZE_SIZE_96KB = 47,
    #[doc = "63: 128 KB of Flash"]
    FLASH_FSIZE_SIZE_128KB = 63,
    #[doc = "95: 192 KB of Flash"]
    FLASH_FSIZE_SIZE_192KB = 95,
    #[doc = "127: 256 KB of Flash"]
    FLASH_FSIZE_SIZE_256KB = 127,
}
impl From<FLASH_FSIZE_SIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: FLASH_FSIZE_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLASH_FSIZE_SIZE_A {
    type Ux = u16;
}
impl FLASH_FSIZE_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FLASH_FSIZE_SIZE_A> {
        match self.bits {
            3 => Some(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_8KB),
            7 => Some(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_16KB),
            15 => Some(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_32KB),
            31 => Some(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_64KB),
            47 => Some(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_96KB),
            63 => Some(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_128KB),
            95 => Some(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_192KB),
            127 => Some(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_256KB),
            _ => None,
        }
    }
    #[doc = "8 KB of Flash"]
    #[inline(always)]
    pub fn is_flash_fsize_size_8kb(&self) -> bool {
        *self == FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_8KB
    }
    #[doc = "16 KB of Flash"]
    #[inline(always)]
    pub fn is_flash_fsize_size_16kb(&self) -> bool {
        *self == FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_16KB
    }
    #[doc = "32 KB of Flash"]
    #[inline(always)]
    pub fn is_flash_fsize_size_32kb(&self) -> bool {
        *self == FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_32KB
    }
    #[doc = "64 KB of Flash"]
    #[inline(always)]
    pub fn is_flash_fsize_size_64kb(&self) -> bool {
        *self == FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_64KB
    }
    #[doc = "96 KB of Flash"]
    #[inline(always)]
    pub fn is_flash_fsize_size_96kb(&self) -> bool {
        *self == FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_96KB
    }
    #[doc = "128 KB of Flash"]
    #[inline(always)]
    pub fn is_flash_fsize_size_128kb(&self) -> bool {
        *self == FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_128KB
    }
    #[doc = "192 KB of Flash"]
    #[inline(always)]
    pub fn is_flash_fsize_size_192kb(&self) -> bool {
        *self == FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_192KB
    }
    #[doc = "256 KB of Flash"]
    #[inline(always)]
    pub fn is_flash_fsize_size_256kb(&self) -> bool {
        *self == FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_256KB
    }
}
#[doc = "Field `FLASH_FSIZE_SIZE` writer - Flash Size"]
pub type FLASH_FSIZE_SIZE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, FLASH_FSIZE_SIZE_A>;
impl<'a, REG, const O: u8> FLASH_FSIZE_SIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "8 KB of Flash"]
    #[inline(always)]
    pub fn flash_fsize_size_8kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_8KB)
    }
    #[doc = "16 KB of Flash"]
    #[inline(always)]
    pub fn flash_fsize_size_16kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_16KB)
    }
    #[doc = "32 KB of Flash"]
    #[inline(always)]
    pub fn flash_fsize_size_32kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_32KB)
    }
    #[doc = "64 KB of Flash"]
    #[inline(always)]
    pub fn flash_fsize_size_64kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_64KB)
    }
    #[doc = "96 KB of Flash"]
    #[inline(always)]
    pub fn flash_fsize_size_96kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_96KB)
    }
    #[doc = "128 KB of Flash"]
    #[inline(always)]
    pub fn flash_fsize_size_128kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_128KB)
    }
    #[doc = "192 KB of Flash"]
    #[inline(always)]
    pub fn flash_fsize_size_192kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_192KB)
    }
    #[doc = "256 KB of Flash"]
    #[inline(always)]
    pub fn flash_fsize_size_256kb(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_FSIZE_SIZE_A::FLASH_FSIZE_SIZE_256KB)
    }
}
impl R {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn flash_fsize_size(&self) -> FLASH_FSIZE_SIZE_R {
        FLASH_FSIZE_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fsize_size(&mut self) -> FLASH_FSIZE_SIZE_W<FSIZE_SPEC, 0> {
        FLASH_FSIZE_SIZE_W::new(self)
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
#[doc = "Flash Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSIZE_SPEC;
impl crate::RegisterSpec for FSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsize::R`](R) reader structure"]
impl crate::Readable for FSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsize::W`](W) writer structure"]
impl crate::Writable for FSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSIZE to value 0"]
impl crate::Resettable for FSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
