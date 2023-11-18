#[doc = "Register `TXMAXP4` reader"]
pub type R = crate::R<TXMAXP4_SPEC>;
#[doc = "Register `TXMAXP4` writer"]
pub type W = crate::W<TXMAXP4_SPEC>;
#[doc = "Field `USB_TXMAXP4_MAXLOAD` reader - Maximum Payload"]
pub type USB_TXMAXP4_MAXLOAD_R = crate::FieldReader<u16>;
#[doc = "Field `USB_TXMAXP4_MAXLOAD` writer - Maximum Payload"]
pub type USB_TXMAXP4_MAXLOAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    pub fn usb_txmaxp4_maxload(&self) -> USB_TXMAXP4_MAXLOAD_R {
        USB_TXMAXP4_MAXLOAD_R::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txmaxp4_maxload(&mut self) -> USB_TXMAXP4_MAXLOAD_W<TXMAXP4_SPEC, 0> {
        USB_TXMAXP4_MAXLOAD_W::new(self)
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
#[doc = "USB Maximum Transmit Data Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmaxp4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmaxp4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMAXP4_SPEC;
impl crate::RegisterSpec for TXMAXP4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`txmaxp4::R`](R) reader structure"]
impl crate::Readable for TXMAXP4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txmaxp4::W`](W) writer structure"]
impl crate::Writable for TXMAXP4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXMAXP4 to value 0"]
impl crate::Resettable for TXMAXP4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
