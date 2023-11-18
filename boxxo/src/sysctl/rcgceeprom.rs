#[doc = "Register `RCGCEEPROM` reader"]
pub type R = crate::R<RCGCEEPROM_SPEC>;
#[doc = "Register `RCGCEEPROM` writer"]
pub type W = crate::W<RCGCEEPROM_SPEC>;
#[doc = "Field `SYSCTL_RCGCEEPROM_R0` reader - EEPROM Module Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCEEPROM_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCEEPROM_R0` writer - EEPROM Module Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCEEPROM_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgceeprom_r0(&self) -> SYSCTL_RCGCEEPROM_R0_R {
        SYSCTL_RCGCEEPROM_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgceeprom_r0(&mut self) -> SYSCTL_RCGCEEPROM_R0_W<RCGCEEPROM_SPEC, 0> {
        SYSCTL_RCGCEEPROM_R0_W::new(self)
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
#[doc = "EEPROM Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgceeprom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgceeprom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGCEEPROM_SPEC;
impl crate::RegisterSpec for RCGCEEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgceeprom::R`](R) reader structure"]
impl crate::Readable for RCGCEEPROM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgceeprom::W`](W) writer structure"]
impl crate::Writable for RCGCEEPROM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCEEPROM to value 0"]
impl crate::Resettable for RCGCEEPROM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
