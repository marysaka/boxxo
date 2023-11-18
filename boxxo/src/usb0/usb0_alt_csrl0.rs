#[doc = "Register `CSRL0` writer"]
pub type W = crate::W<USB0_ALT_CSRL0_SPEC>;
#[doc = "Field `USB_CSRL0_SETUP` writer - Setup Packet"]
pub type USB_CSRL0_SETUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_ERROR` writer - Error"]
pub type USB_CSRL0_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_REQPKT` writer - Request Packet"]
pub type USB_CSRL0_REQPKT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_STATUS` writer - STATUS Packet"]
pub type USB_CSRL0_STATUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRL0_NAKTO` writer - NAK Timeout"]
pub type USB_CSRL0_NAKTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 3 - Setup Packet"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_setup(&mut self) -> USB_CSRL0_SETUP_W<USB0_ALT_CSRL0_SPEC, 3> {
        USB_CSRL0_SETUP_W::new(self)
    }
    #[doc = "Bit 4 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_error(&mut self) -> USB_CSRL0_ERROR_W<USB0_ALT_CSRL0_SPEC, 4> {
        USB_CSRL0_ERROR_W::new(self)
    }
    #[doc = "Bit 5 - Request Packet"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_reqpkt(&mut self) -> USB_CSRL0_REQPKT_W<USB0_ALT_CSRL0_SPEC, 5> {
        USB_CSRL0_REQPKT_W::new(self)
    }
    #[doc = "Bit 6 - STATUS Packet"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_status(&mut self) -> USB_CSRL0_STATUS_W<USB0_ALT_CSRL0_SPEC, 6> {
        USB_CSRL0_STATUS_W::new(self)
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrl0_nakto(&mut self) -> USB_CSRL0_NAKTO_W<USB0_ALT_CSRL0_SPEC, 7> {
        USB_CSRL0_NAKTO_W::new(self)
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
#[doc = "USB Control and Status Endpoint 0 Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_csrl0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB0_ALT_CSRL0_SPEC;
impl crate::RegisterSpec for USB0_ALT_CSRL0_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`usb0_alt_csrl0::W`](W) writer structure"]
impl crate::Writable for USB0_ALT_CSRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSRL0 to value 0"]
impl crate::Resettable for USB0_ALT_CSRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
