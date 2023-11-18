#[doc = "Register `TXINTERVAL5` reader"]
pub type R = crate::R<TXINTERVAL5_SPEC>;
#[doc = "Register `TXINTERVAL5` writer"]
pub type W = crate::W<TXINTERVAL5_SPEC>;
#[doc = "Field `USB_TXINTERVAL5_TXPOLL` reader - TX Polling"]
pub type USB_TXINTERVAL5_TXPOLL_R = crate::FieldReader;
#[doc = "Field `USB_TXINTERVAL5_TXPOLL` writer - TX Polling"]
pub type USB_TXINTERVAL5_TXPOLL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TX Polling"]
    #[inline(always)]
    pub fn usb_txinterval5_txpoll(&self) -> USB_TXINTERVAL5_TXPOLL_R {
        USB_TXINTERVAL5_TXPOLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Polling"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txinterval5_txpoll(&mut self) -> USB_TXINTERVAL5_TXPOLL_W<TXINTERVAL5_SPEC, 0> {
        USB_TXINTERVAL5_TXPOLL_W::new(self)
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
#[doc = "USB Host Transmit Interval Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txinterval5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txinterval5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXINTERVAL5_SPEC;
impl crate::RegisterSpec for TXINTERVAL5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txinterval5::R`](R) reader structure"]
impl crate::Readable for TXINTERVAL5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txinterval5::W`](W) writer structure"]
impl crate::Writable for TXINTERVAL5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXINTERVAL5 to value 0"]
impl crate::Resettable for TXINTERVAL5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
