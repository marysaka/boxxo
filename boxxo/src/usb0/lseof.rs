#[doc = "Register `LSEOF` reader"]
pub type R = crate::R<LSEOF_SPEC>;
#[doc = "Register `LSEOF` writer"]
pub type W = crate::W<LSEOF_SPEC>;
#[doc = "Field `USB_LSEOF_LSEOFG` reader - Low-Speed End-of-Frame Gap"]
pub type USB_LSEOF_LSEOFG_R = crate::FieldReader;
#[doc = "Field `USB_LSEOF_LSEOFG` writer - Low-Speed End-of-Frame Gap"]
pub type USB_LSEOF_LSEOFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Low-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn usb_lseof_lseofg(&self) -> USB_LSEOF_LSEOFG_R {
        USB_LSEOF_LSEOFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low-Speed End-of-Frame Gap"]
    #[inline(always)]
    #[must_use]
    pub fn usb_lseof_lseofg(&mut self) -> USB_LSEOF_LSEOFG_W<LSEOF_SPEC, 0> {
        USB_LSEOF_LSEOFG_W::new(self)
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
#[doc = "USB Low-Speed Last Transaction to End of Frame Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lseof::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lseof::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSEOF_SPEC;
impl crate::RegisterSpec for LSEOF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lseof::R`](R) reader structure"]
impl crate::Readable for LSEOF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lseof::W`](W) writer structure"]
impl crate::Writable for LSEOF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSEOF to value 0"]
impl crate::Resettable for LSEOF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
