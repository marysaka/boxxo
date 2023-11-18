#[doc = "Register `EESIZE` reader"]
pub type R = crate::R<EESIZE_SPEC>;
#[doc = "Register `EESIZE` writer"]
pub type W = crate::W<EESIZE_SPEC>;
#[doc = "Field `EEPROM_EESIZE_WORDCNT` reader - Number of 32-Bit Words"]
pub type EEPROM_EESIZE_WORDCNT_R = crate::FieldReader<u16>;
#[doc = "Field `EEPROM_EESIZE_WORDCNT` writer - Number of 32-Bit Words"]
pub type EEPROM_EESIZE_WORDCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `EEPROM_EESIZE_BLKCNT` reader - Number of 16-Word Blocks"]
pub type EEPROM_EESIZE_BLKCNT_R = crate::FieldReader<u16>;
#[doc = "Field `EEPROM_EESIZE_BLKCNT` writer - Number of 16-Word Blocks"]
pub type EEPROM_EESIZE_BLKCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of 32-Bit Words"]
    #[inline(always)]
    pub fn eeprom_eesize_wordcnt(&self) -> EEPROM_EESIZE_WORDCNT_R {
        EEPROM_EESIZE_WORDCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - Number of 16-Word Blocks"]
    #[inline(always)]
    pub fn eeprom_eesize_blkcnt(&self) -> EEPROM_EESIZE_BLKCNT_R {
        EEPROM_EESIZE_BLKCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of 32-Bit Words"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eesize_wordcnt(&mut self) -> EEPROM_EESIZE_WORDCNT_W<EESIZE_SPEC, 0> {
        EEPROM_EESIZE_WORDCNT_W::new(self)
    }
    #[doc = "Bits 16:26 - Number of 16-Word Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eesize_blkcnt(&mut self) -> EEPROM_EESIZE_BLKCNT_W<EESIZE_SPEC, 16> {
        EEPROM_EESIZE_BLKCNT_W::new(self)
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
#[doc = "EEPROM Size Information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eesize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eesize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EESIZE_SPEC;
impl crate::RegisterSpec for EESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eesize::R`](R) reader structure"]
impl crate::Readable for EESIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eesize::W`](W) writer structure"]
impl crate::Writable for EESIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EESIZE to value 0"]
impl crate::Resettable for EESIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
