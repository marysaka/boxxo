#[doc = "Register `EERDWRINC` reader"]
pub type R = crate::R<EERDWRINC_SPEC>;
#[doc = "Register `EERDWRINC` writer"]
pub type W = crate::W<EERDWRINC_SPEC>;
#[doc = "Field `EEPROM_EERDWRINC_VALUE` reader - EEPROM Read or Write Data with Increment"]
pub type EEPROM_EERDWRINC_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `EEPROM_EERDWRINC_VALUE` writer - EEPROM Read or Write Data with Increment"]
pub type EEPROM_EERDWRINC_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - EEPROM Read or Write Data with Increment"]
    #[inline(always)]
    pub fn eeprom_eerdwrinc_value(&self) -> EEPROM_EERDWRINC_VALUE_R {
        EEPROM_EERDWRINC_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Read or Write Data with Increment"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eerdwrinc_value(&mut self) -> EEPROM_EERDWRINC_VALUE_W<EERDWRINC_SPEC, 0> {
        EEPROM_EERDWRINC_VALUE_W::new(self)
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
#[doc = "EEPROM Read-Write with Increment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eerdwrinc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eerdwrinc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EERDWRINC_SPEC;
impl crate::RegisterSpec for EERDWRINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eerdwrinc::R`](R) reader structure"]
impl crate::Readable for EERDWRINC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eerdwrinc::W`](W) writer structure"]
impl crate::Writable for EERDWRINC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EERDWRINC to value 0"]
impl crate::Resettable for EERDWRINC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
