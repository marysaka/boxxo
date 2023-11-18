#[doc = "Register `RXCOUNT3` reader"]
pub type R = crate::R<RXCOUNT3_SPEC>;
#[doc = "Register `RXCOUNT3` writer"]
pub type W = crate::W<RXCOUNT3_SPEC>;
#[doc = "Field `USB_RXCOUNT3_COUNT` reader - Receive Packet Count"]
pub type USB_RXCOUNT3_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `USB_RXCOUNT3_COUNT` writer - Receive Packet Count"]
pub type USB_RXCOUNT3_COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline(always)]
    pub fn usb_rxcount3_count(&self) -> USB_RXCOUNT3_COUNT_R {
        USB_RXCOUNT3_COUNT_R::new(self.bits & 0x1fff)
    }
}
impl W {
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcount3_count(&mut self) -> USB_RXCOUNT3_COUNT_W<RXCOUNT3_SPEC, 0> {
        USB_RXCOUNT3_COUNT_W::new(self)
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
#[doc = "USB Receive Byte Count Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcount3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcount3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCOUNT3_SPEC;
impl crate::RegisterSpec for RXCOUNT3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rxcount3::R`](R) reader structure"]
impl crate::Readable for RXCOUNT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxcount3::W`](W) writer structure"]
impl crate::Writable for RXCOUNT3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCOUNT3 to value 0"]
impl crate::Resettable for RXCOUNT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
