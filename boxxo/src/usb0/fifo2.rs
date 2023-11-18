#[doc = "Register `FIFO2` reader"]
pub type R = crate::R<FIFO2_SPEC>;
#[doc = "Register `FIFO2` writer"]
pub type W = crate::W<FIFO2_SPEC>;
#[doc = "Field `USB_FIFO2_EPDATA` reader - Endpoint Data"]
pub type USB_FIFO2_EPDATA_R = crate::FieldReader<u32>;
#[doc = "Field `USB_FIFO2_EPDATA` writer - Endpoint Data"]
pub type USB_FIFO2_EPDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    pub fn usb_fifo2_epdata(&self) -> USB_FIFO2_EPDATA_R {
        USB_FIFO2_EPDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    #[must_use]
    pub fn usb_fifo2_epdata(&mut self) -> USB_FIFO2_EPDATA_W<FIFO2_SPEC, 0> {
        USB_FIFO2_EPDATA_W::new(self)
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
#[doc = "USB FIFO Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO2_SPEC;
impl crate::RegisterSpec for FIFO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo2::R`](R) reader structure"]
impl crate::Readable for FIFO2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo2::W`](W) writer structure"]
impl crate::Writable for FIFO2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO2 to value 0"]
impl crate::Resettable for FIFO2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
