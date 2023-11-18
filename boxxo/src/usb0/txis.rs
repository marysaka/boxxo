#[doc = "Register `TXIS` reader"]
pub type R = crate::R<TXIS_SPEC>;
#[doc = "Register `TXIS` writer"]
pub type W = crate::W<TXIS_SPEC>;
#[doc = "Field `USB_TXIS_EP0` reader - TX and RX Endpoint 0 Interrupt"]
pub type USB_TXIS_EP0_R = crate::BitReader;
#[doc = "Field `USB_TXIS_EP0` writer - TX and RX Endpoint 0 Interrupt"]
pub type USB_TXIS_EP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXIS_EP1` reader - TX Endpoint 1 Interrupt"]
pub type USB_TXIS_EP1_R = crate::BitReader;
#[doc = "Field `USB_TXIS_EP1` writer - TX Endpoint 1 Interrupt"]
pub type USB_TXIS_EP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXIS_EP2` reader - TX Endpoint 2 Interrupt"]
pub type USB_TXIS_EP2_R = crate::BitReader;
#[doc = "Field `USB_TXIS_EP2` writer - TX Endpoint 2 Interrupt"]
pub type USB_TXIS_EP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXIS_EP3` reader - TX Endpoint 3 Interrupt"]
pub type USB_TXIS_EP3_R = crate::BitReader;
#[doc = "Field `USB_TXIS_EP3` writer - TX Endpoint 3 Interrupt"]
pub type USB_TXIS_EP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXIS_EP4` reader - TX Endpoint 4 Interrupt"]
pub type USB_TXIS_EP4_R = crate::BitReader;
#[doc = "Field `USB_TXIS_EP4` writer - TX Endpoint 4 Interrupt"]
pub type USB_TXIS_EP4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXIS_EP5` reader - TX Endpoint 5 Interrupt"]
pub type USB_TXIS_EP5_R = crate::BitReader;
#[doc = "Field `USB_TXIS_EP5` writer - TX Endpoint 5 Interrupt"]
pub type USB_TXIS_EP5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXIS_EP6` reader - TX Endpoint 6 Interrupt"]
pub type USB_TXIS_EP6_R = crate::BitReader;
#[doc = "Field `USB_TXIS_EP6` writer - TX Endpoint 6 Interrupt"]
pub type USB_TXIS_EP6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXIS_EP7` reader - TX Endpoint 7 Interrupt"]
pub type USB_TXIS_EP7_R = crate::BitReader;
#[doc = "Field `USB_TXIS_EP7` writer - TX Endpoint 7 Interrupt"]
pub type USB_TXIS_EP7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TX and RX Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn usb_txis_ep0(&self) -> USB_TXIS_EP0_R {
        USB_TXIS_EP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn usb_txis_ep1(&self) -> USB_TXIS_EP1_R {
        USB_TXIS_EP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn usb_txis_ep2(&self) -> USB_TXIS_EP2_R {
        USB_TXIS_EP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn usb_txis_ep3(&self) -> USB_TXIS_EP3_R {
        USB_TXIS_EP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn usb_txis_ep4(&self) -> USB_TXIS_EP4_R {
        USB_TXIS_EP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn usb_txis_ep5(&self) -> USB_TXIS_EP5_R {
        USB_TXIS_EP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn usb_txis_ep6(&self) -> USB_TXIS_EP6_R {
        USB_TXIS_EP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn usb_txis_ep7(&self) -> USB_TXIS_EP7_R {
        USB_TXIS_EP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX and RX Endpoint 0 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txis_ep0(&mut self) -> USB_TXIS_EP0_W<TXIS_SPEC, 0> {
        USB_TXIS_EP0_W::new(self)
    }
    #[doc = "Bit 1 - TX Endpoint 1 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txis_ep1(&mut self) -> USB_TXIS_EP1_W<TXIS_SPEC, 1> {
        USB_TXIS_EP1_W::new(self)
    }
    #[doc = "Bit 2 - TX Endpoint 2 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txis_ep2(&mut self) -> USB_TXIS_EP2_W<TXIS_SPEC, 2> {
        USB_TXIS_EP2_W::new(self)
    }
    #[doc = "Bit 3 - TX Endpoint 3 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txis_ep3(&mut self) -> USB_TXIS_EP3_W<TXIS_SPEC, 3> {
        USB_TXIS_EP3_W::new(self)
    }
    #[doc = "Bit 4 - TX Endpoint 4 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txis_ep4(&mut self) -> USB_TXIS_EP4_W<TXIS_SPEC, 4> {
        USB_TXIS_EP4_W::new(self)
    }
    #[doc = "Bit 5 - TX Endpoint 5 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txis_ep5(&mut self) -> USB_TXIS_EP5_W<TXIS_SPEC, 5> {
        USB_TXIS_EP5_W::new(self)
    }
    #[doc = "Bit 6 - TX Endpoint 6 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txis_ep6(&mut self) -> USB_TXIS_EP6_W<TXIS_SPEC, 6> {
        USB_TXIS_EP6_W::new(self)
    }
    #[doc = "Bit 7 - TX Endpoint 7 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txis_ep7(&mut self) -> USB_TXIS_EP7_W<TXIS_SPEC, 7> {
        USB_TXIS_EP7_W::new(self)
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
#[doc = "USB Transmit Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXIS_SPEC;
impl crate::RegisterSpec for TXIS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`txis::R`](R) reader structure"]
impl crate::Readable for TXIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txis::W`](W) writer structure"]
impl crate::Writable for TXIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXIS to value 0"]
impl crate::Resettable for TXIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
