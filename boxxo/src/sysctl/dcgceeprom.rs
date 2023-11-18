#[doc = "Register `DCGCEEPROM` reader"]
pub type R = crate::R<DCGCEEPROM_SPEC>;
#[doc = "Register `DCGCEEPROM` writer"]
pub type W = crate::W<DCGCEEPROM_SPEC>;
#[doc = "Field `SYSCTL_DCGCEEPROM_D0` reader - EEPROM Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCEEPROM_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCEEPROM_D0` writer - EEPROM Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCEEPROM_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgceeprom_d0(&self) -> SYSCTL_DCGCEEPROM_D0_R {
        SYSCTL_DCGCEEPROM_D0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgceeprom_d0(&mut self) -> SYSCTL_DCGCEEPROM_D0_W<DCGCEEPROM_SPEC, 0> {
        SYSCTL_DCGCEEPROM_D0_W::new(self)
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
#[doc = "EEPROM Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgceeprom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgceeprom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCEEPROM_SPEC;
impl crate::RegisterSpec for DCGCEEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgceeprom::R`](R) reader structure"]
impl crate::Readable for DCGCEEPROM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgceeprom::W`](W) writer structure"]
impl crate::Writable for DCGCEEPROM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCEEPROM to value 0"]
impl crate::Resettable for DCGCEEPROM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
