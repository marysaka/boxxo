#[doc = "Register `DCGCCAN` reader"]
pub type R = crate::R<DCGCCAN_SPEC>;
#[doc = "Register `DCGCCAN` writer"]
pub type W = crate::W<DCGCCAN_SPEC>;
#[doc = "Field `SYSCTL_DCGCCAN_D0` reader - CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCAN_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCCAN_D0` writer - CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCAN_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCCAN_D1` reader - CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCAN_D1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCCAN_D1` writer - CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCAN_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgccan_d0(&self) -> SYSCTL_DCGCCAN_D0_R {
        SYSCTL_DCGCCAN_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgccan_d1(&self) -> SYSCTL_DCGCCAN_D1_R {
        SYSCTL_DCGCCAN_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgccan_d0(&mut self) -> SYSCTL_DCGCCAN_D0_W<DCGCCAN_SPEC, 0> {
        SYSCTL_DCGCCAN_D0_W::new(self)
    }
    #[doc = "Bit 1 - CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgccan_d1(&mut self) -> SYSCTL_DCGCCAN_D1_W<DCGCCAN_SPEC, 1> {
        SYSCTL_DCGCCAN_D1_W::new(self)
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
#[doc = "Controller Area Network Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgccan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgccan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCCAN_SPEC;
impl crate::RegisterSpec for DCGCCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgccan::R`](R) reader structure"]
impl crate::Readable for DCGCCAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgccan::W`](W) writer structure"]
impl crate::Writable for DCGCCAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCCAN to value 0"]
impl crate::Resettable for DCGCCAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
