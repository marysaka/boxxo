#[doc = "Register `TXHUBPORT1` reader"]
pub type R = crate::R<TXHUBPORT1_SPEC>;
#[doc = "Register `TXHUBPORT1` writer"]
pub type W = crate::W<TXHUBPORT1_SPEC>;
#[doc = "Field `USB_TXHUBPORT1_PORT` reader - Hub Port"]
pub type USB_TXHUBPORT1_PORT_R = crate::FieldReader;
#[doc = "Field `USB_TXHUBPORT1_PORT` writer - Hub Port"]
pub type USB_TXHUBPORT1_PORT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn usb_txhubport1_port(&self) -> USB_TXHUBPORT1_PORT_R {
        USB_TXHUBPORT1_PORT_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txhubport1_port(&mut self) -> USB_TXHUBPORT1_PORT_W<TXHUBPORT1_SPEC, 0> {
        USB_TXHUBPORT1_PORT_W::new(self)
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
#[doc = "USB Transmit Hub Port Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXHUBPORT1_SPEC;
impl crate::RegisterSpec for TXHUBPORT1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txhubport1::R`](R) reader structure"]
impl crate::Readable for TXHUBPORT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txhubport1::W`](W) writer structure"]
impl crate::Writable for TXHUBPORT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXHUBPORT1 to value 0"]
impl crate::Resettable for TXHUBPORT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
