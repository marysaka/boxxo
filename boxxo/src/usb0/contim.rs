#[doc = "Register `CONTIM` reader"]
pub type R = crate::R<CONTIM_SPEC>;
#[doc = "Register `CONTIM` writer"]
pub type W = crate::W<CONTIM_SPEC>;
#[doc = "Field `USB_CONTIM_WTID` reader - Wait ID"]
pub type USB_CONTIM_WTID_R = crate::FieldReader;
#[doc = "Field `USB_CONTIM_WTID` writer - Wait ID"]
pub type USB_CONTIM_WTID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USB_CONTIM_WTCON` reader - Connect Wait"]
pub type USB_CONTIM_WTCON_R = crate::FieldReader;
#[doc = "Field `USB_CONTIM_WTCON` writer - Connect Wait"]
pub type USB_CONTIM_WTCON_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Wait ID"]
    #[inline(always)]
    pub fn usb_contim_wtid(&self) -> USB_CONTIM_WTID_R {
        USB_CONTIM_WTID_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Connect Wait"]
    #[inline(always)]
    pub fn usb_contim_wtcon(&self) -> USB_CONTIM_WTCON_R {
        USB_CONTIM_WTCON_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait ID"]
    #[inline(always)]
    #[must_use]
    pub fn usb_contim_wtid(&mut self) -> USB_CONTIM_WTID_W<CONTIM_SPEC, 0> {
        USB_CONTIM_WTID_W::new(self)
    }
    #[doc = "Bits 4:7 - Connect Wait"]
    #[inline(always)]
    #[must_use]
    pub fn usb_contim_wtcon(&mut self) -> USB_CONTIM_WTCON_W<CONTIM_SPEC, 4> {
        USB_CONTIM_WTCON_W::new(self)
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
#[doc = "USB Connect Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`contim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`contim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTIM_SPEC;
impl crate::RegisterSpec for CONTIM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`contim::R`](R) reader structure"]
impl crate::Readable for CONTIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`contim::W`](W) writer structure"]
impl crate::Writable for CONTIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTIM to value 0"]
impl crate::Resettable for CONTIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
