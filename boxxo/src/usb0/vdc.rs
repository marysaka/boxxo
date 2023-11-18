#[doc = "Register `VDC` reader"]
pub type R = crate::R<VDC_SPEC>;
#[doc = "Register `VDC` writer"]
pub type W = crate::W<VDC_SPEC>;
#[doc = "Field `USB_VDC_VBDEN` reader - VBUS Droop Enable"]
pub type USB_VDC_VBDEN_R = crate::BitReader;
#[doc = "Field `USB_VDC_VBDEN` writer - VBUS Droop Enable"]
pub type USB_VDC_VBDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - VBUS Droop Enable"]
    #[inline(always)]
    pub fn usb_vdc_vbden(&self) -> USB_VDC_VBDEN_R {
        USB_VDC_VBDEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Droop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vdc_vbden(&mut self) -> USB_VDC_VBDEN_W<VDC_SPEC, 0> {
        USB_VDC_VBDEN_W::new(self)
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
#[doc = "USB VBUS Droop Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDC_SPEC;
impl crate::RegisterSpec for VDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdc::R`](R) reader structure"]
impl crate::Readable for VDC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdc::W`](W) writer structure"]
impl crate::Writable for VDC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDC to value 0"]
impl crate::Resettable for VDC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
