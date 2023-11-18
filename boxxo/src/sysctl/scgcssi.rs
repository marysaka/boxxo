#[doc = "Register `SCGCSSI` reader"]
pub type R = crate::R<SCGCSSI_SPEC>;
#[doc = "Register `SCGCSSI` writer"]
pub type W = crate::W<SCGCSSI_SPEC>;
#[doc = "Field `SYSCTL_SCGCSSI_S0` reader - SSI Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCSSI_S0` writer - SSI Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCSSI_S1` reader - SSI Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCSSI_S1` writer - SSI Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCSSI_S2` reader - SSI Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCSSI_S2` writer - SSI Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCSSI_S3` reader - SSI Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCSSI_S3` writer - SSI Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s0(&self) -> SYSCTL_SCGCSSI_S0_R {
        SYSCTL_SCGCSSI_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s1(&self) -> SYSCTL_SCGCSSI_S1_R {
        SYSCTL_SCGCSSI_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s2(&self) -> SYSCTL_SCGCSSI_S2_R {
        SYSCTL_SCGCSSI_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s3(&self) -> SYSCTL_SCGCSSI_S3_R {
        SYSCTL_SCGCSSI_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcssi_s0(&mut self) -> SYSCTL_SCGCSSI_S0_W<SCGCSSI_SPEC, 0> {
        SYSCTL_SCGCSSI_S0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcssi_s1(&mut self) -> SYSCTL_SCGCSSI_S1_W<SCGCSSI_SPEC, 1> {
        SYSCTL_SCGCSSI_S1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcssi_s2(&mut self) -> SYSCTL_SCGCSSI_S2_W<SCGCSSI_SPEC, 2> {
        SYSCTL_SCGCSSI_S2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcssi_s3(&mut self) -> SYSCTL_SCGCSSI_S3_W<SCGCSSI_SPEC, 3> {
        SYSCTL_SCGCSSI_S3_W::new(self)
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
#[doc = "Synchronous Serial Interface Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCSSI_SPEC;
impl crate::RegisterSpec for SCGCSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcssi::R`](R) reader structure"]
impl crate::Readable for SCGCSSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcssi::W`](W) writer structure"]
impl crate::Writable for SCGCSSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCSSI to value 0"]
impl crate::Resettable for SCGCSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
