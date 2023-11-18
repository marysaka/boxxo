#[doc = "Register `DMACTL` reader"]
pub type R = crate::R<DMACTL_SPEC>;
#[doc = "Register `DMACTL` writer"]
pub type W = crate::W<DMACTL_SPEC>;
#[doc = "Field `UART_DMACTL_RXDMAE` reader - Receive DMA Enable"]
pub type UART_DMACTL_RXDMAE_R = crate::BitReader;
#[doc = "Field `UART_DMACTL_RXDMAE` writer - Receive DMA Enable"]
pub type UART_DMACTL_RXDMAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_DMACTL_TXDMAE` reader - Transmit DMA Enable"]
pub type UART_DMACTL_TXDMAE_R = crate::BitReader;
#[doc = "Field `UART_DMACTL_TXDMAE` writer - Transmit DMA Enable"]
pub type UART_DMACTL_TXDMAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_DMACTL_DMAERR` reader - DMA on Error"]
pub type UART_DMACTL_DMAERR_R = crate::BitReader;
#[doc = "Field `UART_DMACTL_DMAERR` writer - DMA on Error"]
pub type UART_DMACTL_DMAERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive DMA Enable"]
    #[inline(always)]
    pub fn uart_dmactl_rxdmae(&self) -> UART_DMACTL_RXDMAE_R {
        UART_DMACTL_RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn uart_dmactl_txdmae(&self) -> UART_DMACTL_TXDMAE_R {
        UART_DMACTL_TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA on Error"]
    #[inline(always)]
    pub fn uart_dmactl_dmaerr(&self) -> UART_DMACTL_DMAERR_R {
        UART_DMACTL_DMAERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart_dmactl_rxdmae(&mut self) -> UART_DMACTL_RXDMAE_W<DMACTL_SPEC, 0> {
        UART_DMACTL_RXDMAE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart_dmactl_txdmae(&mut self) -> UART_DMACTL_TXDMAE_W<DMACTL_SPEC, 1> {
        UART_DMACTL_TXDMAE_W::new(self)
    }
    #[doc = "Bit 2 - DMA on Error"]
    #[inline(always)]
    #[must_use]
    pub fn uart_dmactl_dmaerr(&mut self) -> UART_DMACTL_DMAERR_W<DMACTL_SPEC, 2> {
        UART_DMACTL_DMAERR_W::new(self)
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
#[doc = "UART DMA Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
