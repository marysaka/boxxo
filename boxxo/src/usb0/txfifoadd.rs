#[doc = "Register `TXFIFOADD` reader"]
pub type R = crate::R<TXFIFOADD_SPEC>;
#[doc = "Register `TXFIFOADD` writer"]
pub type W = crate::W<TXFIFOADD_SPEC>;
#[doc = "Field `USB_TXFIFOADD_ADDR` reader - Transmit/Receive Start Address"]
pub type USB_TXFIFOADD_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `USB_TXFIFOADD_ADDR` writer - Transmit/Receive Start Address"]
pub type USB_TXFIFOADD_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    pub fn usb_txfifoadd_addr(&self) -> USB_TXFIFOADD_ADDR_R {
        USB_TXFIFOADD_ADDR_R::new(self.bits & 0x01ff)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txfifoadd_addr(&mut self) -> USB_TXFIFOADD_ADDR_W<TXFIFOADD_SPEC, 0> {
        USB_TXFIFOADD_ADDR_W::new(self)
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
#[doc = "USB Transmit FIFO Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfifoadd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifoadd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFIFOADD_SPEC;
impl crate::RegisterSpec for TXFIFOADD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`txfifoadd::R`](R) reader structure"]
impl crate::Readable for TXFIFOADD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txfifoadd::W`](W) writer structure"]
impl crate::Writable for TXFIFOADD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFIFOADD to value 0"]
impl crate::Resettable for TXFIFOADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
