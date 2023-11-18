#[doc = "Register `EEOFFSET` reader"]
pub type R = crate::R<EEOFFSET_SPEC>;
#[doc = "Register `EEOFFSET` writer"]
pub type W = crate::W<EEOFFSET_SPEC>;
#[doc = "Field `EEPROM_EEOFFSET_OFFSET` reader - Current Address Offset"]
pub type EEPROM_EEOFFSET_OFFSET_R = crate::FieldReader;
#[doc = "Field `EEPROM_EEOFFSET_OFFSET` writer - Current Address Offset"]
pub type EEPROM_EEOFFSET_OFFSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Current Address Offset"]
    #[inline(always)]
    pub fn eeprom_eeoffset_offset(&self) -> EEPROM_EEOFFSET_OFFSET_R {
        EEPROM_EEOFFSET_OFFSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Current Address Offset"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eeoffset_offset(&mut self) -> EEPROM_EEOFFSET_OFFSET_W<EEOFFSET_SPEC, 0> {
        EEPROM_EEOFFSET_OFFSET_W::new(self)
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
#[doc = "EEPROM Current Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeoffset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeoffset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEOFFSET_SPEC;
impl crate::RegisterSpec for EEOFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeoffset::R`](R) reader structure"]
impl crate::Readable for EEOFFSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eeoffset::W`](W) writer structure"]
impl crate::Writable for EEOFFSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEOFFSET to value 0"]
impl crate::Resettable for EEOFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
