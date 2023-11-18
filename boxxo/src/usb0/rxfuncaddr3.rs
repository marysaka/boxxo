#[doc = "Register `RXFUNCADDR3` reader"]
pub type R = crate::R<RXFUNCADDR3_SPEC>;
#[doc = "Register `RXFUNCADDR3` writer"]
pub type W = crate::W<RXFUNCADDR3_SPEC>;
#[doc = "Field `USB_RXFUNCADDR3_ADDR` reader - Device Address"]
pub type USB_RXFUNCADDR3_ADDR_R = crate::FieldReader;
#[doc = "Field `USB_RXFUNCADDR3_ADDR` writer - Device Address"]
pub type USB_RXFUNCADDR3_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn usb_rxfuncaddr3_addr(&self) -> USB_RXFUNCADDR3_ADDR_R {
        USB_RXFUNCADDR3_ADDR_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxfuncaddr3_addr(&mut self) -> USB_RXFUNCADDR3_ADDR_W<RXFUNCADDR3_SPEC, 0> {
        USB_RXFUNCADDR3_ADDR_W::new(self)
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
#[doc = "USB Receive Functional Address Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfuncaddr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfuncaddr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFUNCADDR3_SPEC;
impl crate::RegisterSpec for RXFUNCADDR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxfuncaddr3::R`](R) reader structure"]
impl crate::Readable for RXFUNCADDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxfuncaddr3::W`](W) writer structure"]
impl crate::Writable for RXFUNCADDR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFUNCADDR3 to value 0"]
impl crate::Resettable for RXFUNCADDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
