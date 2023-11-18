#[doc = "Register `FRAME` reader"]
pub type R = crate::R<FRAME_SPEC>;
#[doc = "Register `FRAME` writer"]
pub type W = crate::W<FRAME_SPEC>;
#[doc = "Field `USB_FRAME` reader - Frame Number"]
pub type USB_FRAME_R = crate::FieldReader<u16>;
#[doc = "Field `USB_FRAME` writer - Frame Number"]
pub type USB_FRAME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - Frame Number"]
    #[inline(always)]
    pub fn usb_frame(&self) -> USB_FRAME_R {
        USB_FRAME_R::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn usb_frame(&mut self) -> USB_FRAME_W<FRAME_SPEC, 0> {
        USB_FRAME_W::new(self)
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
#[doc = "USB Frame Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAME_SPEC;
impl crate::RegisterSpec for FRAME_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frame::R`](R) reader structure"]
impl crate::Readable for FRAME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frame::W`](W) writer structure"]
impl crate::Writable for FRAME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAME to value 0"]
impl crate::Resettable for FRAME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
