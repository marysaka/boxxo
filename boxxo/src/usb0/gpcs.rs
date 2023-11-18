#[doc = "Register `GPCS` reader"]
pub type R = crate::R<GPCS_SPEC>;
#[doc = "Register `GPCS` writer"]
pub type W = crate::W<GPCS_SPEC>;
#[doc = "Field `USB_GPCS_DEVMOD` reader - Device Mode"]
pub type USB_GPCS_DEVMOD_R = crate::BitReader;
#[doc = "Field `USB_GPCS_DEVMOD` writer - Device Mode"]
pub type USB_GPCS_DEVMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_GPCS_DEVMODOTG` reader - Enable Device Mode"]
pub type USB_GPCS_DEVMODOTG_R = crate::BitReader;
#[doc = "Field `USB_GPCS_DEVMODOTG` writer - Enable Device Mode"]
pub type USB_GPCS_DEVMODOTG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Device Mode"]
    #[inline(always)]
    pub fn usb_gpcs_devmod(&self) -> USB_GPCS_DEVMOD_R {
        USB_GPCS_DEVMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Device Mode"]
    #[inline(always)]
    pub fn usb_gpcs_devmodotg(&self) -> USB_GPCS_DEVMODOTG_R {
        USB_GPCS_DEVMODOTG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_gpcs_devmod(&mut self) -> USB_GPCS_DEVMOD_W<GPCS_SPEC, 0> {
        USB_GPCS_DEVMOD_W::new(self)
    }
    #[doc = "Bit 1 - Enable Device Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_gpcs_devmodotg(&mut self) -> USB_GPCS_DEVMODOTG_W<GPCS_SPEC, 1> {
        USB_GPCS_DEVMODOTG_W::new(self)
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
#[doc = "USB General-Purpose Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPCS_SPEC;
impl crate::RegisterSpec for GPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpcs::R`](R) reader structure"]
impl crate::Readable for GPCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpcs::W`](W) writer structure"]
impl crate::Writable for GPCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPCS to value 0"]
impl crate::Resettable for GPCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
