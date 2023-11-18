#[doc = "Register `EEPASS0` reader"]
pub type R = crate::R<EEPASS0_SPEC>;
#[doc = "Register `EEPASS0` writer"]
pub type W = crate::W<EEPASS0_SPEC>;
#[doc = "Field `EEPROM_EEPASS0_PASS` reader - Password"]
pub type EEPROM_EEPASS0_PASS_R = crate::FieldReader<u32>;
#[doc = "Field `EEPROM_EEPASS0_PASS` writer - Password"]
pub type EEPROM_EEPASS0_PASS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Password"]
    #[inline(always)]
    pub fn eeprom_eepass0_pass(&self) -> EEPROM_EEPASS0_PASS_R {
        EEPROM_EEPASS0_PASS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Password"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eepass0_pass(&mut self) -> EEPROM_EEPASS0_PASS_W<EEPASS0_SPEC, 0> {
        EEPROM_EEPASS0_PASS_W::new(self)
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
#[doc = "EEPROM Password\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eepass0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eepass0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEPASS0_SPEC;
impl crate::RegisterSpec for EEPASS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eepass0::R`](R) reader structure"]
impl crate::Readable for EEPASS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eepass0::W`](W) writer structure"]
impl crate::Writable for EEPASS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEPASS0 to value 0"]
impl crate::Resettable for EEPASS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
