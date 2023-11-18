#[doc = "Register `EEHIDE` reader"]
pub type R = crate::R<EEHIDE_SPEC>;
#[doc = "Register `EEHIDE` writer"]
pub type W = crate::W<EEHIDE_SPEC>;
#[doc = "Field `EEPROM_EEHIDE_HN` reader - Hide Block"]
pub type EEPROM_EEHIDE_HN_R = crate::FieldReader<u32>;
#[doc = "Field `EEPROM_EEHIDE_HN` writer - Hide Block"]
pub type EEPROM_EEHIDE_HN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
impl R {
    #[doc = "Bits 1:31 - Hide Block"]
    #[inline(always)]
    pub fn eeprom_eehide_hn(&self) -> EEPROM_EEHIDE_HN_R {
        EEPROM_EEHIDE_HN_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31 - Hide Block"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eehide_hn(&mut self) -> EEPROM_EEHIDE_HN_W<EEHIDE_SPEC, 1> {
        EEPROM_EEHIDE_HN_W::new(self)
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
#[doc = "EEPROM Block Hide\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eehide::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eehide::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEHIDE_SPEC;
impl crate::RegisterSpec for EEHIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eehide::R`](R) reader structure"]
impl crate::Readable for EEHIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eehide::W`](W) writer structure"]
impl crate::Writable for EEHIDE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEHIDE to value 0"]
impl crate::Resettable for EEHIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
