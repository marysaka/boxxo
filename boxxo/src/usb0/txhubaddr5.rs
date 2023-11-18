#[doc = "Register `TXHUBADDR5` reader"]
pub type R = crate::R<TXHUBADDR5_SPEC>;
#[doc = "Register `TXHUBADDR5` writer"]
pub type W = crate::W<TXHUBADDR5_SPEC>;
#[doc = "Field `USB_TXHUBADDR5_ADDR` reader - Hub Address"]
pub type USB_TXHUBADDR5_ADDR_R = crate::FieldReader;
#[doc = "Field `USB_TXHUBADDR5_ADDR` writer - Hub Address"]
pub type USB_TXHUBADDR5_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    pub fn usb_txhubaddr5_addr(&self) -> USB_TXHUBADDR5_ADDR_R {
        USB_TXHUBADDR5_ADDR_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txhubaddr5_addr(&mut self) -> USB_TXHUBADDR5_ADDR_W<TXHUBADDR5_SPEC, 0> {
        USB_TXHUBADDR5_ADDR_W::new(self)
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
#[doc = "USB Transmit Hub Address Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubaddr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubaddr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXHUBADDR5_SPEC;
impl crate::RegisterSpec for TXHUBADDR5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txhubaddr5::R`](R) reader structure"]
impl crate::Readable for TXHUBADDR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txhubaddr5::W`](W) writer structure"]
impl crate::Writable for TXHUBADDR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXHUBADDR5 to value 0"]
impl crate::Resettable for TXHUBADDR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
