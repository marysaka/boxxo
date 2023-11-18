#[doc = "Register `FCIM` reader"]
pub type R = crate::R<FCIM_SPEC>;
#[doc = "Register `FCIM` writer"]
pub type W = crate::W<FCIM_SPEC>;
#[doc = "Field `FLASH_FCIM_AMASK` reader - Access Interrupt Mask"]
pub type FLASH_FCIM_AMASK_R = crate::BitReader;
#[doc = "Field `FLASH_FCIM_AMASK` writer - Access Interrupt Mask"]
pub type FLASH_FCIM_AMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCIM_PMASK` reader - Programming Interrupt Mask"]
pub type FLASH_FCIM_PMASK_R = crate::BitReader;
#[doc = "Field `FLASH_FCIM_PMASK` writer - Programming Interrupt Mask"]
pub type FLASH_FCIM_PMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCIM_EMASK` reader - EEPROM Interrupt Mask"]
pub type FLASH_FCIM_EMASK_R = crate::BitReader;
#[doc = "Field `FLASH_FCIM_EMASK` writer - EEPROM Interrupt Mask"]
pub type FLASH_FCIM_EMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCIM_VOLTMASK` reader - VOLT Interrupt Mask"]
pub type FLASH_FCIM_VOLTMASK_R = crate::BitReader;
#[doc = "Field `FLASH_FCIM_VOLTMASK` writer - VOLT Interrupt Mask"]
pub type FLASH_FCIM_VOLTMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCIM_INVDMASK` reader - Invalid Data Interrupt Mask"]
pub type FLASH_FCIM_INVDMASK_R = crate::BitReader;
#[doc = "Field `FLASH_FCIM_INVDMASK` writer - Invalid Data Interrupt Mask"]
pub type FLASH_FCIM_INVDMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCIM_ERMASK` reader - ERVER Interrupt Mask"]
pub type FLASH_FCIM_ERMASK_R = crate::BitReader;
#[doc = "Field `FLASH_FCIM_ERMASK` writer - ERVER Interrupt Mask"]
pub type FLASH_FCIM_ERMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCIM_PROGMASK` reader - PROGVER Interrupt Mask"]
pub type FLASH_FCIM_PROGMASK_R = crate::BitReader;
#[doc = "Field `FLASH_FCIM_PROGMASK` writer - PROGVER Interrupt Mask"]
pub type FLASH_FCIM_PROGMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_amask(&self) -> FLASH_FCIM_AMASK_R {
        FLASH_FCIM_AMASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_pmask(&self) -> FLASH_FCIM_PMASK_R {
        FLASH_FCIM_PMASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEPROM Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_emask(&self) -> FLASH_FCIM_EMASK_R {
        FLASH_FCIM_EMASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - VOLT Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_voltmask(&self) -> FLASH_FCIM_VOLTMASK_R {
        FLASH_FCIM_VOLTMASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invalid Data Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_invdmask(&self) -> FLASH_FCIM_INVDMASK_R {
        FLASH_FCIM_INVDMASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ERVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_ermask(&self) -> FLASH_FCIM_ERMASK_R {
        FLASH_FCIM_ERMASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - PROGVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_progmask(&self) -> FLASH_FCIM_PROGMASK_R {
        FLASH_FCIM_PROGMASK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcim_amask(&mut self) -> FLASH_FCIM_AMASK_W<FCIM_SPEC, 0> {
        FLASH_FCIM_AMASK_W::new(self)
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcim_pmask(&mut self) -> FLASH_FCIM_PMASK_W<FCIM_SPEC, 1> {
        FLASH_FCIM_PMASK_W::new(self)
    }
    #[doc = "Bit 2 - EEPROM Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcim_emask(&mut self) -> FLASH_FCIM_EMASK_W<FCIM_SPEC, 2> {
        FLASH_FCIM_EMASK_W::new(self)
    }
    #[doc = "Bit 9 - VOLT Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcim_voltmask(&mut self) -> FLASH_FCIM_VOLTMASK_W<FCIM_SPEC, 9> {
        FLASH_FCIM_VOLTMASK_W::new(self)
    }
    #[doc = "Bit 10 - Invalid Data Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcim_invdmask(&mut self) -> FLASH_FCIM_INVDMASK_W<FCIM_SPEC, 10> {
        FLASH_FCIM_INVDMASK_W::new(self)
    }
    #[doc = "Bit 11 - ERVER Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcim_ermask(&mut self) -> FLASH_FCIM_ERMASK_W<FCIM_SPEC, 11> {
        FLASH_FCIM_ERMASK_W::new(self)
    }
    #[doc = "Bit 13 - PROGVER Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcim_progmask(&mut self) -> FLASH_FCIM_PROGMASK_W<FCIM_SPEC, 13> {
        FLASH_FCIM_PROGMASK_W::new(self)
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
#[doc = "Flash Controller Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCIM_SPEC;
impl crate::RegisterSpec for FCIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcim::R`](R) reader structure"]
impl crate::Readable for FCIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcim::W`](W) writer structure"]
impl crate::Writable for FCIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCIM to value 0"]
impl crate::Resettable for FCIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
