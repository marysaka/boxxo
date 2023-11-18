#[doc = "Register `RXCSRL5` reader"]
pub type R = crate::R<RXCSRL5_SPEC>;
#[doc = "Register `RXCSRL5` writer"]
pub type W = crate::W<RXCSRL5_SPEC>;
#[doc = "Field `USB_RXCSRL5_RXRDY` reader - Receive Packet Ready"]
pub type USB_RXCSRL5_RXRDY_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL5_RXRDY` writer - Receive Packet Ready"]
pub type USB_RXCSRL5_RXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL5_FULL` reader - FIFO Full"]
pub type USB_RXCSRL5_FULL_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL5_FULL` writer - FIFO Full"]
pub type USB_RXCSRL5_FULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL5_OVER` reader - Overrun"]
pub type USB_RXCSRL5_OVER_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL5_OVER` writer - Overrun"]
pub type USB_RXCSRL5_OVER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL5_DATAERR` reader - Data Error"]
pub type USB_RXCSRL5_DATAERR_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL5_DATAERR` writer - Data Error"]
pub type USB_RXCSRL5_DATAERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL5_FLUSH` reader - Flush FIFO"]
pub type USB_RXCSRL5_FLUSH_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL5_FLUSH` writer - Flush FIFO"]
pub type USB_RXCSRL5_FLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL5_STALL` reader - Send STALL"]
pub type USB_RXCSRL5_STALL_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL5_STALL` writer - Send STALL"]
pub type USB_RXCSRL5_STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL5_STALLED` reader - Endpoint Stalled"]
pub type USB_RXCSRL5_STALLED_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL5_STALLED` writer - Endpoint Stalled"]
pub type USB_RXCSRL5_STALLED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL5_CLRDT` reader - Clear Data Toggle"]
pub type USB_RXCSRL5_CLRDT_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL5_CLRDT` writer - Clear Data Toggle"]
pub type USB_RXCSRL5_CLRDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    pub fn usb_rxcsrl5_rxrdy(&self) -> USB_RXCSRL5_RXRDY_R {
        USB_RXCSRL5_RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    pub fn usb_rxcsrl5_full(&self) -> USB_RXCSRL5_FULL_R {
        USB_RXCSRL5_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    pub fn usb_rxcsrl5_over(&self) -> USB_RXCSRL5_OVER_R {
        USB_RXCSRL5_OVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Error"]
    #[inline(always)]
    pub fn usb_rxcsrl5_dataerr(&self) -> USB_RXCSRL5_DATAERR_R {
        USB_RXCSRL5_DATAERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_rxcsrl5_flush(&self) -> USB_RXCSRL5_FLUSH_R {
        USB_RXCSRL5_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Send STALL"]
    #[inline(always)]
    pub fn usb_rxcsrl5_stall(&self) -> USB_RXCSRL5_STALL_R {
        USB_RXCSRL5_STALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_rxcsrl5_stalled(&self) -> USB_RXCSRL5_STALLED_R {
        USB_RXCSRL5_STALLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrl5_clrdt(&self) -> USB_RXCSRL5_CLRDT_R {
        USB_RXCSRL5_CLRDT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl5_rxrdy(&mut self) -> USB_RXCSRL5_RXRDY_W<RXCSRL5_SPEC, 0> {
        USB_RXCSRL5_RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl5_full(&mut self) -> USB_RXCSRL5_FULL_W<RXCSRL5_SPEC, 1> {
        USB_RXCSRL5_FULL_W::new(self)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl5_over(&mut self) -> USB_RXCSRL5_OVER_W<RXCSRL5_SPEC, 2> {
        USB_RXCSRL5_OVER_W::new(self)
    }
    #[doc = "Bit 3 - Data Error"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl5_dataerr(&mut self) -> USB_RXCSRL5_DATAERR_W<RXCSRL5_SPEC, 3> {
        USB_RXCSRL5_DATAERR_W::new(self)
    }
    #[doc = "Bit 4 - Flush FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl5_flush(&mut self) -> USB_RXCSRL5_FLUSH_W<RXCSRL5_SPEC, 4> {
        USB_RXCSRL5_FLUSH_W::new(self)
    }
    #[doc = "Bit 5 - Send STALL"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl5_stall(&mut self) -> USB_RXCSRL5_STALL_W<RXCSRL5_SPEC, 5> {
        USB_RXCSRL5_STALL_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint Stalled"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl5_stalled(&mut self) -> USB_RXCSRL5_STALLED_W<RXCSRL5_SPEC, 6> {
        USB_RXCSRL5_STALLED_W::new(self)
    }
    #[doc = "Bit 7 - Clear Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl5_clrdt(&mut self) -> USB_RXCSRL5_CLRDT_W<RXCSRL5_SPEC, 7> {
        USB_RXCSRL5_CLRDT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Receive Control and Status Endpoint 5 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrl5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrl5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCSRL5_SPEC;
impl crate::RegisterSpec for RXCSRL5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcsrl5::R`](R) reader structure"]
impl crate::Readable for RXCSRL5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxcsrl5::W`](W) writer structure"]
impl crate::Writable for RXCSRL5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCSRL5 to value 0"]
impl crate::Resettable for RXCSRL5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
