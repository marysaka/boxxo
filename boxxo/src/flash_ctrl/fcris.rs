#[doc = "Register `FCRIS` reader"]
pub type R = crate::R<FCRIS_SPEC>;
#[doc = "Register `FCRIS` writer"]
pub type W = crate::W<FCRIS_SPEC>;
#[doc = "Field `FLASH_FCRIS_ARIS` reader - Access Raw Interrupt Status"]
pub type FLASH_FCRIS_ARIS_R = crate::BitReader;
#[doc = "Field `FLASH_FCRIS_ARIS` writer - Access Raw Interrupt Status"]
pub type FLASH_FCRIS_ARIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCRIS_PRIS` reader - Programming Raw Interrupt Status"]
pub type FLASH_FCRIS_PRIS_R = crate::BitReader;
#[doc = "Field `FLASH_FCRIS_PRIS` writer - Programming Raw Interrupt Status"]
pub type FLASH_FCRIS_PRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCRIS_ERIS` reader - EEPROM Raw Interrupt Status"]
pub type FLASH_FCRIS_ERIS_R = crate::BitReader;
#[doc = "Field `FLASH_FCRIS_ERIS` writer - EEPROM Raw Interrupt Status"]
pub type FLASH_FCRIS_ERIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCRIS_VOLTRIS` reader - VOLTSTAT Raw Interrupt Status"]
pub type FLASH_FCRIS_VOLTRIS_R = crate::BitReader;
#[doc = "Field `FLASH_FCRIS_VOLTRIS` writer - VOLTSTAT Raw Interrupt Status"]
pub type FLASH_FCRIS_VOLTRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCRIS_INVDRIS` reader - Invalid Data Raw Interrupt Status"]
pub type FLASH_FCRIS_INVDRIS_R = crate::BitReader;
#[doc = "Field `FLASH_FCRIS_INVDRIS` writer - Invalid Data Raw Interrupt Status"]
pub type FLASH_FCRIS_INVDRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCRIS_ERRIS` reader - ERVER Raw Interrupt Status"]
pub type FLASH_FCRIS_ERRIS_R = crate::BitReader;
#[doc = "Field `FLASH_FCRIS_ERRIS` writer - ERVER Raw Interrupt Status"]
pub type FLASH_FCRIS_ERRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCRIS_PROGRIS` reader - PROGVER Raw Interrupt Status"]
pub type FLASH_FCRIS_PROGRIS_R = crate::BitReader;
#[doc = "Field `FLASH_FCRIS_PROGRIS` writer - PROGVER Raw Interrupt Status"]
pub type FLASH_FCRIS_PROGRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Access Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_aris(&self) -> FLASH_FCRIS_ARIS_R {
        FLASH_FCRIS_ARIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programming Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_pris(&self) -> FLASH_FCRIS_PRIS_R {
        FLASH_FCRIS_PRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEPROM Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_eris(&self) -> FLASH_FCRIS_ERIS_R {
        FLASH_FCRIS_ERIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - VOLTSTAT Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_voltris(&self) -> FLASH_FCRIS_VOLTRIS_R {
        FLASH_FCRIS_VOLTRIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invalid Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_invdris(&self) -> FLASH_FCRIS_INVDRIS_R {
        FLASH_FCRIS_INVDRIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ERVER Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_erris(&self) -> FLASH_FCRIS_ERRIS_R {
        FLASH_FCRIS_ERRIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - PROGVER Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_progris(&self) -> FLASH_FCRIS_PROGRIS_R {
        FLASH_FCRIS_PROGRIS_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcris_aris(&mut self) -> FLASH_FCRIS_ARIS_W<FCRIS_SPEC, 0> {
        FLASH_FCRIS_ARIS_W::new(self)
    }
    #[doc = "Bit 1 - Programming Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcris_pris(&mut self) -> FLASH_FCRIS_PRIS_W<FCRIS_SPEC, 1> {
        FLASH_FCRIS_PRIS_W::new(self)
    }
    #[doc = "Bit 2 - EEPROM Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcris_eris(&mut self) -> FLASH_FCRIS_ERIS_W<FCRIS_SPEC, 2> {
        FLASH_FCRIS_ERIS_W::new(self)
    }
    #[doc = "Bit 9 - VOLTSTAT Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcris_voltris(&mut self) -> FLASH_FCRIS_VOLTRIS_W<FCRIS_SPEC, 9> {
        FLASH_FCRIS_VOLTRIS_W::new(self)
    }
    #[doc = "Bit 10 - Invalid Data Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcris_invdris(&mut self) -> FLASH_FCRIS_INVDRIS_W<FCRIS_SPEC, 10> {
        FLASH_FCRIS_INVDRIS_W::new(self)
    }
    #[doc = "Bit 11 - ERVER Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcris_erris(&mut self) -> FLASH_FCRIS_ERRIS_W<FCRIS_SPEC, 11> {
        FLASH_FCRIS_ERRIS_W::new(self)
    }
    #[doc = "Bit 13 - PROGVER Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcris_progris(&mut self) -> FLASH_FCRIS_PROGRIS_W<FCRIS_SPEC, 13> {
        FLASH_FCRIS_PROGRIS_W::new(self)
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
#[doc = "Flash Controller Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCRIS_SPEC;
impl crate::RegisterSpec for FCRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcris::R`](R) reader structure"]
impl crate::Readable for FCRIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcris::W`](W) writer structure"]
impl crate::Writable for FCRIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCRIS to value 0"]
impl crate::Resettable for FCRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
