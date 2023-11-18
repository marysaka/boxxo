#[doc = "Register `RQPKTCOUNT2` reader"]
pub type R = crate::R<RQPKTCOUNT2_SPEC>;
#[doc = "Register `RQPKTCOUNT2` writer"]
pub type W = crate::W<RQPKTCOUNT2_SPEC>;
#[doc = "Field `USB_RQPKTCOUNT2` reader - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT2_R = crate::FieldReader<u16>;
#[doc = "Field `USB_RQPKTCOUNT2` writer - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount2(&self) -> USB_RQPKTCOUNT2_R {
        USB_RQPKTCOUNT2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rqpktcount2(&mut self) -> USB_RQPKTCOUNT2_W<RQPKTCOUNT2_SPEC, 0> {
        USB_RQPKTCOUNT2_W::new(self)
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
#[doc = "USB Request Packet Count in Block Transfer Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQPKTCOUNT2_SPEC;
impl crate::RegisterSpec for RQPKTCOUNT2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rqpktcount2::R`](R) reader structure"]
impl crate::Readable for RQPKTCOUNT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rqpktcount2::W`](W) writer structure"]
impl crate::Writable for RQPKTCOUNT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RQPKTCOUNT2 to value 0"]
impl crate::Resettable for RQPKTCOUNT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
