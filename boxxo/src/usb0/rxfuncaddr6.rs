#[doc = "Register `RXFUNCADDR6` reader"]
pub type R = crate::R<RXFUNCADDR6_SPEC>;
#[doc = "Register `RXFUNCADDR6` writer"]
pub type W = crate::W<RXFUNCADDR6_SPEC>;
#[doc = "Field `USB_RXFUNCADDR6_ADDR` reader - Device Address"]
pub type USB_RXFUNCADDR6_ADDR_R = crate::FieldReader;
#[doc = "Field `USB_RXFUNCADDR6_ADDR` writer - Device Address"]
pub type USB_RXFUNCADDR6_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn usb_rxfuncaddr6_addr(&self) -> USB_RXFUNCADDR6_ADDR_R {
        USB_RXFUNCADDR6_ADDR_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxfuncaddr6_addr(&mut self) -> USB_RXFUNCADDR6_ADDR_W<RXFUNCADDR6_SPEC, 0> {
        USB_RXFUNCADDR6_ADDR_W::new(self)
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
#[doc = "USB Receive Functional Address Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfuncaddr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfuncaddr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFUNCADDR6_SPEC;
impl crate::RegisterSpec for RXFUNCADDR6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxfuncaddr6::R`](R) reader structure"]
impl crate::Readable for RXFUNCADDR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxfuncaddr6::W`](W) writer structure"]
impl crate::Writable for RXFUNCADDR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFUNCADDR6 to value 0"]
impl crate::Resettable for RXFUNCADDR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
