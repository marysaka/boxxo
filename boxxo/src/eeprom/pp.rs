#[doc = "Register `PP` reader"]
pub type R = crate::R<PP_SPEC>;
#[doc = "Register `PP` writer"]
pub type W = crate::W<PP_SPEC>;
#[doc = "Field `EEPROM_PP_SIZE` reader - EEPROM Size"]
pub type EEPROM_PP_SIZE_R = crate::FieldReader;
#[doc = "Field `EEPROM_PP_SIZE` writer - EEPROM Size"]
pub type EEPROM_PP_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - EEPROM Size"]
    #[inline(always)]
    pub fn eeprom_pp_size(&self) -> EEPROM_PP_SIZE_R {
        EEPROM_PP_SIZE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - EEPROM Size"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_pp_size(&mut self) -> EEPROM_PP_SIZE_W<PP_SPEC, 0> {
        EEPROM_PP_SIZE_W::new(self)
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
#[doc = "EEPROM Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp::R`](R) reader structure"]
impl crate::Readable for PP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pp::W`](W) writer structure"]
impl crate::Writable for PP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
