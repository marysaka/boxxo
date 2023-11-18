#[doc = "Register `NAKLMT` reader"]
pub type R = crate::R<NAKLMT_SPEC>;
#[doc = "Register `NAKLMT` writer"]
pub type W = crate::W<NAKLMT_SPEC>;
#[doc = "Field `USB_NAKLMT_NAKLMT` reader - EP0 NAK Limit"]
pub type USB_NAKLMT_NAKLMT_R = crate::FieldReader;
#[doc = "Field `USB_NAKLMT_NAKLMT` writer - EP0 NAK Limit"]
pub type USB_NAKLMT_NAKLMT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - EP0 NAK Limit"]
    #[inline(always)]
    pub fn usb_naklmt_naklmt(&self) -> USB_NAKLMT_NAKLMT_R {
        USB_NAKLMT_NAKLMT_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - EP0 NAK Limit"]
    #[inline(always)]
    #[must_use]
    pub fn usb_naklmt_naklmt(&mut self) -> USB_NAKLMT_NAKLMT_W<NAKLMT_SPEC, 0> {
        USB_NAKLMT_NAKLMT_W::new(self)
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
#[doc = "USB NAK Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`naklmt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`naklmt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAKLMT_SPEC;
impl crate::RegisterSpec for NAKLMT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`naklmt::R`](R) reader structure"]
impl crate::Readable for NAKLMT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`naklmt::W`](W) writer structure"]
impl crate::Writable for NAKLMT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NAKLMT to value 0"]
impl crate::Resettable for NAKLMT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
