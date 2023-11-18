#[doc = "Register `RCGCQEI` reader"]
pub type R = crate::R<RCGCQEI_SPEC>;
#[doc = "Register `RCGCQEI` writer"]
pub type W = crate::W<RCGCQEI_SPEC>;
#[doc = "Field `SYSCTL_RCGCQEI_R0` reader - QEI Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCQEI_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCQEI_R0` writer - QEI Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCQEI_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCQEI_R1` reader - QEI Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCQEI_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCQEI_R1` writer - QEI Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCQEI_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - QEI Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcqei_r0(&self) -> SYSCTL_RCGCQEI_R0_R {
        SYSCTL_RCGCQEI_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QEI Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcqei_r1(&self) -> SYSCTL_RCGCQEI_R1_R {
        SYSCTL_RCGCQEI_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QEI Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcqei_r0(&mut self) -> SYSCTL_RCGCQEI_R0_W<RCGCQEI_SPEC, 0> {
        SYSCTL_RCGCQEI_R0_W::new(self)
    }
    #[doc = "Bit 1 - QEI Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcqei_r1(&mut self) -> SYSCTL_RCGCQEI_R1_W<RCGCQEI_SPEC, 1> {
        SYSCTL_RCGCQEI_R1_W::new(self)
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
#[doc = "Quadrature Encoder Interface Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcqei::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcqei::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGCQEI_SPEC;
impl crate::RegisterSpec for RCGCQEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgcqei::R`](R) reader structure"]
impl crate::Readable for RCGCQEI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgcqei::W`](W) writer structure"]
impl crate::Writable for RCGCQEI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCQEI to value 0"]
impl crate::Resettable for RCGCQEI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
