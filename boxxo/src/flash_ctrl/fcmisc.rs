#[doc = "Register `FCMISC` reader"]
pub type R = crate::R<FCMISC_SPEC>;
#[doc = "Register `FCMISC` writer"]
pub type W = crate::W<FCMISC_SPEC>;
#[doc = "Field `FLASH_FCMISC_AMISC` reader - Access Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_AMISC_R = crate::BitReader;
#[doc = "Field `FLASH_FCMISC_AMISC` writer - Access Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_AMISC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCMISC_PMISC` reader - Programming Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_PMISC_R = crate::BitReader;
#[doc = "Field `FLASH_FCMISC_PMISC` writer - Programming Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_PMISC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCMISC_EMISC` reader - EEPROM Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_EMISC_R = crate::BitReader;
#[doc = "Field `FLASH_FCMISC_EMISC` writer - EEPROM Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_EMISC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCMISC_VOLTMISC` reader - VOLT Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_VOLTMISC_R = crate::BitReader;
#[doc = "Field `FLASH_FCMISC_VOLTMISC` writer - VOLT Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_VOLTMISC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCMISC_INVDMISC` reader - Invalid Data Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_INVDMISC_R = crate::BitReader;
#[doc = "Field `FLASH_FCMISC_INVDMISC` writer - Invalid Data Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_INVDMISC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCMISC_ERMISC` reader - ERVER Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_ERMISC_R = crate::BitReader;
#[doc = "Field `FLASH_FCMISC_ERMISC` writer - ERVER Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_ERMISC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FCMISC_PROGMISC` reader - PROGVER Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_PROGMISC_R = crate::BitReader;
#[doc = "Field `FLASH_FCMISC_PROGMISC` writer - PROGVER Masked Interrupt Status and Clear"]
pub type FLASH_FCMISC_PROGMISC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Access Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_amisc(&self) -> FLASH_FCMISC_AMISC_R {
        FLASH_FCMISC_AMISC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programming Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_pmisc(&self) -> FLASH_FCMISC_PMISC_R {
        FLASH_FCMISC_PMISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEPROM Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_emisc(&self) -> FLASH_FCMISC_EMISC_R {
        FLASH_FCMISC_EMISC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - VOLT Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_voltmisc(&self) -> FLASH_FCMISC_VOLTMISC_R {
        FLASH_FCMISC_VOLTMISC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invalid Data Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_invdmisc(&self) -> FLASH_FCMISC_INVDMISC_R {
        FLASH_FCMISC_INVDMISC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ERVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_ermisc(&self) -> FLASH_FCMISC_ERMISC_R {
        FLASH_FCMISC_ERMISC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - PROGVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_progmisc(&self) -> FLASH_FCMISC_PROGMISC_R {
        FLASH_FCMISC_PROGMISC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Masked Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcmisc_amisc(&mut self) -> FLASH_FCMISC_AMISC_W<FCMISC_SPEC, 0> {
        FLASH_FCMISC_AMISC_W::new(self)
    }
    #[doc = "Bit 1 - Programming Masked Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcmisc_pmisc(&mut self) -> FLASH_FCMISC_PMISC_W<FCMISC_SPEC, 1> {
        FLASH_FCMISC_PMISC_W::new(self)
    }
    #[doc = "Bit 2 - EEPROM Masked Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcmisc_emisc(&mut self) -> FLASH_FCMISC_EMISC_W<FCMISC_SPEC, 2> {
        FLASH_FCMISC_EMISC_W::new(self)
    }
    #[doc = "Bit 9 - VOLT Masked Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcmisc_voltmisc(&mut self) -> FLASH_FCMISC_VOLTMISC_W<FCMISC_SPEC, 9> {
        FLASH_FCMISC_VOLTMISC_W::new(self)
    }
    #[doc = "Bit 10 - Invalid Data Masked Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcmisc_invdmisc(&mut self) -> FLASH_FCMISC_INVDMISC_W<FCMISC_SPEC, 10> {
        FLASH_FCMISC_INVDMISC_W::new(self)
    }
    #[doc = "Bit 11 - ERVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcmisc_ermisc(&mut self) -> FLASH_FCMISC_ERMISC_W<FCMISC_SPEC, 11> {
        FLASH_FCMISC_ERMISC_W::new(self)
    }
    #[doc = "Bit 13 - PROGVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fcmisc_progmisc(&mut self) -> FLASH_FCMISC_PROGMISC_W<FCMISC_SPEC, 13> {
        FLASH_FCMISC_PROGMISC_W::new(self)
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
#[doc = "Flash Controller Masked Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcmisc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcmisc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCMISC_SPEC;
impl crate::RegisterSpec for FCMISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcmisc::R`](R) reader structure"]
impl crate::Readable for FCMISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcmisc::W`](W) writer structure"]
impl crate::Writable for FCMISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCMISC to value 0"]
impl crate::Resettable for FCMISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
