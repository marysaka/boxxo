#[doc = "Register `TXFUNCADDR4` reader"]
pub type R = crate::R<TXFUNCADDR4_SPEC>;
#[doc = "Register `TXFUNCADDR4` writer"]
pub type W = crate::W<TXFUNCADDR4_SPEC>;
#[doc = "Field `USB_TXFUNCADDR4_ADDR` reader - Device Address"]
pub type USB_TXFUNCADDR4_ADDR_R = crate::FieldReader;
#[doc = "Field `USB_TXFUNCADDR4_ADDR` writer - Device Address"]
pub type USB_TXFUNCADDR4_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn usb_txfuncaddr4_addr(&self) -> USB_TXFUNCADDR4_ADDR_R {
        USB_TXFUNCADDR4_ADDR_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txfuncaddr4_addr(&mut self) -> USB_TXFUNCADDR4_ADDR_W<TXFUNCADDR4_SPEC, 0> {
        USB_TXFUNCADDR4_ADDR_W::new(self)
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
#[doc = "USB Transmit Functional Address Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfuncaddr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfuncaddr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFUNCADDR4_SPEC;
impl crate::RegisterSpec for TXFUNCADDR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txfuncaddr4::R`](R) reader structure"]
impl crate::Readable for TXFUNCADDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txfuncaddr4::W`](W) writer structure"]
impl crate::Writable for TXFUNCADDR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFUNCADDR4 to value 0"]
impl crate::Resettable for TXFUNCADDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
