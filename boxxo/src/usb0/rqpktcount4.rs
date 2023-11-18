#[doc = "Register `RQPKTCOUNT4` reader"]
pub type R = crate::R<RQPKTCOUNT4_SPEC>;
#[doc = "Register `RQPKTCOUNT4` writer"]
pub type W = crate::W<RQPKTCOUNT4_SPEC>;
#[doc = "Field `USB_RQPKTCOUNT4_COUNT` reader - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT4_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `USB_RQPKTCOUNT4_COUNT` writer - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT4_COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount4_count(&self) -> USB_RQPKTCOUNT4_COUNT_R {
        USB_RQPKTCOUNT4_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rqpktcount4_count(&mut self) -> USB_RQPKTCOUNT4_COUNT_W<RQPKTCOUNT4_SPEC, 0> {
        USB_RQPKTCOUNT4_COUNT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQPKTCOUNT4_SPEC;
impl crate::RegisterSpec for RQPKTCOUNT4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rqpktcount4::R`](R) reader structure"]
impl crate::Readable for RQPKTCOUNT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rqpktcount4::W`](W) writer structure"]
impl crate::Writable for RQPKTCOUNT4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RQPKTCOUNT4 to value 0"]
impl crate::Resettable for RQPKTCOUNT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
