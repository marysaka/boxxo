#[doc = "Register `TXHUBPORT0` reader"]
pub type R = crate::R<TXHUBPORT0_SPEC>;
#[doc = "Register `TXHUBPORT0` writer"]
pub type W = crate::W<TXHUBPORT0_SPEC>;
#[doc = "Field `USB_TXHUBPORT0_PORT` reader - Hub Port"]
pub type USB_TXHUBPORT0_PORT_R = crate::FieldReader;
#[doc = "Field `USB_TXHUBPORT0_PORT` writer - Hub Port"]
pub type USB_TXHUBPORT0_PORT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn usb_txhubport0_port(&self) -> USB_TXHUBPORT0_PORT_R {
        USB_TXHUBPORT0_PORT_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txhubport0_port(&mut self) -> USB_TXHUBPORT0_PORT_W<TXHUBPORT0_SPEC, 0> {
        USB_TXHUBPORT0_PORT_W::new(self)
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
#[doc = "USB Transmit Hub Port Endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXHUBPORT0_SPEC;
impl crate::RegisterSpec for TXHUBPORT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txhubport0::R`](R) reader structure"]
impl crate::Readable for TXHUBPORT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txhubport0::W`](W) writer structure"]
impl crate::Writable for TXHUBPORT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXHUBPORT0 to value 0"]
impl crate::Resettable for TXHUBPORT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
