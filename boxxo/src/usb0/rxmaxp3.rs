#[doc = "Register `RXMAXP3` reader"]
pub type R = crate::R<RXMAXP3_SPEC>;
#[doc = "Register `RXMAXP3` writer"]
pub type W = crate::W<RXMAXP3_SPEC>;
#[doc = "Field `USB_RXMAXP3_MAXLOAD` reader - Maximum Payload"]
pub type USB_RXMAXP3_MAXLOAD_R = crate::FieldReader<u16>;
#[doc = "Field `USB_RXMAXP3_MAXLOAD` writer - Maximum Payload"]
pub type USB_RXMAXP3_MAXLOAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    pub fn usb_rxmaxp3_maxload(&self) -> USB_RXMAXP3_MAXLOAD_R {
        USB_RXMAXP3_MAXLOAD_R::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxmaxp3_maxload(&mut self) -> USB_RXMAXP3_MAXLOAD_W<RXMAXP3_SPEC, 0> {
        USB_RXMAXP3_MAXLOAD_W::new(self)
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
#[doc = "USB Maximum Receive Data Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaxp3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaxp3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMAXP3_SPEC;
impl crate::RegisterSpec for RXMAXP3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rxmaxp3::R`](R) reader structure"]
impl crate::Readable for RXMAXP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxmaxp3::W`](W) writer structure"]
impl crate::Writable for RXMAXP3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXMAXP3 to value 0"]
impl crate::Resettable for RXMAXP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
