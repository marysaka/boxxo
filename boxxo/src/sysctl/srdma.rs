#[doc = "Register `SRDMA` reader"]
pub type R = crate::R<SRDMA_SPEC>;
#[doc = "Register `SRDMA` writer"]
pub type W = crate::W<SRDMA_SPEC>;
#[doc = "Field `SYSCTL_SRDMA_R0` reader - uDMA Module Software Reset"]
pub type SYSCTL_SRDMA_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRDMA_R0` writer - uDMA Module Software Reset"]
pub type SYSCTL_SRDMA_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - uDMA Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srdma_r0(&self) -> SYSCTL_SRDMA_R0_R {
        SYSCTL_SRDMA_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Module Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srdma_r0(&mut self) -> SYSCTL_SRDMA_R0_W<SRDMA_SPEC, 0> {
        SYSCTL_SRDMA_R0_W::new(self)
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
#[doc = "Micro Direct Memory Access Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srdma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srdma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRDMA_SPEC;
impl crate::RegisterSpec for SRDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srdma::R`](R) reader structure"]
impl crate::Readable for SRDMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srdma::W`](W) writer structure"]
impl crate::Writable for SRDMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRDMA to value 0"]
impl crate::Resettable for SRDMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
