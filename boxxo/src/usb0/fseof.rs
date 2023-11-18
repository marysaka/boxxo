#[doc = "Register `FSEOF` reader"]
pub type R = crate::R<FSEOF_SPEC>;
#[doc = "Register `FSEOF` writer"]
pub type W = crate::W<FSEOF_SPEC>;
#[doc = "Field `USB_FSEOF_FSEOFG` reader - Full-Speed End-of-Frame Gap"]
pub type USB_FSEOF_FSEOFG_R = crate::FieldReader;
#[doc = "Field `USB_FSEOF_FSEOFG` writer - Full-Speed End-of-Frame Gap"]
pub type USB_FSEOF_FSEOFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Full-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn usb_fseof_fseofg(&self) -> USB_FSEOF_FSEOFG_R {
        USB_FSEOF_FSEOFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Full-Speed End-of-Frame Gap"]
    #[inline(always)]
    #[must_use]
    pub fn usb_fseof_fseofg(&mut self) -> USB_FSEOF_FSEOFG_W<FSEOF_SPEC, 0> {
        USB_FSEOF_FSEOFG_W::new(self)
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
#[doc = "USB Full-Speed Last Transaction to End of Frame Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fseof::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fseof::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSEOF_SPEC;
impl crate::RegisterSpec for FSEOF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fseof::R`](R) reader structure"]
impl crate::Readable for FSEOF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fseof::W`](W) writer structure"]
impl crate::Writable for FSEOF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSEOF to value 0"]
impl crate::Resettable for FSEOF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
