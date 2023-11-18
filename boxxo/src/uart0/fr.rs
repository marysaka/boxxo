#[doc = "Register `FR` reader"]
pub type R = crate::R<FR_SPEC>;
#[doc = "Register `FR` writer"]
pub type W = crate::W<FR_SPEC>;
#[doc = "Field `UART_FR_CTS` reader - Clear To Send"]
pub type UART_FR_CTS_R = crate::BitReader;
#[doc = "Field `UART_FR_CTS` writer - Clear To Send"]
pub type UART_FR_CTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_FR_BUSY` reader - UART Busy"]
pub type UART_FR_BUSY_R = crate::BitReader;
#[doc = "Field `UART_FR_BUSY` writer - UART Busy"]
pub type UART_FR_BUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_FR_RXFE` reader - UART Receive FIFO Empty"]
pub type UART_FR_RXFE_R = crate::BitReader;
#[doc = "Field `UART_FR_RXFE` writer - UART Receive FIFO Empty"]
pub type UART_FR_RXFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_FR_TXFF` reader - UART Transmit FIFO Full"]
pub type UART_FR_TXFF_R = crate::BitReader;
#[doc = "Field `UART_FR_TXFF` writer - UART Transmit FIFO Full"]
pub type UART_FR_TXFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_FR_RXFF` reader - UART Receive FIFO Full"]
pub type UART_FR_RXFF_R = crate::BitReader;
#[doc = "Field `UART_FR_RXFF` writer - UART Receive FIFO Full"]
pub type UART_FR_RXFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_FR_TXFE` reader - UART Transmit FIFO Empty"]
pub type UART_FR_TXFE_R = crate::BitReader;
#[doc = "Field `UART_FR_TXFE` writer - UART Transmit FIFO Empty"]
pub type UART_FR_TXFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clear To Send"]
    #[inline(always)]
    pub fn uart_fr_cts(&self) -> UART_FR_CTS_R {
        UART_FR_CTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - UART Busy"]
    #[inline(always)]
    pub fn uart_fr_busy(&self) -> UART_FR_BUSY_R {
        UART_FR_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_rxfe(&self) -> UART_FR_RXFE_R {
        UART_FR_RXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Transmit FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_txff(&self) -> UART_FR_TXFF_R {
        UART_FR_TXFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Receive FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_rxff(&self) -> UART_FR_RXFF_R {
        UART_FR_RXFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Transmit FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_txfe(&self) -> UART_FR_TXFE_R {
        UART_FR_TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear To Send"]
    #[inline(always)]
    #[must_use]
    pub fn uart_fr_cts(&mut self) -> UART_FR_CTS_W<FR_SPEC, 0> {
        UART_FR_CTS_W::new(self)
    }
    #[doc = "Bit 3 - UART Busy"]
    #[inline(always)]
    #[must_use]
    pub fn uart_fr_busy(&mut self) -> UART_FR_BUSY_W<FR_SPEC, 3> {
        UART_FR_BUSY_W::new(self)
    }
    #[doc = "Bit 4 - UART Receive FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn uart_fr_rxfe(&mut self) -> UART_FR_RXFE_W<FR_SPEC, 4> {
        UART_FR_RXFE_W::new(self)
    }
    #[doc = "Bit 5 - UART Transmit FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn uart_fr_txff(&mut self) -> UART_FR_TXFF_W<FR_SPEC, 5> {
        UART_FR_TXFF_W::new(self)
    }
    #[doc = "Bit 6 - UART Receive FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn uart_fr_rxff(&mut self) -> UART_FR_RXFF_W<FR_SPEC, 6> {
        UART_FR_RXFF_W::new(self)
    }
    #[doc = "Bit 7 - UART Transmit FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn uart_fr_txfe(&mut self) -> UART_FR_TXFE_W<FR_SPEC, 7> {
        UART_FR_TXFE_W::new(self)
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
#[doc = "UART Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fr::W`](W) writer structure"]
impl crate::Writable for FR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
