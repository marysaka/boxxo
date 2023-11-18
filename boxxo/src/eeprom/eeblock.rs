#[doc = "Register `EEBLOCK` reader"]
pub type R = crate::R<EEBLOCK_SPEC>;
#[doc = "Register `EEBLOCK` writer"]
pub type W = crate::W<EEBLOCK_SPEC>;
#[doc = "Field `EEPROM_EEBLOCK_BLOCK` reader - Current Block"]
pub type EEPROM_EEBLOCK_BLOCK_R = crate::FieldReader<u16>;
#[doc = "Field `EEPROM_EEBLOCK_BLOCK` writer - Current Block"]
pub type EEPROM_EEBLOCK_BLOCK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Current Block"]
    #[inline(always)]
    pub fn eeprom_eeblock_block(&self) -> EEPROM_EEBLOCK_BLOCK_R {
        EEPROM_EEBLOCK_BLOCK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current Block"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eeblock_block(&mut self) -> EEPROM_EEBLOCK_BLOCK_W<EEBLOCK_SPEC, 0> {
        EEPROM_EEBLOCK_BLOCK_W::new(self)
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
#[doc = "EEPROM Current Block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeblock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeblock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEBLOCK_SPEC;
impl crate::RegisterSpec for EEBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeblock::R`](R) reader structure"]
impl crate::Readable for EEBLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eeblock::W`](W) writer structure"]
impl crate::Writable for EEBLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEBLOCK to value 0"]
impl crate::Resettable for EEBLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
