#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MIS_SPEC>;
#[doc = "Field `UART_MIS_CTSMIS` reader - UART Clear to Send Modem Masked Interrupt Status"]
pub type UART_MIS_CTSMIS_R = crate::BitReader;
#[doc = "Field `UART_MIS_CTSMIS` writer - UART Clear to Send Modem Masked Interrupt Status"]
pub type UART_MIS_CTSMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_MIS_RXMIS` reader - UART Receive Masked Interrupt Status"]
pub type UART_MIS_RXMIS_R = crate::BitReader;
#[doc = "Field `UART_MIS_RXMIS` writer - UART Receive Masked Interrupt Status"]
pub type UART_MIS_RXMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_MIS_TXMIS` reader - UART Transmit Masked Interrupt Status"]
pub type UART_MIS_TXMIS_R = crate::BitReader;
#[doc = "Field `UART_MIS_TXMIS` writer - UART Transmit Masked Interrupt Status"]
pub type UART_MIS_TXMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_MIS_RTMIS` reader - UART Receive Time-Out Masked Interrupt Status"]
pub type UART_MIS_RTMIS_R = crate::BitReader;
#[doc = "Field `UART_MIS_RTMIS` writer - UART Receive Time-Out Masked Interrupt Status"]
pub type UART_MIS_RTMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_MIS_FEMIS` reader - UART Framing Error Masked Interrupt Status"]
pub type UART_MIS_FEMIS_R = crate::BitReader;
#[doc = "Field `UART_MIS_FEMIS` writer - UART Framing Error Masked Interrupt Status"]
pub type UART_MIS_FEMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_MIS_PEMIS` reader - UART Parity Error Masked Interrupt Status"]
pub type UART_MIS_PEMIS_R = crate::BitReader;
#[doc = "Field `UART_MIS_PEMIS` writer - UART Parity Error Masked Interrupt Status"]
pub type UART_MIS_PEMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_MIS_BEMIS` reader - UART Break Error Masked Interrupt Status"]
pub type UART_MIS_BEMIS_R = crate::BitReader;
#[doc = "Field `UART_MIS_BEMIS` writer - UART Break Error Masked Interrupt Status"]
pub type UART_MIS_BEMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_MIS_OEMIS` reader - UART Overrun Error Masked Interrupt Status"]
pub type UART_MIS_OEMIS_R = crate::BitReader;
#[doc = "Field `UART_MIS_OEMIS` writer - UART Overrun Error Masked Interrupt Status"]
pub type UART_MIS_OEMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_MIS_9BITMIS` reader - 9-Bit Mode Masked Interrupt Status"]
pub type UART_MIS_9BITMIS_R = crate::BitReader;
#[doc = "Field `UART_MIS_9BITMIS` writer - 9-Bit Mode Masked Interrupt Status"]
pub type UART_MIS_9BITMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - UART Clear to Send Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_ctsmis(&self) -> UART_MIS_CTSMIS_R {
        UART_MIS_CTSMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rxmis(&self) -> UART_MIS_RXMIS_R {
        UART_MIS_RXMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_txmis(&self) -> UART_MIS_TXMIS_R {
        UART_MIS_TXMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rtmis(&self) -> UART_MIS_RTMIS_R {
        UART_MIS_RTMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_femis(&self) -> UART_MIS_FEMIS_R {
        UART_MIS_FEMIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_pemis(&self) -> UART_MIS_PEMIS_R {
        UART_MIS_PEMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_bemis(&self) -> UART_MIS_BEMIS_R {
        UART_MIS_BEMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_oemis(&self) -> UART_MIS_OEMIS_R {
        UART_MIS_OEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-Bit Mode Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_9bitmis(&self) -> UART_MIS_9BITMIS_R {
        UART_MIS_9BITMIS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - UART Clear to Send Modem Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mis_ctsmis(&mut self) -> UART_MIS_CTSMIS_W<MIS_SPEC, 1> {
        UART_MIS_CTSMIS_W::new(self)
    }
    #[doc = "Bit 4 - UART Receive Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mis_rxmis(&mut self) -> UART_MIS_RXMIS_W<MIS_SPEC, 4> {
        UART_MIS_RXMIS_W::new(self)
    }
    #[doc = "Bit 5 - UART Transmit Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mis_txmis(&mut self) -> UART_MIS_TXMIS_W<MIS_SPEC, 5> {
        UART_MIS_TXMIS_W::new(self)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mis_rtmis(&mut self) -> UART_MIS_RTMIS_W<MIS_SPEC, 6> {
        UART_MIS_RTMIS_W::new(self)
    }
    #[doc = "Bit 7 - UART Framing Error Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mis_femis(&mut self) -> UART_MIS_FEMIS_W<MIS_SPEC, 7> {
        UART_MIS_FEMIS_W::new(self)
    }
    #[doc = "Bit 8 - UART Parity Error Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mis_pemis(&mut self) -> UART_MIS_PEMIS_W<MIS_SPEC, 8> {
        UART_MIS_PEMIS_W::new(self)
    }
    #[doc = "Bit 9 - UART Break Error Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mis_bemis(&mut self) -> UART_MIS_BEMIS_W<MIS_SPEC, 9> {
        UART_MIS_BEMIS_W::new(self)
    }
    #[doc = "Bit 10 - UART Overrun Error Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mis_oemis(&mut self) -> UART_MIS_OEMIS_W<MIS_SPEC, 10> {
        UART_MIS_OEMIS_W::new(self)
    }
    #[doc = "Bit 12 - 9-Bit Mode Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mis_9bitmis(&mut self) -> UART_MIS_9BITMIS_W<MIS_SPEC, 12> {
        UART_MIS_9BITMIS_W::new(self)
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
#[doc = "UART Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
