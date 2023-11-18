#[doc = "Register `EESUPP` reader"]
pub type R = crate::R<EESUPP_SPEC>;
#[doc = "Register `EESUPP` writer"]
pub type W = crate::W<EESUPP_SPEC>;
#[doc = "Field `EEPROM_EESUPP_START` reader - Start Erase"]
pub type EEPROM_EESUPP_START_R = crate::BitReader;
#[doc = "Field `EEPROM_EESUPP_START` writer - Start Erase"]
pub type EEPROM_EESUPP_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EEPROM_EESUPP_EREQ` reader - Erase Required"]
pub type EEPROM_EESUPP_EREQ_R = crate::BitReader;
#[doc = "Field `EEPROM_EESUPP_EREQ` writer - Erase Required"]
pub type EEPROM_EESUPP_EREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EEPROM_EESUPP_ERETRY` reader - Erase Must Be Retried"]
pub type EEPROM_EESUPP_ERETRY_R = crate::BitReader;
#[doc = "Field `EEPROM_EESUPP_ERETRY` writer - Erase Must Be Retried"]
pub type EEPROM_EESUPP_ERETRY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EEPROM_EESUPP_PRETRY` reader - Programming Must Be Retried"]
pub type EEPROM_EESUPP_PRETRY_R = crate::BitReader;
#[doc = "Field `EEPROM_EESUPP_PRETRY` writer - Programming Must Be Retried"]
pub type EEPROM_EESUPP_PRETRY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Start Erase"]
    #[inline(always)]
    pub fn eeprom_eesupp_start(&self) -> EEPROM_EESUPP_START_R {
        EEPROM_EESUPP_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Erase Required"]
    #[inline(always)]
    pub fn eeprom_eesupp_ereq(&self) -> EEPROM_EESUPP_EREQ_R {
        EEPROM_EESUPP_EREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Erase Must Be Retried"]
    #[inline(always)]
    pub fn eeprom_eesupp_eretry(&self) -> EEPROM_EESUPP_ERETRY_R {
        EEPROM_EESUPP_ERETRY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming Must Be Retried"]
    #[inline(always)]
    pub fn eeprom_eesupp_pretry(&self) -> EEPROM_EESUPP_PRETRY_R {
        EEPROM_EESUPP_PRETRY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Erase"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eesupp_start(&mut self) -> EEPROM_EESUPP_START_W<EESUPP_SPEC, 0> {
        EEPROM_EESUPP_START_W::new(self)
    }
    #[doc = "Bit 1 - Erase Required"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eesupp_ereq(&mut self) -> EEPROM_EESUPP_EREQ_W<EESUPP_SPEC, 1> {
        EEPROM_EESUPP_EREQ_W::new(self)
    }
    #[doc = "Bit 2 - Erase Must Be Retried"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eesupp_eretry(&mut self) -> EEPROM_EESUPP_ERETRY_W<EESUPP_SPEC, 2> {
        EEPROM_EESUPP_ERETRY_W::new(self)
    }
    #[doc = "Bit 3 - Programming Must Be Retried"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eesupp_pretry(&mut self) -> EEPROM_EESUPP_PRETRY_W<EESUPP_SPEC, 3> {
        EEPROM_EESUPP_PRETRY_W::new(self)
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
#[doc = "EEPROM Support Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eesupp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eesupp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EESUPP_SPEC;
impl crate::RegisterSpec for EESUPP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eesupp::R`](R) reader structure"]
impl crate::Readable for EESUPP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eesupp::W`](W) writer structure"]
impl crate::Writable for EESUPP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EESUPP to value 0"]
impl crate::Resettable for EESUPP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
