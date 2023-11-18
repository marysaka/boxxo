#[doc = "Register `RQPKTCOUNT6` reader"]
pub type R = crate::R<RQPKTCOUNT6_SPEC>;
#[doc = "Register `RQPKTCOUNT6` writer"]
pub type W = crate::W<RQPKTCOUNT6_SPEC>;
#[doc = "Field `USB_RQPKTCOUNT6_COUNT` reader - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT6_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `USB_RQPKTCOUNT6_COUNT` writer - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT6_COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount6_count(&self) -> USB_RQPKTCOUNT6_COUNT_R {
        USB_RQPKTCOUNT6_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rqpktcount6_count(&mut self) -> USB_RQPKTCOUNT6_COUNT_W<RQPKTCOUNT6_SPEC, 0> {
        USB_RQPKTCOUNT6_COUNT_W::new(self)
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
#[doc = "USB Request Packet Count in Block Transfer Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQPKTCOUNT6_SPEC;
impl crate::RegisterSpec for RQPKTCOUNT6_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rqpktcount6::R`](R) reader structure"]
impl crate::Readable for RQPKTCOUNT6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rqpktcount6::W`](W) writer structure"]
impl crate::Writable for RQPKTCOUNT6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RQPKTCOUNT6 to value 0"]
impl crate::Resettable for RQPKTCOUNT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
