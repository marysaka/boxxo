#[doc = "Register `DMASEL` reader"]
pub type R = crate::R<DMASEL_SPEC>;
#[doc = "Register `DMASEL` writer"]
pub type W = crate::W<DMASEL_SPEC>;
#[doc = "Field `USB_DMASEL_DMAARX` reader - DMA A RX Select"]
pub type USB_DMASEL_DMAARX_R = crate::FieldReader;
#[doc = "Field `USB_DMASEL_DMAARX` writer - DMA A RX Select"]
pub type USB_DMASEL_DMAARX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USB_DMASEL_DMAATX` reader - DMA A TX Select"]
pub type USB_DMASEL_DMAATX_R = crate::FieldReader;
#[doc = "Field `USB_DMASEL_DMAATX` writer - DMA A TX Select"]
pub type USB_DMASEL_DMAATX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USB_DMASEL_DMABRX` reader - DMA B RX Select"]
pub type USB_DMASEL_DMABRX_R = crate::FieldReader;
#[doc = "Field `USB_DMASEL_DMABRX` writer - DMA B RX Select"]
pub type USB_DMASEL_DMABRX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USB_DMASEL_DMABTX` reader - DMA B TX Select"]
pub type USB_DMASEL_DMABTX_R = crate::FieldReader;
#[doc = "Field `USB_DMASEL_DMABTX` writer - DMA B TX Select"]
pub type USB_DMASEL_DMABTX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USB_DMASEL_DMACRX` reader - DMA C RX Select"]
pub type USB_DMASEL_DMACRX_R = crate::FieldReader;
#[doc = "Field `USB_DMASEL_DMACRX` writer - DMA C RX Select"]
pub type USB_DMASEL_DMACRX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USB_DMASEL_DMACTX` reader - DMA C TX Select"]
pub type USB_DMASEL_DMACTX_R = crate::FieldReader;
#[doc = "Field `USB_DMASEL_DMACTX` writer - DMA C TX Select"]
pub type USB_DMASEL_DMACTX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - DMA A RX Select"]
    #[inline(always)]
    pub fn usb_dmasel_dmaarx(&self) -> USB_DMASEL_DMAARX_R {
        USB_DMASEL_DMAARX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA A TX Select"]
    #[inline(always)]
    pub fn usb_dmasel_dmaatx(&self) -> USB_DMASEL_DMAATX_R {
        USB_DMASEL_DMAATX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA B RX Select"]
    #[inline(always)]
    pub fn usb_dmasel_dmabrx(&self) -> USB_DMASEL_DMABRX_R {
        USB_DMASEL_DMABRX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA B TX Select"]
    #[inline(always)]
    pub fn usb_dmasel_dmabtx(&self) -> USB_DMASEL_DMABTX_R {
        USB_DMASEL_DMABTX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA C RX Select"]
    #[inline(always)]
    pub fn usb_dmasel_dmacrx(&self) -> USB_DMASEL_DMACRX_R {
        USB_DMASEL_DMACRX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA C TX Select"]
    #[inline(always)]
    pub fn usb_dmasel_dmactx(&self) -> USB_DMASEL_DMACTX_R {
        USB_DMASEL_DMACTX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA A RX Select"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dmasel_dmaarx(&mut self) -> USB_DMASEL_DMAARX_W<DMASEL_SPEC, 0> {
        USB_DMASEL_DMAARX_W::new(self)
    }
    #[doc = "Bits 4:7 - DMA A TX Select"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dmasel_dmaatx(&mut self) -> USB_DMASEL_DMAATX_W<DMASEL_SPEC, 4> {
        USB_DMASEL_DMAATX_W::new(self)
    }
    #[doc = "Bits 8:11 - DMA B RX Select"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dmasel_dmabrx(&mut self) -> USB_DMASEL_DMABRX_W<DMASEL_SPEC, 8> {
        USB_DMASEL_DMABRX_W::new(self)
    }
    #[doc = "Bits 12:15 - DMA B TX Select"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dmasel_dmabtx(&mut self) -> USB_DMASEL_DMABTX_W<DMASEL_SPEC, 12> {
        USB_DMASEL_DMABTX_W::new(self)
    }
    #[doc = "Bits 16:19 - DMA C RX Select"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dmasel_dmacrx(&mut self) -> USB_DMASEL_DMACRX_W<DMASEL_SPEC, 16> {
        USB_DMASEL_DMACRX_W::new(self)
    }
    #[doc = "Bits 20:23 - DMA C TX Select"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dmasel_dmactx(&mut self) -> USB_DMASEL_DMACTX_W<DMASEL_SPEC, 20> {
        USB_DMASEL_DMACTX_W::new(self)
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
#[doc = "USB DMA Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASEL_SPEC;
impl crate::RegisterSpec for DMASEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasel::R`](R) reader structure"]
impl crate::Readable for DMASEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmasel::W`](W) writer structure"]
impl crate::Writable for DMASEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMASEL to value 0"]
impl crate::Resettable for DMASEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
