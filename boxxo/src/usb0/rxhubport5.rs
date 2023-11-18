#[doc = "Register `RXHUBPORT5` reader"]
pub type R = crate::R<RXHUBPORT5_SPEC>;
#[doc = "Register `RXHUBPORT5` writer"]
pub type W = crate::W<RXHUBPORT5_SPEC>;
#[doc = "Field `USB_RXHUBPORT5_PORT` reader - Hub Port"]
pub type USB_RXHUBPORT5_PORT_R = crate::FieldReader;
#[doc = "Field `USB_RXHUBPORT5_PORT` writer - Hub Port"]
pub type USB_RXHUBPORT5_PORT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn usb_rxhubport5_port(&self) -> USB_RXHUBPORT5_PORT_R {
        USB_RXHUBPORT5_PORT_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxhubport5_port(&mut self) -> USB_RXHUBPORT5_PORT_W<RXHUBPORT5_SPEC, 0> {
        USB_RXHUBPORT5_PORT_W::new(self)
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
#[doc = "USB Receive Hub Port Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubport5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubport5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXHUBPORT5_SPEC;
impl crate::RegisterSpec for RXHUBPORT5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxhubport5::R`](R) reader structure"]
impl crate::Readable for RXHUBPORT5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxhubport5::W`](W) writer structure"]
impl crate::Writable for RXHUBPORT5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXHUBPORT5 to value 0"]
impl crate::Resettable for RXHUBPORT5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
