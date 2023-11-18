#[doc = "Register `SCGCHIB` reader"]
pub type R = crate::R<SCGCHIB_SPEC>;
#[doc = "Register `SCGCHIB` writer"]
pub type W = crate::W<SCGCHIB_SPEC>;
#[doc = "Field `SYSCTL_SCGCHIB_S0` reader - Hibernation Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCHIB_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCHIB_S0` writer - Hibernation Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCHIB_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Hibernation Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgchib_s0(&self) -> SYSCTL_SCGCHIB_S0_R {
        SYSCTL_SCGCHIB_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernation Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgchib_s0(&mut self) -> SYSCTL_SCGCHIB_S0_W<SCGCHIB_SPEC, 0> {
        SYSCTL_SCGCHIB_S0_W::new(self)
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
#[doc = "Hibernation Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgchib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgchib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCHIB_SPEC;
impl crate::RegisterSpec for SCGCHIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgchib::R`](R) reader structure"]
impl crate::Readable for SCGCHIB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgchib::W`](W) writer structure"]
impl crate::Writable for SCGCHIB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCHIB to value 0"]
impl crate::Resettable for SCGCHIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
