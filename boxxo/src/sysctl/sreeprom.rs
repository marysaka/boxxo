#[doc = "Register `SREEPROM` reader"]
pub type R = crate::R<SREEPROM_SPEC>;
#[doc = "Register `SREEPROM` writer"]
pub type W = crate::W<SREEPROM_SPEC>;
#[doc = "Field `SYSCTL_SREEPROM_R0` reader - EEPROM Module Software Reset"]
pub type SYSCTL_SREEPROM_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SREEPROM_R0` writer - EEPROM Module Software Reset"]
pub type SYSCTL_SREEPROM_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_sreeprom_r0(&self) -> SYSCTL_SREEPROM_R0_R {
        SYSCTL_SREEPROM_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_sreeprom_r0(&mut self) -> SYSCTL_SREEPROM_R0_W<SREEPROM_SPEC, 0> {
        SYSCTL_SREEPROM_R0_W::new(self)
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
#[doc = "EEPROM Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sreeprom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sreeprom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SREEPROM_SPEC;
impl crate::RegisterSpec for SREEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sreeprom::R`](R) reader structure"]
impl crate::Readable for SREEPROM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sreeprom::W`](W) writer structure"]
impl crate::Writable for SREEPROM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SREEPROM to value 0"]
impl crate::Resettable for SREEPROM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
