#[doc = "Register `EEDONE` reader"]
pub type R = crate::R<EEDONE_SPEC>;
#[doc = "Register `EEDONE` writer"]
pub type W = crate::W<EEDONE_SPEC>;
#[doc = "Field `EEPROM_EEDONE_WORKING` reader - EEPROM Working"]
pub type EEPROM_EEDONE_WORKING_R = crate::BitReader;
#[doc = "Field `EEPROM_EEDONE_WORKING` writer - EEPROM Working"]
pub type EEPROM_EEDONE_WORKING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EEPROM_EEDONE_WKERASE` reader - Working on an Erase"]
pub type EEPROM_EEDONE_WKERASE_R = crate::BitReader;
#[doc = "Field `EEPROM_EEDONE_WKERASE` writer - Working on an Erase"]
pub type EEPROM_EEDONE_WKERASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EEPROM_EEDONE_WKCOPY` reader - Working on a Copy"]
pub type EEPROM_EEDONE_WKCOPY_R = crate::BitReader;
#[doc = "Field `EEPROM_EEDONE_WKCOPY` writer - Working on a Copy"]
pub type EEPROM_EEDONE_WKCOPY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EEPROM_EEDONE_NOPERM` reader - Write Without Permission"]
pub type EEPROM_EEDONE_NOPERM_R = crate::BitReader;
#[doc = "Field `EEPROM_EEDONE_NOPERM` writer - Write Without Permission"]
pub type EEPROM_EEDONE_NOPERM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EEPROM_EEDONE_WRBUSY` reader - Write Busy"]
pub type EEPROM_EEDONE_WRBUSY_R = crate::BitReader;
#[doc = "Field `EEPROM_EEDONE_WRBUSY` writer - Write Busy"]
pub type EEPROM_EEDONE_WRBUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EEPROM_EEDONE_INVPL` reader - Invalid Program Voltage Level"]
pub type EEPROM_EEDONE_INVPL_R = crate::BitReader;
#[doc = "Field `EEPROM_EEDONE_INVPL` writer - Invalid Program Voltage Level"]
pub type EEPROM_EEDONE_INVPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - EEPROM Working"]
    #[inline(always)]
    pub fn eeprom_eedone_working(&self) -> EEPROM_EEDONE_WORKING_R {
        EEPROM_EEDONE_WORKING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Working on an Erase"]
    #[inline(always)]
    pub fn eeprom_eedone_wkerase(&self) -> EEPROM_EEDONE_WKERASE_R {
        EEPROM_EEDONE_WKERASE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Working on a Copy"]
    #[inline(always)]
    pub fn eeprom_eedone_wkcopy(&self) -> EEPROM_EEDONE_WKCOPY_R {
        EEPROM_EEDONE_WKCOPY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Without Permission"]
    #[inline(always)]
    pub fn eeprom_eedone_noperm(&self) -> EEPROM_EEDONE_NOPERM_R {
        EEPROM_EEDONE_NOPERM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn eeprom_eedone_wrbusy(&self) -> EEPROM_EEDONE_WRBUSY_R {
        EEPROM_EEDONE_WRBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Invalid Program Voltage Level"]
    #[inline(always)]
    pub fn eeprom_eedone_invpl(&self) -> EEPROM_EEDONE_INVPL_R {
        EEPROM_EEDONE_INVPL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Working"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eedone_working(&mut self) -> EEPROM_EEDONE_WORKING_W<EEDONE_SPEC, 0> {
        EEPROM_EEDONE_WORKING_W::new(self)
    }
    #[doc = "Bit 2 - Working on an Erase"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eedone_wkerase(&mut self) -> EEPROM_EEDONE_WKERASE_W<EEDONE_SPEC, 2> {
        EEPROM_EEDONE_WKERASE_W::new(self)
    }
    #[doc = "Bit 3 - Working on a Copy"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eedone_wkcopy(&mut self) -> EEPROM_EEDONE_WKCOPY_W<EEDONE_SPEC, 3> {
        EEPROM_EEDONE_WKCOPY_W::new(self)
    }
    #[doc = "Bit 4 - Write Without Permission"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eedone_noperm(&mut self) -> EEPROM_EEDONE_NOPERM_W<EEDONE_SPEC, 4> {
        EEPROM_EEDONE_NOPERM_W::new(self)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eedone_wrbusy(&mut self) -> EEPROM_EEDONE_WRBUSY_W<EEDONE_SPEC, 5> {
        EEPROM_EEDONE_WRBUSY_W::new(self)
    }
    #[doc = "Bit 8 - Invalid Program Voltage Level"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eedone_invpl(&mut self) -> EEPROM_EEDONE_INVPL_W<EEDONE_SPEC, 8> {
        EEPROM_EEDONE_INVPL_W::new(self)
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
#[doc = "EEPROM Done Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eedone::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eedone::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEDONE_SPEC;
impl crate::RegisterSpec for EEDONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eedone::R`](R) reader structure"]
impl crate::Readable for EEDONE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eedone::W`](W) writer structure"]
impl crate::Writable for EEDONE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEDONE to value 0"]
impl crate::Resettable for EEDONE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
