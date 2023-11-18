#[doc = "Register `CSRH0` writer"]
pub type W = crate::W<CSRH0_SPEC>;
#[doc = "Field `USB_CSRH0_FLUSH` writer - Flush FIFO"]
pub type USB_CSRH0_FLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRH0_DT` writer - Data Toggle"]
pub type USB_CSRH0_DT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_CSRH0_DTWE` writer - Data Toggle Write Enable"]
pub type USB_CSRH0_DTWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Flush FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrh0_flush(&mut self) -> USB_CSRH0_FLUSH_W<CSRH0_SPEC, 0> {
        USB_CSRH0_FLUSH_W::new(self)
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrh0_dt(&mut self) -> USB_CSRH0_DT_W<CSRH0_SPEC, 1> {
        USB_CSRH0_DT_W::new(self)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_csrh0_dtwe(&mut self) -> USB_CSRH0_DTWE_W<CSRH0_SPEC, 2> {
        USB_CSRH0_DTWE_W::new(self)
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
#[doc = "USB Control and Status Endpoint 0 High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csrh0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRH0_SPEC;
impl crate::RegisterSpec for CSRH0_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`csrh0::W`](W) writer structure"]
impl crate::Writable for CSRH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSRH0 to value 0"]
impl crate::Resettable for CSRH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
