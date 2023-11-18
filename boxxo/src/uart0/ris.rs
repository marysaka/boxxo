#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `UART_RIS_CTSRIS` reader - UART Clear to Send Modem Raw Interrupt Status"]
pub type UART_RIS_CTSRIS_R = crate::BitReader;
#[doc = "Field `UART_RIS_CTSRIS` writer - UART Clear to Send Modem Raw Interrupt Status"]
pub type UART_RIS_CTSRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RIS_RXRIS` reader - UART Receive Raw Interrupt Status"]
pub type UART_RIS_RXRIS_R = crate::BitReader;
#[doc = "Field `UART_RIS_RXRIS` writer - UART Receive Raw Interrupt Status"]
pub type UART_RIS_RXRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RIS_TXRIS` reader - UART Transmit Raw Interrupt Status"]
pub type UART_RIS_TXRIS_R = crate::BitReader;
#[doc = "Field `UART_RIS_TXRIS` writer - UART Transmit Raw Interrupt Status"]
pub type UART_RIS_TXRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RIS_RTRIS` reader - UART Receive Time-Out Raw Interrupt Status"]
pub type UART_RIS_RTRIS_R = crate::BitReader;
#[doc = "Field `UART_RIS_RTRIS` writer - UART Receive Time-Out Raw Interrupt Status"]
pub type UART_RIS_RTRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RIS_FERIS` reader - UART Framing Error Raw Interrupt Status"]
pub type UART_RIS_FERIS_R = crate::BitReader;
#[doc = "Field `UART_RIS_FERIS` writer - UART Framing Error Raw Interrupt Status"]
pub type UART_RIS_FERIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RIS_PERIS` reader - UART Parity Error Raw Interrupt Status"]
pub type UART_RIS_PERIS_R = crate::BitReader;
#[doc = "Field `UART_RIS_PERIS` writer - UART Parity Error Raw Interrupt Status"]
pub type UART_RIS_PERIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RIS_BERIS` reader - UART Break Error Raw Interrupt Status"]
pub type UART_RIS_BERIS_R = crate::BitReader;
#[doc = "Field `UART_RIS_BERIS` writer - UART Break Error Raw Interrupt Status"]
pub type UART_RIS_BERIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RIS_OERIS` reader - UART Overrun Error Raw Interrupt Status"]
pub type UART_RIS_OERIS_R = crate::BitReader;
#[doc = "Field `UART_RIS_OERIS` writer - UART Overrun Error Raw Interrupt Status"]
pub type UART_RIS_OERIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RIS_9BITRIS` reader - 9-Bit Mode Raw Interrupt Status"]
pub type UART_RIS_9BITRIS_R = crate::BitReader;
#[doc = "Field `UART_RIS_9BITRIS` writer - 9-Bit Mode Raw Interrupt Status"]
pub type UART_RIS_9BITRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - UART Clear to Send Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_ctsris(&self) -> UART_RIS_CTSRIS_R {
        UART_RIS_CTSRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rxris(&self) -> UART_RIS_RXRIS_R {
        UART_RIS_RXRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_txris(&self) -> UART_RIS_TXRIS_R {
        UART_RIS_TXRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rtris(&self) -> UART_RIS_RTRIS_R {
        UART_RIS_RTRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_feris(&self) -> UART_RIS_FERIS_R {
        UART_RIS_FERIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_peris(&self) -> UART_RIS_PERIS_R {
        UART_RIS_PERIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_beris(&self) -> UART_RIS_BERIS_R {
        UART_RIS_BERIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_oeris(&self) -> UART_RIS_OERIS_R {
        UART_RIS_OERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-Bit Mode Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_9bitris(&self) -> UART_RIS_9BITRIS_R {
        UART_RIS_9BITRIS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - UART Clear to Send Modem Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ris_ctsris(&mut self) -> UART_RIS_CTSRIS_W<RIS_SPEC, 1> {
        UART_RIS_CTSRIS_W::new(self)
    }
    #[doc = "Bit 4 - UART Receive Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ris_rxris(&mut self) -> UART_RIS_RXRIS_W<RIS_SPEC, 4> {
        UART_RIS_RXRIS_W::new(self)
    }
    #[doc = "Bit 5 - UART Transmit Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ris_txris(&mut self) -> UART_RIS_TXRIS_W<RIS_SPEC, 5> {
        UART_RIS_TXRIS_W::new(self)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ris_rtris(&mut self) -> UART_RIS_RTRIS_W<RIS_SPEC, 6> {
        UART_RIS_RTRIS_W::new(self)
    }
    #[doc = "Bit 7 - UART Framing Error Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ris_feris(&mut self) -> UART_RIS_FERIS_W<RIS_SPEC, 7> {
        UART_RIS_FERIS_W::new(self)
    }
    #[doc = "Bit 8 - UART Parity Error Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ris_peris(&mut self) -> UART_RIS_PERIS_W<RIS_SPEC, 8> {
        UART_RIS_PERIS_W::new(self)
    }
    #[doc = "Bit 9 - UART Break Error Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ris_beris(&mut self) -> UART_RIS_BERIS_W<RIS_SPEC, 9> {
        UART_RIS_BERIS_W::new(self)
    }
    #[doc = "Bit 10 - UART Overrun Error Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ris_oeris(&mut self) -> UART_RIS_OERIS_W<RIS_SPEC, 10> {
        UART_RIS_OERIS_W::new(self)
    }
    #[doc = "Bit 12 - 9-Bit Mode Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ris_9bitris(&mut self) -> UART_RIS_9BITRIS_W<RIS_SPEC, 12> {
        UART_RIS_9BITRIS_W::new(self)
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
#[doc = "UART Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
