#[doc = "Register `EEINT` reader"]
pub type R = crate::R<EEINT_SPEC>;
#[doc = "Register `EEINT` writer"]
pub type W = crate::W<EEINT_SPEC>;
#[doc = "Field `EEPROM_EEINT_INT` reader - Interrupt Enable"]
pub type EEPROM_EEINT_INT_R = crate::BitReader;
#[doc = "Field `EEPROM_EEINT_INT` writer - Interrupt Enable"]
pub type EEPROM_EEINT_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn eeprom_eeint_int(&self) -> EEPROM_EEINT_INT_R {
        EEPROM_EEINT_INT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eeint_int(&mut self) -> EEPROM_EEINT_INT_W<EEINT_SPEC, 0> {
        EEPROM_EEINT_INT_W::new(self)
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
#[doc = "EEPROM Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEINT_SPEC;
impl crate::RegisterSpec for EEINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeint::R`](R) reader structure"]
impl crate::Readable for EEINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eeint::W`](W) writer structure"]
impl crate::Writable for EEINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEINT to value 0"]
impl crate::Resettable for EEINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
