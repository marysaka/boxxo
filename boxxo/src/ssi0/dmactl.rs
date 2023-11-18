#[doc = "Register `DMACTL` reader"]
pub type R = crate::R<DMACTL_SPEC>;
#[doc = "Register `DMACTL` writer"]
pub type W = crate::W<DMACTL_SPEC>;
#[doc = "Field `SSI_DMACTL_RXDMAE` reader - Receive DMA Enable"]
pub type SSI_DMACTL_RXDMAE_R = crate::BitReader;
#[doc = "Field `SSI_DMACTL_RXDMAE` writer - Receive DMA Enable"]
pub type SSI_DMACTL_RXDMAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_DMACTL_TXDMAE` reader - Transmit DMA Enable"]
pub type SSI_DMACTL_TXDMAE_R = crate::BitReader;
#[doc = "Field `SSI_DMACTL_TXDMAE` writer - Transmit DMA Enable"]
pub type SSI_DMACTL_TXDMAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive DMA Enable"]
    #[inline(always)]
    pub fn ssi_dmactl_rxdmae(&self) -> SSI_DMACTL_RXDMAE_R {
        SSI_DMACTL_RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn ssi_dmactl_txdmae(&self) -> SSI_DMACTL_TXDMAE_R {
        SSI_DMACTL_TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_dmactl_rxdmae(&mut self) -> SSI_DMACTL_RXDMAE_W<DMACTL_SPEC, 0> {
        SSI_DMACTL_RXDMAE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_dmactl_txdmae(&mut self) -> SSI_DMACTL_TXDMAE_W<DMACTL_SPEC, 1> {
        SSI_DMACTL_TXDMAE_W::new(self)
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
#[doc = "SSI DMA Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTL_SPEC;
impl crate::RegisterSpec for DMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactl::R`](R) reader structure"]
impl crate::Readable for DMACTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmactl::W`](W) writer structure"]
impl crate::Writable for DMACTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DMACTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
