#[doc = "Register `SCGCCAN` reader"]
pub type R = crate::R<SCGCCAN_SPEC>;
#[doc = "Register `SCGCCAN` writer"]
pub type W = crate::W<SCGCCAN_SPEC>;
#[doc = "Field `SYSCTL_SCGCCAN_S0` reader - CAN Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCAN_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCCAN_S0` writer - CAN Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCAN_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCCAN_S1` reader - CAN Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCAN_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCCAN_S1` writer - CAN Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCAN_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgccan_s0(&self) -> SYSCTL_SCGCCAN_S0_R {
        SYSCTL_SCGCCAN_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgccan_s1(&self) -> SYSCTL_SCGCCAN_S1_R {
        SYSCTL_SCGCCAN_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgccan_s0(&mut self) -> SYSCTL_SCGCCAN_S0_W<SCGCCAN_SPEC, 0> {
        SYSCTL_SCGCCAN_S0_W::new(self)
    }
    #[doc = "Bit 1 - CAN Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgccan_s1(&mut self) -> SYSCTL_SCGCCAN_S1_W<SCGCCAN_SPEC, 1> {
        SYSCTL_SCGCCAN_S1_W::new(self)
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
#[doc = "Controller Area Network Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgccan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgccan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCCAN_SPEC;
impl crate::RegisterSpec for SCGCCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgccan::R`](R) reader structure"]
impl crate::Readable for SCGCCAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgccan::W`](W) writer structure"]
impl crate::Writable for SCGCCAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCCAN to value 0"]
impl crate::Resettable for SCGCCAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
