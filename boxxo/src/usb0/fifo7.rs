#[doc = "Register `FIFO7` reader"]
pub type R = crate::R<FIFO7_SPEC>;
#[doc = "Register `FIFO7` writer"]
pub type W = crate::W<FIFO7_SPEC>;
#[doc = "Field `USB_FIFO7_EPDATA` reader - Endpoint Data"]
pub type USB_FIFO7_EPDATA_R = crate::FieldReader<u32>;
#[doc = "Field `USB_FIFO7_EPDATA` writer - Endpoint Data"]
pub type USB_FIFO7_EPDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    pub fn usb_fifo7_epdata(&self) -> USB_FIFO7_EPDATA_R {
        USB_FIFO7_EPDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    #[must_use]
    pub fn usb_fifo7_epdata(&mut self) -> USB_FIFO7_EPDATA_W<FIFO7_SPEC, 0> {
        USB_FIFO7_EPDATA_W::new(self)
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
#[doc = "USB FIFO Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO7_SPEC;
impl crate::RegisterSpec for FIFO7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo7::R`](R) reader structure"]
impl crate::Readable for FIFO7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo7::W`](W) writer structure"]
impl crate::Writable for FIFO7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO7 to value 0"]
impl crate::Resettable for FIFO7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
