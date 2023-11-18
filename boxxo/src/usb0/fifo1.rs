#[doc = "Register `FIFO1` reader"]
pub type R = crate::R<FIFO1_SPEC>;
#[doc = "Register `FIFO1` writer"]
pub type W = crate::W<FIFO1_SPEC>;
#[doc = "Field `USB_FIFO1_EPDATA` reader - Endpoint Data"]
pub type USB_FIFO1_EPDATA_R = crate::FieldReader<u32>;
#[doc = "Field `USB_FIFO1_EPDATA` writer - Endpoint Data"]
pub type USB_FIFO1_EPDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    pub fn usb_fifo1_epdata(&self) -> USB_FIFO1_EPDATA_R {
        USB_FIFO1_EPDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    #[must_use]
    pub fn usb_fifo1_epdata(&mut self) -> USB_FIFO1_EPDATA_W<FIFO1_SPEC, 0> {
        USB_FIFO1_EPDATA_W::new(self)
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
#[doc = "USB FIFO Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO1_SPEC;
impl crate::RegisterSpec for FIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo1::R`](R) reader structure"]
impl crate::Readable for FIFO1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo1::W`](W) writer structure"]
impl crate::Writable for FIFO1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO1 to value 0"]
impl crate::Resettable for FIFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
