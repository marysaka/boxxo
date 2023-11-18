#[doc = "Register `SCGCEEPROM` reader"]
pub type R = crate::R<SCGCEEPROM_SPEC>;
#[doc = "Register `SCGCEEPROM` writer"]
pub type W = crate::W<SCGCEEPROM_SPEC>;
#[doc = "Field `SYSCTL_SCGCEEPROM_S0` reader - EEPROM Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCEEPROM_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCEEPROM_S0` writer - EEPROM Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCEEPROM_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgceeprom_s0(&self) -> SYSCTL_SCGCEEPROM_S0_R {
        SYSCTL_SCGCEEPROM_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgceeprom_s0(&mut self) -> SYSCTL_SCGCEEPROM_S0_W<SCGCEEPROM_SPEC, 0> {
        SYSCTL_SCGCEEPROM_S0_W::new(self)
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
#[doc = "EEPROM Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgceeprom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgceeprom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCEEPROM_SPEC;
impl crate::RegisterSpec for SCGCEEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgceeprom::R`](R) reader structure"]
impl crate::Readable for SCGCEEPROM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgceeprom::W`](W) writer structure"]
impl crate::Writable for SCGCEEPROM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCEEPROM to value 0"]
impl crate::Resettable for SCGCEEPROM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
