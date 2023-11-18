#[doc = "Register `EEDBGME` reader"]
pub type R = crate::R<EEDBGME_SPEC>;
#[doc = "Register `EEDBGME` writer"]
pub type W = crate::W<EEDBGME_SPEC>;
#[doc = "Field `EEPROM_EEDBGME_ME` reader - Mass Erase"]
pub type EEPROM_EEDBGME_ME_R = crate::BitReader;
#[doc = "Field `EEPROM_EEDBGME_ME` writer - Mass Erase"]
pub type EEPROM_EEDBGME_ME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EEPROM_EEDBGME_KEY` reader - Erase Key"]
pub type EEPROM_EEDBGME_KEY_R = crate::FieldReader<u16>;
#[doc = "Field `EEPROM_EEDBGME_KEY` writer - Erase Key"]
pub type EEPROM_EEDBGME_KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Mass Erase"]
    #[inline(always)]
    pub fn eeprom_eedbgme_me(&self) -> EEPROM_EEDBGME_ME_R {
        EEPROM_EEDBGME_ME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - Erase Key"]
    #[inline(always)]
    pub fn eeprom_eedbgme_key(&self) -> EEPROM_EEDBGME_KEY_R {
        EEPROM_EEDBGME_KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Mass Erase"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eedbgme_me(&mut self) -> EEPROM_EEDBGME_ME_W<EEDBGME_SPEC, 0> {
        EEPROM_EEDBGME_ME_W::new(self)
    }
    #[doc = "Bits 16:31 - Erase Key"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eedbgme_key(&mut self) -> EEPROM_EEDBGME_KEY_W<EEDBGME_SPEC, 16> {
        EEPROM_EEDBGME_KEY_W::new(self)
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
#[doc = "EEPROM Debug Mass Erase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eedbgme::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eedbgme::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEDBGME_SPEC;
impl crate::RegisterSpec for EEDBGME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eedbgme::R`](R) reader structure"]
impl crate::Readable for EEDBGME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eedbgme::W`](W) writer structure"]
impl crate::Writable for EEDBGME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEDBGME to value 0"]
impl crate::Resettable for EEDBGME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
