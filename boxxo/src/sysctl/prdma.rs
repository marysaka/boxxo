#[doc = "Register `PRDMA` reader"]
pub type R = crate::R<PRDMA_SPEC>;
#[doc = "Register `PRDMA` writer"]
pub type W = crate::W<PRDMA_SPEC>;
#[doc = "Field `SYSCTL_PRDMA_R0` reader - uDMA Module Peripheral Ready"]
pub type SYSCTL_PRDMA_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRDMA_R0` writer - uDMA Module Peripheral Ready"]
pub type SYSCTL_PRDMA_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - uDMA Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prdma_r0(&self) -> SYSCTL_PRDMA_R0_R {
        SYSCTL_PRDMA_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Module Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prdma_r0(&mut self) -> SYSCTL_PRDMA_R0_W<PRDMA_SPEC, 0> {
        SYSCTL_PRDMA_R0_W::new(self)
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
#[doc = "Micro Direct Memory Access Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prdma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prdma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRDMA_SPEC;
impl crate::RegisterSpec for PRDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prdma::R`](R) reader structure"]
impl crate::Readable for PRDMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prdma::W`](W) writer structure"]
impl crate::Writable for PRDMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRDMA to value 0"]
impl crate::Resettable for PRDMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
