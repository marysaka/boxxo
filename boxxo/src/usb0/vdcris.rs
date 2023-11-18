#[doc = "Register `VDCRIS` reader"]
pub type R = crate::R<VDCRIS_SPEC>;
#[doc = "Register `VDCRIS` writer"]
pub type W = crate::W<VDCRIS_SPEC>;
#[doc = "Field `USB_VDCRIS_VD` reader - VBUS Droop Raw Interrupt Status"]
pub type USB_VDCRIS_VD_R = crate::BitReader;
#[doc = "Field `USB_VDCRIS_VD` writer - VBUS Droop Raw Interrupt Status"]
pub type USB_VDCRIS_VD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - VBUS Droop Raw Interrupt Status"]
    #[inline(always)]
    pub fn usb_vdcris_vd(&self) -> USB_VDCRIS_VD_R {
        USB_VDCRIS_VD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Droop Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vdcris_vd(&mut self) -> USB_VDCRIS_VD_W<VDCRIS_SPEC, 0> {
        USB_VDCRIS_VD_W::new(self)
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
#[doc = "USB VBUS Droop Control Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdcris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdcris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDCRIS_SPEC;
impl crate::RegisterSpec for VDCRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdcris::R`](R) reader structure"]
impl crate::Readable for VDCRIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdcris::W`](W) writer structure"]
impl crate::Writable for VDCRIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDCRIS to value 0"]
impl crate::Resettable for VDCRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
