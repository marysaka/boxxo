#[doc = "Register `EEUNLOCK` reader"]
pub type R = crate::R<EEUNLOCK_SPEC>;
#[doc = "Register `EEUNLOCK` writer"]
pub type W = crate::W<EEUNLOCK_SPEC>;
#[doc = "Field `EEPROM_EEUNLOCK_UNLOCK` reader - EEPROM Unlock"]
pub type EEPROM_EEUNLOCK_UNLOCK_R = crate::FieldReader<u32>;
#[doc = "Field `EEPROM_EEUNLOCK_UNLOCK` writer - EEPROM Unlock"]
pub type EEPROM_EEUNLOCK_UNLOCK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - EEPROM Unlock"]
    #[inline(always)]
    pub fn eeprom_eeunlock_unlock(&self) -> EEPROM_EEUNLOCK_UNLOCK_R {
        EEPROM_EEUNLOCK_UNLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eeunlock_unlock(&mut self) -> EEPROM_EEUNLOCK_UNLOCK_W<EEUNLOCK_SPEC, 0> {
        EEPROM_EEUNLOCK_UNLOCK_W::new(self)
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
#[doc = "EEPROM Unlock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeunlock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeunlock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEUNLOCK_SPEC;
impl crate::RegisterSpec for EEUNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeunlock::R`](R) reader structure"]
impl crate::Readable for EEUNLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eeunlock::W`](W) writer structure"]
impl crate::Writable for EEUNLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEUNLOCK to value 0"]
impl crate::Resettable for EEUNLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
