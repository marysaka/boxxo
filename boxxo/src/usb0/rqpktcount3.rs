#[doc = "Register `RQPKTCOUNT3` reader"]
pub type R = crate::R<RQPKTCOUNT3_SPEC>;
#[doc = "Register `RQPKTCOUNT3` writer"]
pub type W = crate::W<RQPKTCOUNT3_SPEC>;
#[doc = "Field `USB_RQPKTCOUNT3` reader - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT3_R = crate::FieldReader<u16>;
#[doc = "Field `USB_RQPKTCOUNT3` writer - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount3(&self) -> USB_RQPKTCOUNT3_R {
        USB_RQPKTCOUNT3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rqpktcount3(&mut self) -> USB_RQPKTCOUNT3_W<RQPKTCOUNT3_SPEC, 0> {
        USB_RQPKTCOUNT3_W::new(self)
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
#[doc = "USB Request Packet Count in Block Transfer Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQPKTCOUNT3_SPEC;
impl crate::RegisterSpec for RQPKTCOUNT3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rqpktcount3::R`](R) reader structure"]
impl crate::Readable for RQPKTCOUNT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rqpktcount3::W`](W) writer structure"]
impl crate::Writable for RQPKTCOUNT3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RQPKTCOUNT3 to value 0"]
impl crate::Resettable for RQPKTCOUNT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
