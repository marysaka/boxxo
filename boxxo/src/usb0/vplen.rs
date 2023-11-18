#[doc = "Register `VPLEN` reader"]
pub type R = crate::R<VPLEN_SPEC>;
#[doc = "Register `VPLEN` writer"]
pub type W = crate::W<VPLEN_SPEC>;
#[doc = "Field `USB_VPLEN_VPLEN` reader - VBUS Pulse Length"]
pub type USB_VPLEN_VPLEN_R = crate::FieldReader;
#[doc = "Field `USB_VPLEN_VPLEN` writer - VBUS Pulse Length"]
pub type USB_VPLEN_VPLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - VBUS Pulse Length"]
    #[inline(always)]
    pub fn usb_vplen_vplen(&self) -> USB_VPLEN_VPLEN_R {
        USB_VPLEN_VPLEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - VBUS Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vplen_vplen(&mut self) -> USB_VPLEN_VPLEN_W<VPLEN_SPEC, 0> {
        USB_VPLEN_VPLEN_W::new(self)
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
#[doc = "USB OTG VBUS Pulse Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vplen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vplen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VPLEN_SPEC;
impl crate::RegisterSpec for VPLEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vplen::R`](R) reader structure"]
impl crate::Readable for VPLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vplen::W`](W) writer structure"]
impl crate::Writable for VPLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VPLEN to value 0"]
impl crate::Resettable for VPLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
