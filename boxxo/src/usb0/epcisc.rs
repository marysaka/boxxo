#[doc = "Register `EPCISC` reader"]
pub type R = crate::R<EPCISC_SPEC>;
#[doc = "Register `EPCISC` writer"]
pub type W = crate::W<EPCISC_SPEC>;
#[doc = "Field `USB_EPCISC_PF` reader - USB Power Fault Interrupt Status and Clear"]
pub type USB_EPCISC_PF_R = crate::BitReader;
#[doc = "Field `USB_EPCISC_PF` writer - USB Power Fault Interrupt Status and Clear"]
pub type USB_EPCISC_PF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB Power Fault Interrupt Status and Clear"]
    #[inline(always)]
    pub fn usb_epcisc_pf(&self) -> USB_EPCISC_PF_R {
        USB_EPCISC_PF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Power Fault Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb_epcisc_pf(&mut self) -> USB_EPCISC_PF_W<EPCISC_SPEC, 0> {
        USB_EPCISC_PF_W::new(self)
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
#[doc = "USB External Power Control Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epcisc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epcisc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPCISC_SPEC;
impl crate::RegisterSpec for EPCISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epcisc::R`](R) reader structure"]
impl crate::Readable for EPCISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epcisc::W`](W) writer structure"]
impl crate::Writable for EPCISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPCISC to value 0"]
impl crate::Resettable for EPCISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
