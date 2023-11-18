#[doc = "Register `SRSSI` reader"]
pub type R = crate::R<SRSSI_SPEC>;
#[doc = "Register `SRSSI` writer"]
pub type W = crate::W<SRSSI_SPEC>;
#[doc = "Field `SYSCTL_SRSSI_R0` reader - SSI Module 0 Software Reset"]
pub type SYSCTL_SRSSI_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRSSI_R0` writer - SSI Module 0 Software Reset"]
pub type SYSCTL_SRSSI_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRSSI_R1` reader - SSI Module 1 Software Reset"]
pub type SYSCTL_SRSSI_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRSSI_R1` writer - SSI Module 1 Software Reset"]
pub type SYSCTL_SRSSI_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRSSI_R2` reader - SSI Module 2 Software Reset"]
pub type SYSCTL_SRSSI_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRSSI_R2` writer - SSI Module 2 Software Reset"]
pub type SYSCTL_SRSSI_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRSSI_R3` reader - SSI Module 3 Software Reset"]
pub type SYSCTL_SRSSI_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRSSI_R3` writer - SSI Module 3 Software Reset"]
pub type SYSCTL_SRSSI_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r0(&self) -> SYSCTL_SRSSI_R0_R {
        SYSCTL_SRSSI_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r1(&self) -> SYSCTL_SRSSI_R1_R {
        SYSCTL_SRSSI_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r2(&self) -> SYSCTL_SRSSI_R2_R {
        SYSCTL_SRSSI_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r3(&self) -> SYSCTL_SRSSI_R3_R {
        SYSCTL_SRSSI_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srssi_r0(&mut self) -> SYSCTL_SRSSI_R0_W<SRSSI_SPEC, 0> {
        SYSCTL_SRSSI_R0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srssi_r1(&mut self) -> SYSCTL_SRSSI_R1_W<SRSSI_SPEC, 1> {
        SYSCTL_SRSSI_R1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srssi_r2(&mut self) -> SYSCTL_SRSSI_R2_W<SRSSI_SPEC, 2> {
        SYSCTL_SRSSI_R2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srssi_r3(&mut self) -> SYSCTL_SRSSI_R3_W<SRSSI_SPEC, 3> {
        SYSCTL_SRSSI_R3_W::new(self)
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
#[doc = "Synchronous Serial Interface Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSSI_SPEC;
impl crate::RegisterSpec for SRSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srssi::R`](R) reader structure"]
impl crate::Readable for SRSSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srssi::W`](W) writer structure"]
impl crate::Writable for SRSSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSSI to value 0"]
impl crate::Resettable for SRSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
