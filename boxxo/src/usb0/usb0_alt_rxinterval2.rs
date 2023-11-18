#[doc = "Register `RXINTERVAL2` reader"]
pub type R = crate::R<USB0_ALT_RXINTERVAL2_SPEC>;
#[doc = "Register `RXINTERVAL2` writer"]
pub type W = crate::W<USB0_ALT_RXINTERVAL2_SPEC>;
#[doc = "Field `USB_RXINTERVAL2_NAKLMT` reader - NAK Limit"]
pub type USB_RXINTERVAL2_NAKLMT_R = crate::FieldReader;
#[doc = "Field `USB_RXINTERVAL2_NAKLMT` writer - NAK Limit"]
pub type USB_RXINTERVAL2_NAKLMT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - NAK Limit"]
    #[inline(always)]
    pub fn usb_rxinterval2_naklmt(&self) -> USB_RXINTERVAL2_NAKLMT_R {
        USB_RXINTERVAL2_NAKLMT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - NAK Limit"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxinterval2_naklmt(
        &mut self,
    ) -> USB_RXINTERVAL2_NAKLMT_W<USB0_ALT_RXINTERVAL2_SPEC, 0> {
        USB_RXINTERVAL2_NAKLMT_W::new(self)
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
#[doc = "USB Host Receive Polling Interval Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxinterval2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxinterval2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB0_ALT_RXINTERVAL2_SPEC;
impl crate::RegisterSpec for USB0_ALT_RXINTERVAL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb0_alt_rxinterval2::R`](R) reader structure"]
impl crate::Readable for USB0_ALT_RXINTERVAL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb0_alt_rxinterval2::W`](W) writer structure"]
impl crate::Writable for USB0_ALT_RXINTERVAL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXINTERVAL2 to value 0"]
impl crate::Resettable for USB0_ALT_RXINTERVAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
