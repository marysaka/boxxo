#[doc = "Register `CSRL0` writer"]
pub type W = crate::W<CSRL0_SPEC>;
#[doc = "Field `USB_CSRL0_RXRDY` writer - Receive Packet Ready"]
pub type USB_CSRL0_RXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_TXRDY` writer - Transmit Packet Ready"]
pub type USB_CSRL0_TXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_STALLED` writer - Endpoint Stalled"]
pub type USB_CSRL0_STALLED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_DATAEND` writer - Data End"]
pub type USB_CSRL0_DATAEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_SETEND` writer - Setup End"]
pub type USB_CSRL0_SETEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_STALL` writer - Send Stall"]
pub type USB_CSRL0_STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_RXRDYC` writer - RXRDY Clear"]
pub type USB_CSRL0_RXRDYC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_SETENDC` writer - Setup End Clear"]
pub type USB_CSRL0_SETENDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_rxrdy(&mut self) -> USB_CSRL0_RXRDY_W<CSRL0_SPEC, 0> {
        USB_CSRL0_RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Packet Ready"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_txrdy(&mut self) -> USB_CSRL0_TXRDY_W<CSRL0_SPEC, 1> {
        USB_CSRL0_TXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint Stalled"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_stalled(&mut self) -> USB_CSRL0_STALLED_W<CSRL0_SPEC, 2> {
        USB_CSRL0_STALLED_W::new(self)
    }
    #[doc = "Bit 3 - Data End"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_dataend(&mut self) -> USB_CSRL0_DATAEND_W<CSRL0_SPEC, 3> {
        USB_CSRL0_DATAEND_W::new(self)
    }
    #[doc = "Bit 4 - Setup End"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_setend(&mut self) -> USB_CSRL0_SETEND_W<CSRL0_SPEC, 4> {
        USB_CSRL0_SETEND_W::new(self)
    }
    #[doc = "Bit 5 - Send Stall"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_stall(&mut self) -> USB_CSRL0_STALL_W<CSRL0_SPEC, 5> {
        USB_CSRL0_STALL_W::new(self)
    }
    #[doc = "Bit 6 - RXRDY Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_rxrdyc(&mut self) -> USB_CSRL0_RXRDYC_W<CSRL0_SPEC, 6> {
        USB_CSRL0_RXRDYC_W::new(self)
    }
    #[doc = "Bit 7 - Setup End Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_setendc(&mut self) -> USB_CSRL0_SETENDC_W<CSRL0_SPEC, 7> {
        USB_CSRL0_SETENDC_W::new(self)
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
#[doc = "USB Control and Status Endpoint 0 Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csrl0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRL0_SPEC;
impl crate::RegisterSpec for CSRL0_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`csrl0::W`](W) writer structure"]
impl crate::Writable for CSRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSRL0 to value 0"]
impl crate::Resettable for CSRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
