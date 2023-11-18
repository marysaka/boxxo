#[doc = "Register `RXHUBADDR4` reader"]
pub type R = crate::R<RXHUBADDR4_SPEC>;
#[doc = "Register `RXHUBADDR4` writer"]
pub type W = crate::W<RXHUBADDR4_SPEC>;
#[doc = "Field `USB_RXHUBADDR4_ADDR` reader - Hub Address"]
pub type USB_RXHUBADDR4_ADDR_R = crate::FieldReader;
#[doc = "Field `USB_RXHUBADDR4_ADDR` writer - Hub Address"]
pub type USB_RXHUBADDR4_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    pub fn usb_rxhubaddr4_addr(&self) -> USB_RXHUBADDR4_ADDR_R {
        USB_RXHUBADDR4_ADDR_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxhubaddr4_addr(&mut self) -> USB_RXHUBADDR4_ADDR_W<RXHUBADDR4_SPEC, 0> {
        USB_RXHUBADDR4_ADDR_W::new(self)
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
#[doc = "USB Receive Hub Address Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubaddr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubaddr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXHUBADDR4_SPEC;
impl crate::RegisterSpec for RXHUBADDR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxhubaddr4::R`](R) reader structure"]
impl crate::Readable for RXHUBADDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxhubaddr4::W`](W) writer structure"]
impl crate::Writable for RXHUBADDR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXHUBADDR4 to value 0"]
impl crate::Resettable for RXHUBADDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
