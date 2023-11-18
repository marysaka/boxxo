#[doc = "Register `TXCSRL4` reader"]
pub type R = crate::R<USB0_ALT_TXCSRL4_SPEC>;
#[doc = "Register `TXCSRL4` writer"]
pub type W = crate::W<USB0_ALT_TXCSRL4_SPEC>;
#[doc = "Field `USB_TXCSRL4_UNDRN` reader - Underrun"]
pub type USB_TXCSRL4_UNDRN_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_UNDRN` writer - Underrun"]
pub type USB_TXCSRL4_UNDRN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRL4_STALL` reader - Send STALL"]
pub type USB_TXCSRL4_STALL_R = crate::BitReader;
#[doc = "Field `USB_TXCSRL4_STALL` writer - Send STALL"]
pub type USB_TXCSRL4_STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Underrun"]
    #[inline(always)]
    pub fn usb_txcsrl4_undrn(&self) -> USB_TXCSRL4_UNDRN_R {
        USB_TXCSRL4_UNDRN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Send STALL"]
    #[inline(always)]
    pub fn usb_txcsrl4_stall(&self) -> USB_TXCSRL4_STALL_R {
        USB_TXCSRL4_STALL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_undrn(&mut self) -> USB_TXCSRL4_UNDRN_W<USB0_ALT_TXCSRL4_SPEC, 2> {
        USB_TXCSRL4_UNDRN_W::new(self)
    }
    #[doc = "Bit 4 - Send STALL"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrl4_stall(&mut self) -> USB_TXCSRL4_STALL_W<USB0_ALT_TXCSRL4_SPEC, 4> {
        USB_TXCSRL4_STALL_W::new(self)
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
#[doc = "USB Transmit Control and Status Endpoint 4 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txcsrl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txcsrl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB0_ALT_TXCSRL4_SPEC;
impl crate::RegisterSpec for USB0_ALT_TXCSRL4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb0_alt_txcsrl4::R`](R) reader structure"]
impl crate::Readable for USB0_ALT_TXCSRL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb0_alt_txcsrl4::W`](W) writer structure"]
impl crate::Writable for USB0_ALT_TXCSRL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCSRL4 to value 0"]
impl crate::Resettable for USB0_ALT_TXCSRL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
