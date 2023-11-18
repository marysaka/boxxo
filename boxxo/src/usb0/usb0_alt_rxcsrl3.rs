#[doc = "Register `RXCSRL3` reader"]
pub type R = crate::R<USB0_ALT_RXCSRL3_SPEC>;
#[doc = "Register `RXCSRL3` writer"]
pub type W = crate::W<USB0_ALT_RXCSRL3_SPEC>;
#[doc = "Field `USB_RXCSRL3_ERROR` reader - Error"]
pub type USB_RXCSRL3_ERROR_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL3_ERROR` writer - Error"]
pub type USB_RXCSRL3_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL3_NAKTO` reader - NAK Timeout"]
pub type USB_RXCSRL3_NAKTO_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL3_NAKTO` writer - NAK Timeout"]
pub type USB_RXCSRL3_NAKTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRL3_REQPKT` reader - Request Packet"]
pub type USB_RXCSRL3_REQPKT_R = crate::BitReader;
#[doc = "Field `USB_RXCSRL3_REQPKT` writer - Request Packet"]
pub type USB_RXCSRL3_REQPKT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_rxcsrl3_error(&self) -> USB_RXCSRL3_ERROR_R {
        USB_RXCSRL3_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_rxcsrl3_nakto(&self) -> USB_RXCSRL3_NAKTO_R {
        USB_RXCSRL3_NAKTO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Request Packet"]
    #[inline(always)]
    pub fn usb_rxcsrl3_reqpkt(&self) -> USB_RXCSRL3_REQPKT_R {
        USB_RXCSRL3_REQPKT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl3_error(&mut self) -> USB_RXCSRL3_ERROR_W<USB0_ALT_RXCSRL3_SPEC, 2> {
        USB_RXCSRL3_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - NAK Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl3_nakto(&mut self) -> USB_RXCSRL3_NAKTO_W<USB0_ALT_RXCSRL3_SPEC, 3> {
        USB_RXCSRL3_NAKTO_W::new(self)
    }
    #[doc = "Bit 5 - Request Packet"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrl3_reqpkt(&mut self) -> USB_RXCSRL3_REQPKT_W<USB0_ALT_RXCSRL3_SPEC, 5> {
        USB_RXCSRL3_REQPKT_W::new(self)
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
#[doc = "USB Receive Control and Status Endpoint 3 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB0_ALT_RXCSRL3_SPEC;
impl crate::RegisterSpec for USB0_ALT_RXCSRL3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb0_alt_rxcsrl3::R`](R) reader structure"]
impl crate::Readable for USB0_ALT_RXCSRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb0_alt_rxcsrl3::W`](W) writer structure"]
impl crate::Writable for USB0_ALT_RXCSRL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCSRL3 to value 0"]
impl crate::Resettable for USB0_ALT_RXCSRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
