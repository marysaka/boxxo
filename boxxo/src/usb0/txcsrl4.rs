#[doc = "Register `TXCSRL4` reader"]
pub type R = crate::R<TXCSRL4_SPEC>;
#[doc = "Register `TXCSRL4` writer"]
pub type W = crate::W<TXCSRL4_SPEC>;
#[doc = "Field `USB_TXCSRL4_TXRDY` reader - Transmit Packet Ready"]
pub type USB_TXCSRL4_TXRDY_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_TXRDY` writer - Transmit Packet Ready"]
pub type USB_TXCSRL4_TXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRL4_FIFONE` reader - FIFO Not Empty"]
pub type USB_TXCSRL4_FIFONE_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_FIFONE` writer - FIFO Not Empty"]
pub type USB_TXCSRL4_FIFONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRL4_ERROR` reader - Error"]
pub type USB_TXCSRL4_ERROR_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_ERROR` writer - Error"]
pub type USB_TXCSRL4_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRL4_FLUSH` reader - Flush FIFO"]
pub type USB_TXCSRL4_FLUSH_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_FLUSH` writer - Flush FIFO"]
pub type USB_TXCSRL4_FLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRL4_SETUP` reader - Setup Packet"]
pub type USB_TXCSRL4_SETUP_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_SETUP` writer - Setup Packet"]
pub type USB_TXCSRL4_SETUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRL4_STALLED` reader - Endpoint Stalled"]
pub type USB_TXCSRL4_STALLED_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_STALLED` writer - Endpoint Stalled"]
pub type USB_TXCSRL4_STALLED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRL4_CLRDT` reader - Clear Data Toggle"]
pub type USB_TXCSRL4_CLRDT_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_CLRDT` writer - Clear Data Toggle"]
pub type USB_TXCSRL4_CLRDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRL4_NAKTO` reader - NAK Timeout"]
pub type USB_TXCSRL4_NAKTO_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_NAKTO` writer - NAK Timeout"]
pub type USB_TXCSRL4_NAKTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn usb_txcsrl4_txrdy(&self) -> USB_TXCSRL4_TXRDY_R {
        USB_TXCSRL4_TXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    pub fn usb_txcsrl4_fifone(&self) -> USB_TXCSRL4_FIFONE_R {
        USB_TXCSRL4_FIFONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_txcsrl4_error(&self) -> USB_TXCSRL4_ERROR_R {
        USB_TXCSRL4_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_txcsrl4_flush(&self) -> USB_TXCSRL4_FLUSH_R {
        USB_TXCSRL4_FLUSH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    pub fn usb_txcsrl4_setup(&self) -> USB_TXCSRL4_SETUP_R {
        USB_TXCSRL4_SETUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_txcsrl4_stalled(&self) -> USB_TXCSRL4_STALLED_R {
        USB_TXCSRL4_STALLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrl4_clrdt(&self) -> USB_TXCSRL4_CLRDT_R {
        USB_TXCSRL4_CLRDT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_txcsrl4_nakto(&self) -> USB_TXCSRL4_NAKTO_R {
        USB_TXCSRL4_NAKTO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_txrdy(&mut self) -> USB_TXCSRL4_TXRDY_W<TXCSRL4_SPEC, 0> {
        USB_TXCSRL4_TXRDY_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_fifone(&mut self) -> USB_TXCSRL4_FIFONE_W<TXCSRL4_SPEC, 1> {
        USB_TXCSRL4_FIFONE_W::new(self)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_error(&mut self) -> USB_TXCSRL4_ERROR_W<TXCSRL4_SPEC, 2> {
        USB_TXCSRL4_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_flush(&mut self) -> USB_TXCSRL4_FLUSH_W<TXCSRL4_SPEC, 3> {
        USB_TXCSRL4_FLUSH_W::new(self)
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_setup(&mut self) -> USB_TXCSRL4_SETUP_W<TXCSRL4_SPEC, 4> {
        USB_TXCSRL4_SETUP_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_stalled(&mut self) -> USB_TXCSRL4_STALLED_W<TXCSRL4_SPEC, 5> {
        USB_TXCSRL4_STALLED_W::new(self)
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_clrdt(&mut self) -> USB_TXCSRL4_CLRDT_W<TXCSRL4_SPEC, 6> {
        USB_TXCSRL4_CLRDT_W::new(self)
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_nakto(&mut self) -> USB_TXCSRL4_NAKTO_W<TXCSRL4_SPEC, 7> {
        USB_TXCSRL4_NAKTO_W::new(self)
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
#[doc = "USB Transmit Control and Status Endpoint 4 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCSRL4_SPEC;
impl crate::RegisterSpec for TXCSRL4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txcsrl4::R`](R) reader structure"]
impl crate::Readable for TXCSRL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txcsrl4::W`](W) writer structure"]
impl crate::Writable for TXCSRL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCSRL4 to value 0"]
impl crate::Resettable for TXCSRL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
