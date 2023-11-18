#[doc = "Register `SRQEI` reader"]
pub type R = crate::R<SRQEI_SPEC>;
#[doc = "Register `SRQEI` writer"]
pub type W = crate::W<SRQEI_SPEC>;
#[doc = "Field `SYSCTL_SRQEI_R0` reader - QEI Module 0 Software Reset"]
pub type SYSCTL_SRQEI_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRQEI_R0` writer - QEI Module 0 Software Reset"]
pub type SYSCTL_SRQEI_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRQEI_R1` reader - QEI Module 1 Software Reset"]
pub type SYSCTL_SRQEI_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRQEI_R1` writer - QEI Module 1 Software Reset"]
pub type SYSCTL_SRQEI_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - QEI Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srqei_r0(&self) -> SYSCTL_SRQEI_R0_R {
        SYSCTL_SRQEI_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QEI Module 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srqei_r1(&self) -> SYSCTL_SRQEI_R1_R {
        SYSCTL_SRQEI_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QEI Module 0 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srqei_r0(&mut self) -> SYSCTL_SRQEI_R0_W<SRQEI_SPEC, 0> {
        SYSCTL_SRQEI_R0_W::new(self)
    }
    #[doc = "Bit 1 - QEI Module 1 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srqei_r1(&mut self) -> SYSCTL_SRQEI_R1_W<SRQEI_SPEC, 1> {
        SYSCTL_SRQEI_R1_W::new(self)
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
#[doc = "Quadrature Encoder Interface Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srqei::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srqei::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRQEI_SPEC;
impl crate::RegisterSpec for SRQEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqei::R`](R) reader structure"]
impl crate::Readable for SRQEI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srqei::W`](W) writer structure"]
impl crate::Writable for SRQEI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRQEI to value 0"]
impl crate::Resettable for SRQEI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
