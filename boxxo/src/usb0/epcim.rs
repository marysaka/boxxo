#[doc = "Register `EPCIM` reader"]
pub type R = crate::R<EPCIM_SPEC>;
#[doc = "Register `EPCIM` writer"]
pub type W = crate::W<EPCIM_SPEC>;
#[doc = "Field `USB_EPCIM_PF` reader - USB Power Fault Interrupt Mask"]
pub type USB_EPCIM_PF_R = crate::BitReader;
#[doc = "Field `USB_EPCIM_PF` writer - USB Power Fault Interrupt Mask"]
pub type USB_EPCIM_PF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB Power Fault Interrupt Mask"]
    #[inline(always)]
    pub fn usb_epcim_pf(&self) -> USB_EPCIM_PF_R {
        USB_EPCIM_PF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Power Fault Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usb_epcim_pf(&mut self) -> USB_EPCIM_PF_W<EPCIM_SPEC, 0> {
        USB_EPCIM_PF_W::new(self)
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
#[doc = "USB External Power Control Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epcim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epcim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPCIM_SPEC;
impl crate::RegisterSpec for EPCIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epcim::R`](R) reader structure"]
impl crate::Readable for EPCIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epcim::W`](W) writer structure"]
impl crate::Writable for EPCIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPCIM to value 0"]
impl crate::Resettable for EPCIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
