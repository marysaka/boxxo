#[doc = "Register `SCGCDMA` reader"]
pub type R = crate::R<SCGCDMA_SPEC>;
#[doc = "Register `SCGCDMA` writer"]
pub type W = crate::W<SCGCDMA_SPEC>;
#[doc = "Field `SYSCTL_SCGCDMA_S0` reader - uDMA Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCDMA_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCDMA_S0` writer - uDMA Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCDMA_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - uDMA Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcdma_s0(&self) -> SYSCTL_SCGCDMA_S0_R {
        SYSCTL_SCGCDMA_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcdma_s0(&mut self) -> SYSCTL_SCGCDMA_S0_W<SCGCDMA_SPEC, 0> {
        SYSCTL_SCGCDMA_S0_W::new(self)
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
#[doc = "Micro Direct Memory Access Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcdma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcdma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCDMA_SPEC;
impl crate::RegisterSpec for SCGCDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcdma::R`](R) reader structure"]
impl crate::Readable for SCGCDMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcdma::W`](W) writer structure"]
impl crate::Writable for SCGCDMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCDMA to value 0"]
impl crate::Resettable for SCGCDMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
