#[doc = "Register `COUNT0` reader"]
pub type R = crate::R<COUNT0_SPEC>;
#[doc = "Register `COUNT0` writer"]
pub type W = crate::W<COUNT0_SPEC>;
#[doc = "Field `USB_COUNT0_COUNT` reader - FIFO Count"]
pub type USB_COUNT0_COUNT_R = crate::FieldReader;
#[doc = "Field `USB_COUNT0_COUNT` writer - FIFO Count"]
pub type USB_COUNT0_COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - FIFO Count"]
    #[inline(always)]
    pub fn usb_count0_count(&self) -> USB_COUNT0_COUNT_R {
        USB_COUNT0_COUNT_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - FIFO Count"]
    #[inline(always)]
    #[must_use]
    pub fn usb_count0_count(&mut self) -> USB_COUNT0_COUNT_W<COUNT0_SPEC, 0> {
        USB_COUNT0_COUNT_W::new(self)
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
#[doc = "USB Receive Byte Count Endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNT0_SPEC;
impl crate::RegisterSpec for COUNT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`count0::R`](R) reader structure"]
impl crate::Readable for COUNT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`count0::W`](W) writer structure"]
impl crate::Writable for COUNT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNT0 to value 0"]
impl crate::Resettable for COUNT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
