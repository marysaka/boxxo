#[doc = "Register `FADDR` reader"]
pub type R = crate::R<FADDR_SPEC>;
#[doc = "Register `FADDR` writer"]
pub type W = crate::W<FADDR_SPEC>;
#[doc = "Field `USB_FADDR` reader - Function Address"]
pub type USB_FADDR_R = crate::FieldReader;
#[doc = "Field `USB_FADDR` writer - Function Address"]
pub type USB_FADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Function Address"]
    #[inline(always)]
    pub fn usb_faddr(&self) -> USB_FADDR_R {
        USB_FADDR_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Function Address"]
    #[inline(always)]
    #[must_use]
    pub fn usb_faddr(&mut self) -> USB_FADDR_W<FADDR_SPEC, 0> {
        USB_FADDR_W::new(self)
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
#[doc = "USB Device Functional Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FADDR_SPEC;
impl crate::RegisterSpec for FADDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`faddr::R`](R) reader structure"]
impl crate::Readable for FADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`faddr::W`](W) writer structure"]
impl crate::Writable for FADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FADDR to value 0"]
impl crate::Resettable for FADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
