#[doc = "Register `RXINTERVAL4` reader"]
pub type R = crate::R<RXINTERVAL4_SPEC>;
#[doc = "Register `RXINTERVAL4` writer"]
pub type W = crate::W<RXINTERVAL4_SPEC>;
#[doc = "Field `USB_RXINTERVAL4_TXPOLL` reader - RX Polling"]
pub type USB_RXINTERVAL4_TXPOLL_R = crate::FieldReader;
#[doc = "Field `USB_RXINTERVAL4_TXPOLL` writer - RX Polling"]
pub type USB_RXINTERVAL4_TXPOLL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - RX Polling"]
    #[inline(always)]
    pub fn usb_rxinterval4_txpoll(&self) -> USB_RXINTERVAL4_TXPOLL_R {
        USB_RXINTERVAL4_TXPOLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RX Polling"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxinterval4_txpoll(&mut self) -> USB_RXINTERVAL4_TXPOLL_W<RXINTERVAL4_SPEC, 0> {
        USB_RXINTERVAL4_TXPOLL_W::new(self)
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
#[doc = "USB Host Receive Polling Interval Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxinterval4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxinterval4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXINTERVAL4_SPEC;
impl crate::RegisterSpec for RXINTERVAL4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxinterval4::R`](R) reader structure"]
impl crate::Readable for RXINTERVAL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxinterval4::W`](W) writer structure"]
impl crate::Writable for RXINTERVAL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXINTERVAL4 to value 0"]
impl crate::Resettable for RXINTERVAL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
