#[doc = "Register `RXHUBADDR3` reader"]
pub type R = crate::R<RXHUBADDR3_SPEC>;
#[doc = "Register `RXHUBADDR3` writer"]
pub type W = crate::W<RXHUBADDR3_SPEC>;
#[doc = "Field `USB_RXHUBADDR3_ADDR` reader - Hub Address"]
pub type USB_RXHUBADDR3_ADDR_R = crate::FieldReader;
#[doc = "Field `USB_RXHUBADDR3_ADDR` writer - Hub Address"]
pub type USB_RXHUBADDR3_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    pub fn usb_rxhubaddr3_addr(&self) -> USB_RXHUBADDR3_ADDR_R {
        USB_RXHUBADDR3_ADDR_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxhubaddr3_addr(&mut self) -> USB_RXHUBADDR3_ADDR_W<RXHUBADDR3_SPEC, 0> {
        USB_RXHUBADDR3_ADDR_W::new(self)
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
#[doc = "USB Receive Hub Address Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubaddr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubaddr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXHUBADDR3_SPEC;
impl crate::RegisterSpec for RXHUBADDR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxhubaddr3::R`](R) reader structure"]
impl crate::Readable for RXHUBADDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxhubaddr3::W`](W) writer structure"]
impl crate::Writable for RXHUBADDR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXHUBADDR3 to value 0"]
impl crate::Resettable for RXHUBADDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
