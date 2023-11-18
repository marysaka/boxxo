#[doc = "Register `DRISC` writer"]
pub type W = crate::W<DRISC_SPEC>;
#[doc = "Field `USB_DRISC_RESUME` writer - RESUME Interrupt Status and Clear"]
pub type USB_DRISC_RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - RESUME Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb_drisc_resume(&mut self) -> USB_DRISC_RESUME_W<DRISC_SPEC, 0> {
        USB_DRISC_RESUME_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Device RESUME Interrupt Status and Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drisc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRISC_SPEC;
impl crate::RegisterSpec for DRISC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`drisc::W`](W) writer structure"]
impl crate::Writable for DRISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRISC to value 0"]
impl crate::Resettable for DRISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
