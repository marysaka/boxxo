#[doc = "Register `RXCOUNT5` reader"]
pub type R = crate::R<RXCOUNT5_SPEC>;
#[doc = "Register `RXCOUNT5` writer"]
pub type W = crate::W<RXCOUNT5_SPEC>;
#[doc = "Field `USB_RXCOUNT5_COUNT` reader - Receive Packet Count"]
pub type USB_RXCOUNT5_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `USB_RXCOUNT5_COUNT` writer - Receive Packet Count"]
pub type USB_RXCOUNT5_COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline(always)]
    pub fn usb_rxcount5_count(&self) -> USB_RXCOUNT5_COUNT_R {
        USB_RXCOUNT5_COUNT_R::new(self.bits & 0x1fff)
    }
}
impl W {
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcount5_count(&mut self) -> USB_RXCOUNT5_COUNT_W<RXCOUNT5_SPEC, 0> {
        USB_RXCOUNT5_COUNT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Receive Byte Count Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcount5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcount5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCOUNT5_SPEC;
impl crate::RegisterSpec for RXCOUNT5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rxcount5::R`](R) reader structure"]
impl crate::Readable for RXCOUNT5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxcount5::W`](W) writer structure"]
impl crate::Writable for RXCOUNT5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCOUNT5 to value 0"]
impl crate::Resettable for RXCOUNT5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
