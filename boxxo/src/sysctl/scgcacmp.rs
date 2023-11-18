#[doc = "Register `SCGCACMP` reader"]
pub type R = crate::R<SCGCACMP_SPEC>;
#[doc = "Register `SCGCACMP` writer"]
pub type W = crate::W<SCGCACMP_SPEC>;
#[doc = "Field `SYSCTL_SCGCACMP_S0` reader - Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCACMP_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCACMP_S0` writer - Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCACMP_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcacmp_s0(&self) -> SYSCTL_SCGCACMP_S0_R {
        SYSCTL_SCGCACMP_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcacmp_s0(&mut self) -> SYSCTL_SCGCACMP_S0_W<SCGCACMP_SPEC, 0> {
        SYSCTL_SCGCACMP_S0_W::new(self)
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
#[doc = "Analog Comparator Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcacmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcacmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCACMP_SPEC;
impl crate::RegisterSpec for SCGCACMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcacmp::R`](R) reader structure"]
impl crate::Readable for SCGCACMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcacmp::W`](W) writer structure"]
impl crate::Writable for SCGCACMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCACMP to value 0"]
impl crate::Resettable for SCGCACMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
