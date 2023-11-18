#[doc = "Register `TXCSRH2` reader"]
pub type R = crate::R<TXCSRH2_SPEC>;
#[doc = "Register `TXCSRH2` writer"]
pub type W = crate::W<TXCSRH2_SPEC>;
#[doc = "Field `USB_TXCSRH2_DT` reader - Data Toggle"]
pub type USB_TXCSRH2_DT_R = crate::BitReader;
#[doc = "Field `USB_TXCSRH2_DT` writer - Data Toggle"]
pub type USB_TXCSRH2_DT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRH2_DTWE` reader - Data Toggle Write Enable"]
pub type USB_TXCSRH2_DTWE_R = crate::BitReader;
#[doc = "Field `USB_TXCSRH2_DTWE` writer - Data Toggle Write Enable"]
pub type USB_TXCSRH2_DTWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRH2_DMAMOD` reader - DMA Request Mode"]
pub type USB_TXCSRH2_DMAMOD_R = crate::BitReader;
#[doc = "Field `USB_TXCSRH2_DMAMOD` writer - DMA Request Mode"]
pub type USB_TXCSRH2_DMAMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRH2_FDT` reader - Force Data Toggle"]
pub type USB_TXCSRH2_FDT_R = crate::BitReader;
#[doc = "Field `USB_TXCSRH2_FDT` writer - Force Data Toggle"]
pub type USB_TXCSRH2_FDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRH2_DMAEN` reader - DMA Request Enable"]
pub type USB_TXCSRH2_DMAEN_R = crate::BitReader;
#[doc = "Field `USB_TXCSRH2_DMAEN` writer - DMA Request Enable"]
pub type USB_TXCSRH2_DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRH2_MODE` reader - Mode"]
pub type USB_TXCSRH2_MODE_R = crate::BitReader;
#[doc = "Field `USB_TXCSRH2_MODE` writer - Mode"]
pub type USB_TXCSRH2_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRH2_ISO` reader - Isochronous Transfers"]
pub type USB_TXCSRH2_ISO_R = crate::BitReader;
#[doc = "Field `USB_TXCSRH2_ISO` writer - Isochronous Transfers"]
pub type USB_TXCSRH2_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXCSRH2_AUTOSET` reader - Auto Set"]
pub type USB_TXCSRH2_AUTOSET_R = crate::BitReader;
#[doc = "Field `USB_TXCSRH2_AUTOSET` writer - Auto Set"]
pub type USB_TXCSRH2_AUTOSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrh2_dt(&self) -> USB_TXCSRH2_DT_R {
        USB_TXCSRH2_DT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_txcsrh2_dtwe(&self) -> USB_TXCSRH2_DTWE_R {
        USB_TXCSRH2_DTWE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_txcsrh2_dmamod(&self) -> USB_TXCSRH2_DMAMOD_R {
        USB_TXCSRH2_DMAMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrh2_fdt(&self) -> USB_TXCSRH2_FDT_R {
        USB_TXCSRH2_FDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_txcsrh2_dmaen(&self) -> USB_TXCSRH2_DMAEN_R {
        USB_TXCSRH2_DMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode"]
    #[inline(always)]
    pub fn usb_txcsrh2_mode(&self) -> USB_TXCSRH2_MODE_R {
        USB_TXCSRH2_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn usb_txcsrh2_iso(&self) -> USB_TXCSRH2_ISO_R {
        USB_TXCSRH2_ISO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto Set"]
    #[inline(always)]
    pub fn usb_txcsrh2_autoset(&self) -> USB_TXCSRH2_AUTOSET_R {
        USB_TXCSRH2_AUTOSET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrh2_dt(&mut self) -> USB_TXCSRH2_DT_W<TXCSRH2_SPEC, 0> {
        USB_TXCSRH2_DT_W::new(self)
    }
    #[doc = "Bit 1 - Data Toggle Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrh2_dtwe(&mut self) -> USB_TXCSRH2_DTWE_W<TXCSRH2_SPEC, 1> {
        USB_TXCSRH2_DTWE_W::new(self)
    }
    #[doc = "Bit 2 - DMA Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrh2_dmamod(&mut self) -> USB_TXCSRH2_DMAMOD_W<TXCSRH2_SPEC, 2> {
        USB_TXCSRH2_DMAMOD_W::new(self)
    }
    #[doc = "Bit 3 - Force Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrh2_fdt(&mut self) -> USB_TXCSRH2_FDT_W<TXCSRH2_SPEC, 3> {
        USB_TXCSRH2_FDT_W::new(self)
    }
    #[doc = "Bit 4 - DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrh2_dmaen(&mut self) -> USB_TXCSRH2_DMAEN_W<TXCSRH2_SPEC, 4> {
        USB_TXCSRH2_DMAEN_W::new(self)
    }
    #[doc = "Bit 5 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrh2_mode(&mut self) -> USB_TXCSRH2_MODE_W<TXCSRH2_SPEC, 5> {
        USB_TXCSRH2_MODE_W::new(self)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrh2_iso(&mut self) -> USB_TXCSRH2_ISO_W<TXCSRH2_SPEC, 6> {
        USB_TXCSRH2_ISO_W::new(self)
    }
    #[doc = "Bit 7 - Auto Set"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txcsrh2_autoset(&mut self) -> USB_TXCSRH2_AUTOSET_W<TXCSRH2_SPEC, 7> {
        USB_TXCSRH2_AUTOSET_W::new(self)
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
#[doc = "USB Transmit Control and Status Endpoint 2 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrh2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrh2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCSRH2_SPEC;
impl crate::RegisterSpec for TXCSRH2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txcsrh2::R`](R) reader structure"]
impl crate::Readable for TXCSRH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txcsrh2::W`](W) writer structure"]
impl crate::Writable for TXCSRH2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCSRH2 to value 0"]
impl crate::Resettable for TXCSRH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
