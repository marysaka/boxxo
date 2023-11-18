#[doc = "Register `RXCSRH7` reader"]
pub type R = crate::R<RXCSRH7_SPEC>;
#[doc = "Register `RXCSRH7` writer"]
pub type W = crate::W<RXCSRH7_SPEC>;
#[doc = "Field `USB_RXCSRH7_DT` reader - Data Toggle"]
pub type USB_RXCSRH7_DT_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH7_DT` writer - Data Toggle"]
pub type USB_RXCSRH7_DT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH7_DTWE` reader - Data Toggle Write Enable"]
pub type USB_RXCSRH7_DTWE_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH7_DTWE` writer - Data Toggle Write Enable"]
pub type USB_RXCSRH7_DTWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH7_DMAMOD` reader - DMA Request Mode"]
pub type USB_RXCSRH7_DMAMOD_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH7_DMAMOD` writer - DMA Request Mode"]
pub type USB_RXCSRH7_DMAMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH7_PIDERR` reader - PID Error"]
pub type USB_RXCSRH7_PIDERR_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH7_PIDERR` writer - PID Error"]
pub type USB_RXCSRH7_PIDERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH7_DMAEN` reader - DMA Request Enable"]
pub type USB_RXCSRH7_DMAEN_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH7_DMAEN` writer - DMA Request Enable"]
pub type USB_RXCSRH7_DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH7_AUTORQ` reader - Auto Request"]
pub type USB_RXCSRH7_AUTORQ_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH7_AUTORQ` writer - Auto Request"]
pub type USB_RXCSRH7_AUTORQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH7_AUTOCL` reader - Auto Clear"]
pub type USB_RXCSRH7_AUTOCL_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH7_AUTOCL` writer - Auto Clear"]
pub type USB_RXCSRH7_AUTOCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dt(&self) -> USB_RXCSRH7_DT_R {
        USB_RXCSRH7_DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dtwe(&self) -> USB_RXCSRH7_DTWE_R {
        USB_RXCSRH7_DTWE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dmamod(&self) -> USB_RXCSRH7_DMAMOD_R {
        USB_RXCSRH7_DMAMOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh7_piderr(&self) -> USB_RXCSRH7_PIDERR_R {
        USB_RXCSRH7_PIDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dmaen(&self) -> USB_RXCSRH7_DMAEN_R {
        USB_RXCSRH7_DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh7_autorq(&self) -> USB_RXCSRH7_AUTORQ_R {
        USB_RXCSRH7_AUTORQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh7_autocl(&self) -> USB_RXCSRH7_AUTOCL_R {
        USB_RXCSRH7_AUTOCL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh7_dt(&mut self) -> USB_RXCSRH7_DT_W<RXCSRH7_SPEC, 1> {
        USB_RXCSRH7_DT_W::new(self)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh7_dtwe(&mut self) -> USB_RXCSRH7_DTWE_W<RXCSRH7_SPEC, 2> {
        USB_RXCSRH7_DTWE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh7_dmamod(&mut self) -> USB_RXCSRH7_DMAMOD_W<RXCSRH7_SPEC, 3> {
        USB_RXCSRH7_DMAMOD_W::new(self)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh7_piderr(&mut self) -> USB_RXCSRH7_PIDERR_W<RXCSRH7_SPEC, 4> {
        USB_RXCSRH7_PIDERR_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh7_dmaen(&mut self) -> USB_RXCSRH7_DMAEN_W<RXCSRH7_SPEC, 5> {
        USB_RXCSRH7_DMAEN_W::new(self)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh7_autorq(&mut self) -> USB_RXCSRH7_AUTORQ_W<RXCSRH7_SPEC, 6> {
        USB_RXCSRH7_AUTORQ_W::new(self)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh7_autocl(&mut self) -> USB_RXCSRH7_AUTOCL_W<RXCSRH7_SPEC, 7> {
        USB_RXCSRH7_AUTOCL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Receive Control and Status Endpoint 7 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCSRH7_SPEC;
impl crate::RegisterSpec for RXCSRH7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcsrh7::R`](R) reader structure"]
impl crate::Readable for RXCSRH7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxcsrh7::W`](W) writer structure"]
impl crate::Writable for RXCSRH7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCSRH7 to value 0"]
impl crate::Resettable for RXCSRH7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
