#[doc = "Register `DCGCSSI` reader"]
pub type R = crate::R<DCGCSSI_SPEC>;
#[doc = "Register `DCGCSSI` writer"]
pub type W = crate::W<DCGCSSI_SPEC>;
#[doc = "Field `SYSCTL_DCGCSSI_D0` reader - SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCSSI_D0` writer - SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCSSI_D1` reader - SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCSSI_D1` writer - SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCSSI_D2` reader - SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCSSI_D2` writer - SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCSSI_D3` reader - SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCSSI_D3` writer - SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d0(&self) -> SYSCTL_DCGCSSI_D0_R {
        SYSCTL_DCGCSSI_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d1(&self) -> SYSCTL_DCGCSSI_D1_R {
        SYSCTL_DCGCSSI_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d2(&self) -> SYSCTL_DCGCSSI_D2_R {
        SYSCTL_DCGCSSI_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d3(&self) -> SYSCTL_DCGCSSI_D3_R {
        SYSCTL_DCGCSSI_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcssi_d0(&mut self) -> SYSCTL_DCGCSSI_D0_W<DCGCSSI_SPEC, 0> {
        SYSCTL_DCGCSSI_D0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcssi_d1(&mut self) -> SYSCTL_DCGCSSI_D1_W<DCGCSSI_SPEC, 1> {
        SYSCTL_DCGCSSI_D1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcssi_d2(&mut self) -> SYSCTL_DCGCSSI_D2_W<DCGCSSI_SPEC, 2> {
        SYSCTL_DCGCSSI_D2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcssi_d3(&mut self) -> SYSCTL_DCGCSSI_D3_W<DCGCSSI_SPEC, 3> {
        SYSCTL_DCGCSSI_D3_W::new(self)
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
#[doc = "Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCSSI_SPEC;
impl crate::RegisterSpec for DCGCSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcssi::R`](R) reader structure"]
impl crate::Readable for DCGCSSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgcssi::W`](W) writer structure"]
impl crate::Writable for DCGCSSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCSSI to value 0"]
impl crate::Resettable for DCGCSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
