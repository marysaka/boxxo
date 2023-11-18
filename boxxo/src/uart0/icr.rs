#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `UART_ICR_CTSMIC` writer - UART Clear to Send Modem Interrupt Clear"]
pub type UART_ICR_CTSMIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_ICR_RXIC` writer - Receive Interrupt Clear"]
pub type UART_ICR_RXIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_ICR_TXIC` writer - Transmit Interrupt Clear"]
pub type UART_ICR_TXIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_ICR_RTIC` writer - Receive Time-Out Interrupt Clear"]
pub type UART_ICR_RTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_ICR_FEIC` writer - Framing Error Interrupt Clear"]
pub type UART_ICR_FEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_ICR_PEIC` writer - Parity Error Interrupt Clear"]
pub type UART_ICR_PEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_ICR_BEIC` writer - Break Error Interrupt Clear"]
pub type UART_ICR_BEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_ICR_OEIC` writer - Overrun Error Interrupt Clear"]
pub type UART_ICR_OEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_ICR_9BITIC` writer - 9-Bit Mode Interrupt Clear"]
pub type UART_ICR_9BITIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uart_icr_ctsmic(&mut self) -> UART_ICR_CTSMIC_W<ICR_SPEC, 1> {
        UART_ICR_CTSMIC_W::new(self)
    }
    #[doc = "Bit 4 - Receive Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uart_icr_rxic(&mut self) -> UART_ICR_RXIC_W<ICR_SPEC, 4> {
        UART_ICR_RXIC_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uart_icr_txic(&mut self) -> UART_ICR_TXIC_W<ICR_SPEC, 5> {
        UART_ICR_TXIC_W::new(self)
    }
    #[doc = "Bit 6 - Receive Time-Out Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uart_icr_rtic(&mut self) -> UART_ICR_RTIC_W<ICR_SPEC, 6> {
        UART_ICR_RTIC_W::new(self)
    }
    #[doc = "Bit 7 - Framing Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uart_icr_feic(&mut self) -> UART_ICR_FEIC_W<ICR_SPEC, 7> {
        UART_ICR_FEIC_W::new(self)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uart_icr_peic(&mut self) -> UART_ICR_PEIC_W<ICR_SPEC, 8> {
        UART_ICR_PEIC_W::new(self)
    }
    #[doc = "Bit 9 - Break Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uart_icr_beic(&mut self) -> UART_ICR_BEIC_W<ICR_SPEC, 9> {
        UART_ICR_BEIC_W::new(self)
    }
    #[doc = "Bit 10 - Overrun Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uart_icr_oeic(&mut self) -> UART_ICR_OEIC_W<ICR_SPEC, 10> {
        UART_ICR_OEIC_W::new(self)
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uart_icr_9bitic(&mut self) -> UART_ICR_9BITIC_W<ICR_SPEC, 12> {
        UART_ICR_9BITIC_W::new(self)
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
#[doc = "UART Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
