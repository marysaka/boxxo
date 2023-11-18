#[doc = "Register `RXCSRH5` reader"]
pub type R = crate::R<RXCSRH5_SPEC>;
#[doc = "Register `RXCSRH5` writer"]
pub type W = crate::W<RXCSRH5_SPEC>;
#[doc = "Field `USB_RXCSRH5_DT` reader - Data Toggle"]
pub type USB_RXCSRH5_DT_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH5_DT` writer - Data Toggle"]
pub type USB_RXCSRH5_DT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH5_DTWE` reader - Data Toggle Write Enable"]
pub type USB_RXCSRH5_DTWE_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH5_DTWE` writer - Data Toggle Write Enable"]
pub type USB_RXCSRH5_DTWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH5_DMAMOD` reader - DMA Request Mode"]
pub type USB_RXCSRH5_DMAMOD_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH5_DMAMOD` writer - DMA Request Mode"]
pub type USB_RXCSRH5_DMAMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH5_PIDERR` reader - PID Error"]
pub type USB_RXCSRH5_PIDERR_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH5_PIDERR` writer - PID Error"]
pub type USB_RXCSRH5_PIDERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH5_DMAEN` reader - DMA Request Enable"]
pub type USB_RXCSRH5_DMAEN_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH5_DMAEN` writer - DMA Request Enable"]
pub type USB_RXCSRH5_DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH5_AUTORQ` reader - Auto Request"]
pub type USB_RXCSRH5_AUTORQ_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH5_AUTORQ` writer - Auto Request"]
pub type USB_RXCSRH5_AUTORQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXCSRH5_AUTOCL` reader - Auto Clear"]
pub type USB_RXCSRH5_AUTOCL_R = crate::BitReader;
#[doc = "Field `USB_RXCSRH5_AUTOCL` writer - Auto Clear"]
pub type USB_RXCSRH5_AUTOCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh5_dt(&self) -> USB_RXCSRH5_DT_R {
        USB_RXCSRH5_DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh5_dtwe(&self) -> USB_RXCSRH5_DTWE_R {
        USB_RXCSRH5_DTWE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh5_dmamod(&self) -> USB_RXCSRH5_DMAMOD_R {
        USB_RXCSRH5_DMAMOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh5_piderr(&self) -> USB_RXCSRH5_PIDERR_R {
        USB_RXCSRH5_PIDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh5_dmaen(&self) -> USB_RXCSRH5_DMAEN_R {
        USB_RXCSRH5_DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh5_autorq(&self) -> USB_RXCSRH5_AUTORQ_R {
        USB_RXCSRH5_AUTORQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh5_autocl(&self) -> USB_RXCSRH5_AUTOCL_R {
        USB_RXCSRH5_AUTOCL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh5_dt(&mut self) -> USB_RXCSRH5_DT_W<RXCSRH5_SPEC, 1> {
        USB_RXCSRH5_DT_W::new(self)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh5_dtwe(&mut self) -> USB_RXCSRH5_DTWE_W<RXCSRH5_SPEC, 2> {
        USB_RXCSRH5_DTWE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh5_dmamod(&mut self) -> USB_RXCSRH5_DMAMOD_W<RXCSRH5_SPEC, 3> {
        USB_RXCSRH5_DMAMOD_W::new(self)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh5_piderr(&mut self) -> USB_RXCSRH5_PIDERR_W<RXCSRH5_SPEC, 4> {
        USB_RXCSRH5_PIDERR_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh5_dmaen(&mut self) -> USB_RXCSRH5_DMAEN_W<RXCSRH5_SPEC, 5> {
        USB_RXCSRH5_DMAEN_W::new(self)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh5_autorq(&mut self) -> USB_RXCSRH5_AUTORQ_W<RXCSRH5_SPEC, 6> {
        USB_RXCSRH5_AUTORQ_W::new(self)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxcsrh5_autocl(&mut self) -> USB_RXCSRH5_AUTOCL_W<RXCSRH5_SPEC, 7> {
        USB_RXCSRH5_AUTOCL_W::new(self)
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
#[doc = "USB Receive Control and Status Endpoint 5 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCSRH5_SPEC;
impl crate::RegisterSpec for RXCSRH5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcsrh5::R`](R) reader structure"]
impl crate::Readable for RXCSRH5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxcsrh5::W`](W) writer structure"]
impl crate::Writable for RXCSRH5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCSRH5 to value 0"]
impl crate::Resettable for RXCSRH5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
