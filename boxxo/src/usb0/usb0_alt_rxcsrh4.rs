#[doc = "Register `RXCSRH4` reader"]
pub type R = crate::R<USB0_ALT_RXCSRH4_SPEC>;
#[doc = "Register `RXCSRH4` writer"]
pub type W = crate::W<USB0_ALT_RXCSRH4_SPEC>;
#[doc = "Field `USB_RXCSRH4_DISNYET` reader - Disable NYET"]
pub type USB_RXCSRH4_DISNYET_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH4_DISNYET` writer - Disable NYET"]
pub type USB_RXCSRH4_DISNYET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH4_ISO` reader - Isochronous Transfers"]
pub type USB_RXCSRH4_ISO_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH4_ISO` writer - Isochronous Transfers"]
pub type USB_RXCSRH4_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    pub fn usb_rxcsrh4_disnyet(&self) -> USB_RXCSRH4_DISNYET_R {
        USB_RXCSRH4_DISNYET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn usb_rxcsrh4_iso(&self) -> USB_RXCSRH4_ISO_R {
        USB_RXCSRH4_ISO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh4_disnyet(&mut self) -> USB_RXCSRH4_DISNYET_W<USB0_ALT_RXCSRH4_SPEC, 4> {
        USB_RXCSRH4_DISNYET_W::new(self)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh4_iso(&mut self) -> USB_RXCSRH4_ISO_W<USB0_ALT_RXCSRH4_SPEC, 6> {
        USB_RXCSRH4_ISO_W::new(self)
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
#[doc = "USB Receive Control and Status Endpoint 4 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrh4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrh4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB0_ALT_RXCSRH4_SPEC;
impl crate::RegisterSpec for USB0_ALT_RXCSRH4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb0_alt_rxcsrh4::R`](R) reader structure"]
impl crate::Readable for USB0_ALT_RXCSRH4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb0_alt_rxcsrh4::W`](W) writer structure"]
impl crate::Writable for USB0_ALT_RXCSRH4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCSRH4 to value 0"]
impl crate::Resettable for USB0_ALT_RXCSRH4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
