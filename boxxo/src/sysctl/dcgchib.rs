#[doc = "Register `DCGCHIB` reader"]
pub type R = crate::R<DCGCHIB_SPEC>;
#[doc = "Register `DCGCHIB` writer"]
pub type W = crate::W<DCGCHIB_SPEC>;
#[doc = "Field `SYSCTL_DCGCHIB_D0` reader - Hibernation Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCHIB_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCHIB_D0` writer - Hibernation Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCHIB_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Hibernation Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgchib_d0(&self) -> SYSCTL_DCGCHIB_D0_R {
        SYSCTL_DCGCHIB_D0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernation Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgchib_d0(&mut self) -> SYSCTL_DCGCHIB_D0_W<DCGCHIB_SPEC, 0> {
        SYSCTL_DCGCHIB_D0_W::new(self)
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
#[doc = "Hibernation Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgchib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgchib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCHIB_SPEC;
impl crate::RegisterSpec for DCGCHIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgchib::R`](R) reader structure"]
impl crate::Readable for DCGCHIB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgchib::W`](W) writer structure"]
impl crate::Writable for DCGCHIB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCHIB to value 0"]
impl crate::Resettable for DCGCHIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
