#[doc = "Register `RXCSRH6` reader"]
pub type R = crate::R<USB0_ALT_RXCSRH6_SPEC>;
#[doc = "Register `RXCSRH6` writer"]
pub type W = crate::W<USB0_ALT_RXCSRH6_SPEC>;
#[doc = "Field `USB_RXCSRH6_DISNYET` reader - Disable NYET"]
pub type USB_RXCSRH6_DISNYET_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH6_DISNYET` writer - Disable NYET"]
pub type USB_RXCSRH6_DISNYET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH6_ISO` reader - Isochronous Transfers"]
pub type USB_RXCSRH6_ISO_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH6_ISO` writer - Isochronous Transfers"]
pub type USB_RXCSRH6_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    pub fn usb_rxcsrh6_disnyet(&self) -> USB_RXCSRH6_DISNYET_R {
        USB_RXCSRH6_DISNYET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn usb_rxcsrh6_iso(&self) -> USB_RXCSRH6_ISO_R {
        USB_RXCSRH6_ISO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh6_disnyet(&mut self) -> USB_RXCSRH6_DISNYET_W<USB0_ALT_RXCSRH6_SPEC, 4> {
        USB_RXCSRH6_DISNYET_W::new(self)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh6_iso(&mut self) -> USB_RXCSRH6_ISO_W<USB0_ALT_RXCSRH6_SPEC, 6> {
        USB_RXCSRH6_ISO_W::new(self)
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
#[doc = "USB Receive Control and Status Endpoint 6 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrh6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrh6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB0_ALT_RXCSRH6_SPEC;
impl crate::RegisterSpec for USB0_ALT_RXCSRH6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb0_alt_rxcsrh6::R`](R) reader structure"]
impl crate::Readable for USB0_ALT_RXCSRH6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb0_alt_rxcsrh6::W`](W) writer structure"]
impl crate::Writable for USB0_ALT_RXCSRH6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCSRH6 to value 0"]
impl crate::Resettable for USB0_ALT_RXCSRH6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
