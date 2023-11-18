#[doc = "Register `VDCISC` reader"]
pub type R = crate::R<VDCISC_SPEC>;
#[doc = "Register `VDCISC` writer"]
pub type W = crate::W<VDCISC_SPEC>;
#[doc = "Field `USB_VDCISC_VD` reader - VBUS Droop Interrupt Status and Clear"]
pub type USB_VDCISC_VD_R = crate::BitReader;
#[doc = "Field `USB_VDCISC_VD` writer - VBUS Droop Interrupt Status and Clear"]
pub type USB_VDCISC_VD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - VBUS Droop Interrupt Status and Clear"]
    #[inline(always)]
    pub fn usb_vdcisc_vd(&self) -> USB_VDCISC_VD_R {
        USB_VDCISC_VD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Droop Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vdcisc_vd(&mut self) -> USB_VDCISC_VD_W<VDCISC_SPEC, 0> {
        USB_VDCISC_VD_W::new(self)
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
#[doc = "USB VBUS Droop Control Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdcisc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdcisc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDCISC_SPEC;
impl crate::RegisterSpec for VDCISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdcisc::R`](R) reader structure"]
impl crate::Readable for VDCISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdcisc::W`](W) writer structure"]
impl crate::Writable for VDCISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDCISC to value 0"]
impl crate::Resettable for VDCISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
