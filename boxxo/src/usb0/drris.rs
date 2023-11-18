#[doc = "Register `DRRIS` reader"]
pub type R = crate::R<DRRIS_SPEC>;
#[doc = "Register `DRRIS` writer"]
pub type W = crate::W<DRRIS_SPEC>;
#[doc = "Field `USB_DRRIS_RESUME` reader - RESUME Interrupt Status"]
pub type USB_DRRIS_RESUME_R = crate::BitReader;
#[doc = "Field `USB_DRRIS_RESUME` writer - RESUME Interrupt Status"]
pub type USB_DRRIS_RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RESUME Interrupt Status"]
    #[inline(always)]
    pub fn usb_drris_resume(&self) -> USB_DRRIS_RESUME_R {
        USB_DRRIS_RESUME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESUME Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn usb_drris_resume(&mut self) -> USB_DRRIS_RESUME_W<DRRIS_SPEC, 0> {
        USB_DRRIS_RESUME_W::new(self)
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
#[doc = "USB Device RESUME Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRRIS_SPEC;
impl crate::RegisterSpec for DRRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drris::R`](R) reader structure"]
impl crate::Readable for DRRIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`drris::W`](W) writer structure"]
impl crate::Writable for DRRIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRRIS to value 0"]
impl crate::Resettable for DRRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
