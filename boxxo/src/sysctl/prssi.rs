#[doc = "Register `PRSSI` reader"]
pub type R = crate::R<PRSSI_SPEC>;
#[doc = "Register `PRSSI` writer"]
pub type W = crate::W<PRSSI_SPEC>;
#[doc = "Field `SYSCTL_PRSSI_R0` reader - SSI Module 0 Peripheral Ready"]
pub type SYSCTL_PRSSI_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRSSI_R0` writer - SSI Module 0 Peripheral Ready"]
pub type SYSCTL_PRSSI_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRSSI_R1` reader - SSI Module 1 Peripheral Ready"]
pub type SYSCTL_PRSSI_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRSSI_R1` writer - SSI Module 1 Peripheral Ready"]
pub type SYSCTL_PRSSI_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRSSI_R2` reader - SSI Module 2 Peripheral Ready"]
pub type SYSCTL_PRSSI_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRSSI_R2` writer - SSI Module 2 Peripheral Ready"]
pub type SYSCTL_PRSSI_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRSSI_R3` reader - SSI Module 3 Peripheral Ready"]
pub type SYSCTL_PRSSI_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRSSI_R3` writer - SSI Module 3 Peripheral Ready"]
pub type SYSCTL_PRSSI_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r0(&self) -> SYSCTL_PRSSI_R0_R {
        SYSCTL_PRSSI_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r1(&self) -> SYSCTL_PRSSI_R1_R {
        SYSCTL_PRSSI_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r2(&self) -> SYSCTL_PRSSI_R2_R {
        SYSCTL_PRSSI_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prssi_r3(&self) -> SYSCTL_PRSSI_R3_R {
        SYSCTL_PRSSI_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prssi_r0(&mut self) -> SYSCTL_PRSSI_R0_W<PRSSI_SPEC, 0> {
        SYSCTL_PRSSI_R0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prssi_r1(&mut self) -> SYSCTL_PRSSI_R1_W<PRSSI_SPEC, 1> {
        SYSCTL_PRSSI_R1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prssi_r2(&mut self) -> SYSCTL_PRSSI_R2_W<PRSSI_SPEC, 2> {
        SYSCTL_PRSSI_R2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prssi_r3(&mut self) -> SYSCTL_PRSSI_R3_W<PRSSI_SPEC, 3> {
        SYSCTL_PRSSI_R3_W::new(self)
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
#[doc = "Synchronous Serial Interface Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSSI_SPEC;
impl crate::RegisterSpec for PRSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prssi::R`](R) reader structure"]
impl crate::Readable for PRSSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prssi::W`](W) writer structure"]
impl crate::Writable for PRSSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSSI to value 0"]
impl crate::Resettable for PRSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
