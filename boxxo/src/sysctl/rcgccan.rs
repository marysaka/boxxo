#[doc = "Register `RCGCCAN` reader"]
pub type R = crate::R<RCGCCAN_SPEC>;
#[doc = "Register `RCGCCAN` writer"]
pub type W = crate::W<RCGCCAN_SPEC>;
#[doc = "Field `SYSCTL_RCGCCAN_R0` reader - CAN Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCCAN_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCCAN_R0` writer - CAN Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCCAN_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCCAN_R1` reader - CAN Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCCAN_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCCAN_R1` writer - CAN Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCCAN_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CAN Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgccan_r0(&self) -> SYSCTL_RCGCCAN_R0_R {
        SYSCTL_RCGCCAN_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgccan_r1(&self) -> SYSCTL_RCGCCAN_R1_R {
        SYSCTL_RCGCCAN_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgccan_r0(&mut self) -> SYSCTL_RCGCCAN_R0_W<RCGCCAN_SPEC, 0> {
        SYSCTL_RCGCCAN_R0_W::new(self)
    }
    #[doc = "Bit 1 - CAN Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgccan_r1(&mut self) -> SYSCTL_RCGCCAN_R1_W<RCGCCAN_SPEC, 1> {
        SYSCTL_RCGCCAN_R1_W::new(self)
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
#[doc = "Controller Area Network Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgccan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgccan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGCCAN_SPEC;
impl crate::RegisterSpec for RCGCCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgccan::R`](R) reader structure"]
impl crate::Readable for RCGCCAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgccan::W`](W) writer structure"]
impl crate::Writable for RCGCCAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCCAN to value 0"]
impl crate::Resettable for RCGCCAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
