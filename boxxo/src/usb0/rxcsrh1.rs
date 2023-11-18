#[doc = "Register `RXCSRH1` reader"]
pub type R = crate::R<RXCSRH1_SPEC>;
#[doc = "Register `RXCSRH1` writer"]
pub type W = crate::W<RXCSRH1_SPEC>;
#[doc = "Field `USB_RXCSRH1_DT` reader - Data Toggle"]
pub type USB_RXCSRH1_DT_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH1_DT` writer - Data Toggle"]
pub type USB_RXCSRH1_DT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH1_DTWE` reader - Data Toggle Write Enable"]
pub type USB_RXCSRH1_DTWE_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH1_DTWE` writer - Data Toggle Write Enable"]
pub type USB_RXCSRH1_DTWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH1_DMAMOD` reader - DMA Request Mode"]
pub type USB_RXCSRH1_DMAMOD_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH1_DMAMOD` writer - DMA Request Mode"]
pub type USB_RXCSRH1_DMAMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH1_PIDERR` reader - PID Error"]
pub type USB_RXCSRH1_PIDERR_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH1_PIDERR` writer - PID Error"]
pub type USB_RXCSRH1_PIDERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH1_DMAEN` reader - DMA Request Enable"]
pub type USB_RXCSRH1_DMAEN_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH1_DMAEN` writer - DMA Request Enable"]
pub type USB_RXCSRH1_DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH1_AUTORQ` reader - Auto Request"]
pub type USB_RXCSRH1_AUTORQ_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH1_AUTORQ` writer - Auto Request"]
pub type USB_RXCSRH1_AUTORQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH1_AUTOCL` reader - Auto Clear"]
pub type USB_RXCSRH1_AUTOCL_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH1_AUTOCL` writer - Auto Clear"]
pub type USB_RXCSRH1_AUTOCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh1_dt(&self) -> USB_RXCSRH1_DT_R {
        USB_RXCSRH1_DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh1_dtwe(&self) -> USB_RXCSRH1_DTWE_R {
        USB_RXCSRH1_DTWE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh1_dmamod(&self) -> USB_RXCSRH1_DMAMOD_R {
        USB_RXCSRH1_DMAMOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh1_piderr(&self) -> USB_RXCSRH1_PIDERR_R {
        USB_RXCSRH1_PIDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh1_dmaen(&self) -> USB_RXCSRH1_DMAEN_R {
        USB_RXCSRH1_DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh1_autorq(&self) -> USB_RXCSRH1_AUTORQ_R {
        USB_RXCSRH1_AUTORQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh1_autocl(&self) -> USB_RXCSRH1_AUTOCL_R {
        USB_RXCSRH1_AUTOCL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh1_dt(&mut self) -> USB_RXCSRH1_DT_W<RXCSRH1_SPEC, 1> {
        USB_RXCSRH1_DT_W::new(self)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh1_dtwe(&mut self) -> USB_RXCSRH1_DTWE_W<RXCSRH1_SPEC, 2> {
        USB_RXCSRH1_DTWE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh1_dmamod(&mut self) -> USB_RXCSRH1_DMAMOD_W<RXCSRH1_SPEC, 3> {
        USB_RXCSRH1_DMAMOD_W::new(self)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh1_piderr(&mut self) -> USB_RXCSRH1_PIDERR_W<RXCSRH1_SPEC, 4> {
        USB_RXCSRH1_PIDERR_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh1_dmaen(&mut self) -> USB_RXCSRH1_DMAEN_W<RXCSRH1_SPEC, 5> {
        USB_RXCSRH1_DMAEN_W::new(self)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh1_autorq(&mut self) -> USB_RXCSRH1_AUTORQ_W<RXCSRH1_SPEC, 6> {
        USB_RXCSRH1_AUTORQ_W::new(self)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh1_autocl(&mut self) -> USB_RXCSRH1_AUTOCL_W<RXCSRH1_SPEC, 7> {
        USB_RXCSRH1_AUTOCL_W::new(self)
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
#[doc = "USB Receive Control and Status Endpoint 1 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCSRH1_SPEC;
impl crate::RegisterSpec for RXCSRH1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcsrh1::R`](R) reader structure"]
impl crate::Readable for RXCSRH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxcsrh1::W`](W) writer structure"]
impl crate::Writable for RXCSRH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCSRH1 to value 0"]
impl crate::Resettable for RXCSRH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
