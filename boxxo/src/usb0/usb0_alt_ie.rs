#[doc = "Register `IE` reader"]
pub type R = crate::R<USB0_ALT_IE_SPEC>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<USB0_ALT_IE_SPEC>;
#[doc = "Field `USB_IE_RESET` reader - Enable RESET Interrupt"]
pub type USB_IE_RESET_R = crate::BitReader;
#[doc = "Field `USB_IE_RESET` writer - Enable RESET Interrupt"]
pub type USB_IE_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Enable RESET Interrupt"]
    #[inline(always)]
    pub fn usb_ie_reset(&self) -> USB_IE_RESET_R {
        USB_IE_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enable RESET Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ie_reset(&mut self) -> USB_IE_RESET_W<USB0_ALT_IE_SPEC, 2> {
        USB_IE_RESET_W::new(self)
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
#[doc = "USB Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB0_ALT_IE_SPEC;
impl crate::RegisterSpec for USB0_ALT_IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb0_alt_ie::R`](R) reader structure"]
impl crate::Readable for USB0_ALT_IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb0_alt_ie::W`](W) writer structure"]
impl crate::Writable for USB0_ALT_IE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for USB0_ALT_IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
