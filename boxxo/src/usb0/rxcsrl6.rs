#[doc = "Register `RXCSRL6` reader"]
pub type R = crate::R<RXCSRL6_SPEC>;
#[doc = "Register `RXCSRL6` writer"]
pub type W = crate::W<RXCSRL6_SPEC>;
#[doc = "Field `USB_RXCSRL6_RXRDY` reader - Receive Packet Ready"]
pub type USB_RXCSRL6_RXRDY_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL6_RXRDY` writer - Receive Packet Ready"]
pub type USB_RXCSRL6_RXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL6_FULL` reader - FIFO Full"]
pub type USB_RXCSRL6_FULL_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL6_FULL` writer - FIFO Full"]
pub type USB_RXCSRL6_FULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL6_OVER` reader - Overrun"]
pub type USB_RXCSRL6_OVER_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL6_OVER` writer - Overrun"]
pub type USB_RXCSRL6_OVER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL6_DATAERR` reader - Data Error"]
pub type USB_RXCSRL6_DATAERR_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL6_DATAERR` writer - Data Error"]
pub type USB_RXCSRL6_DATAERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL6_FLUSH` reader - Flush FIFO"]
pub type USB_RXCSRL6_FLUSH_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL6_FLUSH` writer - Flush FIFO"]
pub type USB_RXCSRL6_FLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL6_STALL` reader - Send STALL"]
pub type USB_RXCSRL6_STALL_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL6_STALL` writer - Send STALL"]
pub type USB_RXCSRL6_STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL6_STALLED` reader - Endpoint Stalled"]
pub type USB_RXCSRL6_STALLED_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL6_STALLED` writer - Endpoint Stalled"]
pub type USB_RXCSRL6_STALLED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL6_CLRDT` reader - Clear Data Toggle"]
pub type USB_RXCSRL6_CLRDT_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL6_CLRDT` writer - Clear Data Toggle"]
pub type USB_RXCSRL6_CLRDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    pub fn usb_rxcsrl6_rxrdy(&self) -> USB_RXCSRL6_RXRDY_R {
        USB_RXCSRL6_RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    pub fn usb_rxcsrl6_full(&self) -> USB_RXCSRL6_FULL_R {
        USB_RXCSRL6_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    pub fn usb_rxcsrl6_over(&self) -> USB_RXCSRL6_OVER_R {
        USB_RXCSRL6_OVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Error"]
    #[inline(always)]
    pub fn usb_rxcsrl6_dataerr(&self) -> USB_RXCSRL6_DATAERR_R {
        USB_RXCSRL6_DATAERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_rxcsrl6_flush(&self) -> USB_RXCSRL6_FLUSH_R {
        USB_RXCSRL6_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Send STALL"]
    #[inline(always)]
    pub fn usb_rxcsrl6_stall(&self) -> USB_RXCSRL6_STALL_R {
        USB_RXCSRL6_STALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_rxcsrl6_stalled(&self) -> USB_RXCSRL6_STALLED_R {
        USB_RXCSRL6_STALLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrl6_clrdt(&self) -> USB_RXCSRL6_CLRDT_R {
        USB_RXCSRL6_CLRDT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl6_rxrdy(&mut self) -> USB_RXCSRL6_RXRDY_W<RXCSRL6_SPEC, 0> {
        USB_RXCSRL6_RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl6_full(&mut self) -> USB_RXCSRL6_FULL_W<RXCSRL6_SPEC, 1> {
        USB_RXCSRL6_FULL_W::new(self)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl6_over(&mut self) -> USB_RXCSRL6_OVER_W<RXCSRL6_SPEC, 2> {
        USB_RXCSRL6_OVER_W::new(self)
    }
    #[doc = "Bit 3 - Data Error"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl6_dataerr(&mut self) -> USB_RXCSRL6_DATAERR_W<RXCSRL6_SPEC, 3> {
        USB_RXCSRL6_DATAERR_W::new(self)
    }
    #[doc = "Bit 4 - Flush FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl6_flush(&mut self) -> USB_RXCSRL6_FLUSH_W<RXCSRL6_SPEC, 4> {
        USB_RXCSRL6_FLUSH_W::new(self)
    }
    #[doc = "Bit 5 - Send STALL"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl6_stall(&mut self) -> USB_RXCSRL6_STALL_W<RXCSRL6_SPEC, 5> {
        USB_RXCSRL6_STALL_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint Stalled"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl6_stalled(&mut self) -> USB_RXCSRL6_STALLED_W<RXCSRL6_SPEC, 6> {
        USB_RXCSRL6_STALLED_W::new(self)
    }
    #[doc = "Bit 7 - Clear Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl6_clrdt(&mut self) -> USB_RXCSRL6_CLRDT_W<RXCSRL6_SPEC, 7> {
        USB_RXCSRL6_CLRDT_W::new(self)
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
#[doc = "USB Receive Control and Status Endpoint 6 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrl6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrl6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCSRL6_SPEC;
impl crate::RegisterSpec for RXCSRL6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcsrl6::R`](R) reader structure"]
impl crate::Readable for RXCSRL6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxcsrl6::W`](W) writer structure"]
impl crate::Writable for RXCSRL6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCSRL6 to value 0"]
impl crate::Resettable for RXCSRL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
