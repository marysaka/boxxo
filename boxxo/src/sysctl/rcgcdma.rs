#[doc = "Register `RCGCDMA` reader"]
pub type R = crate::R<RCGCDMA_SPEC>;
#[doc = "Register `RCGCDMA` writer"]
pub type W = crate::W<RCGCDMA_SPEC>;
#[doc = "Field `SYSCTL_RCGCDMA_R0` reader - uDMA Module Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCDMA_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCDMA_R0` writer - uDMA Module Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCDMA_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - uDMA Module Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcdma_r0(&self) -> SYSCTL_RCGCDMA_R0_R {
        SYSCTL_RCGCDMA_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Module Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcdma_r0(&mut self) -> SYSCTL_RCGCDMA_R0_W<RCGCDMA_SPEC, 0> {
        SYSCTL_RCGCDMA_R0_W::new(self)
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
#[doc = "Micro Direct Memory Access Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcdma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcdma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGCDMA_SPEC;
impl crate::RegisterSpec for RCGCDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgcdma::R`](R) reader structure"]
impl crate::Readable for RCGCDMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgcdma::W`](W) writer structure"]
impl crate::Writable for RCGCDMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCDMA to value 0"]
impl crate::Resettable for RCGCDMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
