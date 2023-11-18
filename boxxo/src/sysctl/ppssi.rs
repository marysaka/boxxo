#[doc = "Register `PPSSI` reader"]
pub type R = crate::R<PPSSI_SPEC>;
#[doc = "Register `PPSSI` writer"]
pub type W = crate::W<PPSSI_SPEC>;
#[doc = "Field `SYSCTL_PPSSI_P0` reader - SSI Module 0 Present"]
pub type SYSCTL_PPSSI_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPSSI_P0` writer - SSI Module 0 Present"]
pub type SYSCTL_PPSSI_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPSSI_P1` reader - SSI Module 1 Present"]
pub type SYSCTL_PPSSI_P1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPSSI_P1` writer - SSI Module 1 Present"]
pub type SYSCTL_PPSSI_P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPSSI_P2` reader - SSI Module 2 Present"]
pub type SYSCTL_PPSSI_P2_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPSSI_P2` writer - SSI Module 2 Present"]
pub type SYSCTL_PPSSI_P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPSSI_P3` reader - SSI Module 3 Present"]
pub type SYSCTL_PPSSI_P3_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPSSI_P3` writer - SSI Module 3 Present"]
pub type SYSCTL_PPSSI_P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p0(&self) -> SYSCTL_PPSSI_P0_R {
        SYSCTL_PPSSI_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p1(&self) -> SYSCTL_PPSSI_P1_R {
        SYSCTL_PPSSI_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p2(&self) -> SYSCTL_PPSSI_P2_R {
        SYSCTL_PPSSI_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p3(&self) -> SYSCTL_PPSSI_P3_R {
        SYSCTL_PPSSI_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppssi_p0(&mut self) -> SYSCTL_PPSSI_P0_W<PPSSI_SPEC, 0> {
        SYSCTL_PPSSI_P0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppssi_p1(&mut self) -> SYSCTL_PPSSI_P1_W<PPSSI_SPEC, 1> {
        SYSCTL_PPSSI_P1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppssi_p2(&mut self) -> SYSCTL_PPSSI_P2_W<PPSSI_SPEC, 2> {
        SYSCTL_PPSSI_P2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppssi_p3(&mut self) -> SYSCTL_PPSSI_P3_W<PPSSI_SPEC, 3> {
        SYSCTL_PPSSI_P3_W::new(self)
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
#[doc = "Synchronous Serial Interface Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPSSI_SPEC;
impl crate::RegisterSpec for PPSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppssi::R`](R) reader structure"]
impl crate::Readable for PPSSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppssi::W`](W) writer structure"]
impl crate::Writable for PPSSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPSSI to value 0"]
impl crate::Resettable for PPSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
