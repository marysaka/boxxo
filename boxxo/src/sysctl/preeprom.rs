#[doc = "Register `PREEPROM` reader"]
pub type R = crate::R<PREEPROM_SPEC>;
#[doc = "Register `PREEPROM` writer"]
pub type W = crate::W<PREEPROM_SPEC>;
#[doc = "Field `SYSCTL_PREEPROM_R0` reader - EEPROM Module Peripheral Ready"]
pub type SYSCTL_PREEPROM_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PREEPROM_R0` writer - EEPROM Module Peripheral Ready"]
pub type SYSCTL_PREEPROM_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_preeprom_r0(&self) -> SYSCTL_PREEPROM_R0_R {
        SYSCTL_PREEPROM_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_preeprom_r0(&mut self) -> SYSCTL_PREEPROM_R0_W<PREEPROM_SPEC, 0> {
        SYSCTL_PREEPROM_R0_W::new(self)
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
#[doc = "EEPROM Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preeprom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preeprom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PREEPROM_SPEC;
impl crate::RegisterSpec for PREEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preeprom::R`](R) reader structure"]
impl crate::Readable for PREEPROM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`preeprom::W`](W) writer structure"]
impl crate::Writable for PREEPROM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PREEPROM to value 0"]
impl crate::Resettable for PREEPROM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
