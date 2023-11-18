#[doc = "Register `PPEEPROM` reader"]
pub type R = crate::R<PPEEPROM_SPEC>;
#[doc = "Register `PPEEPROM` writer"]
pub type W = crate::W<PPEEPROM_SPEC>;
#[doc = "Field `SYSCTL_PPEEPROM_P0` reader - EEPROM Module Present"]
pub type SYSCTL_PPEEPROM_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPEEPROM_P0` writer - EEPROM Module Present"]
pub type SYSCTL_PPEEPROM_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Present"]
    #[inline(always)]
    pub fn sysctl_ppeeprom_p0(&self) -> SYSCTL_PPEEPROM_P0_R {
        SYSCTL_PPEEPROM_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppeeprom_p0(&mut self) -> SYSCTL_PPEEPROM_P0_W<PPEEPROM_SPEC, 0> {
        SYSCTL_PPEEPROM_P0_W::new(self)
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
#[doc = "EEPROM Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppeeprom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppeeprom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPEEPROM_SPEC;
impl crate::RegisterSpec for PPEEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppeeprom::R`](R) reader structure"]
impl crate::Readable for PPEEPROM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppeeprom::W`](W) writer structure"]
impl crate::Writable for PPEEPROM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPEEPROM to value 0"]
impl crate::Resettable for PPEEPROM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
