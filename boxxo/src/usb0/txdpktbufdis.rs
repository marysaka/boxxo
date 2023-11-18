#[doc = "Register `TXDPKTBUFDIS` reader"]
pub type R = crate::R<TXDPKTBUFDIS_SPEC>;
#[doc = "Register `TXDPKTBUFDIS` writer"]
pub type W = crate::W<TXDPKTBUFDIS_SPEC>;
#[doc = "Field `USB_TXDPKTBUFDIS_EP1` reader - EP1 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP1_R = crate::BitReader;
#[doc = "Field `USB_TXDPKTBUFDIS_EP1` writer - EP1 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXDPKTBUFDIS_EP2` reader - EP2 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP2_R = crate::BitReader;
#[doc = "Field `USB_TXDPKTBUFDIS_EP2` writer - EP2 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXDPKTBUFDIS_EP3` reader - EP3 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP3_R = crate::BitReader;
#[doc = "Field `USB_TXDPKTBUFDIS_EP3` writer - EP3 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXDPKTBUFDIS_EP4` reader - EP4 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP4_R = crate::BitReader;
#[doc = "Field `USB_TXDPKTBUFDIS_EP4` writer - EP4 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXDPKTBUFDIS_EP5` reader - EP5 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP5_R = crate::BitReader;
#[doc = "Field `USB_TXDPKTBUFDIS_EP5` writer - EP5 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXDPKTBUFDIS_EP6` reader - EP6 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP6_R = crate::BitReader;
#[doc = "Field `USB_TXDPKTBUFDIS_EP6` writer - EP6 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TXDPKTBUFDIS_EP7` reader - EP7 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP7_R = crate::BitReader;
#[doc = "Field `USB_TXDPKTBUFDIS_EP7` writer - EP7 TX Double-Packet Buffer Disable"]
pub type USB_TXDPKTBUFDIS_EP7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - EP1 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_txdpktbufdis_ep1(&self) -> USB_TXDPKTBUFDIS_EP1_R {
        USB_TXDPKTBUFDIS_EP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP2 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_txdpktbufdis_ep2(&self) -> USB_TXDPKTBUFDIS_EP2_R {
        USB_TXDPKTBUFDIS_EP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP3 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_txdpktbufdis_ep3(&self) -> USB_TXDPKTBUFDIS_EP3_R {
        USB_TXDPKTBUFDIS_EP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP4 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_txdpktbufdis_ep4(&self) -> USB_TXDPKTBUFDIS_EP4_R {
        USB_TXDPKTBUFDIS_EP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EP5 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_txdpktbufdis_ep5(&self) -> USB_TXDPKTBUFDIS_EP5_R {
        USB_TXDPKTBUFDIS_EP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EP6 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_txdpktbufdis_ep6(&self) -> USB_TXDPKTBUFDIS_EP6_R {
        USB_TXDPKTBUFDIS_EP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EP7 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_txdpktbufdis_ep7(&self) -> USB_TXDPKTBUFDIS_EP7_R {
        USB_TXDPKTBUFDIS_EP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - EP1 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txdpktbufdis_ep1(&mut self) -> USB_TXDPKTBUFDIS_EP1_W<TXDPKTBUFDIS_SPEC, 1> {
        USB_TXDPKTBUFDIS_EP1_W::new(self)
    }
    #[doc = "Bit 2 - EP2 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txdpktbufdis_ep2(&mut self) -> USB_TXDPKTBUFDIS_EP2_W<TXDPKTBUFDIS_SPEC, 2> {
        USB_TXDPKTBUFDIS_EP2_W::new(self)
    }
    #[doc = "Bit 3 - EP3 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txdpktbufdis_ep3(&mut self) -> USB_TXDPKTBUFDIS_EP3_W<TXDPKTBUFDIS_SPEC, 3> {
        USB_TXDPKTBUFDIS_EP3_W::new(self)
    }
    #[doc = "Bit 4 - EP4 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txdpktbufdis_ep4(&mut self) -> USB_TXDPKTBUFDIS_EP4_W<TXDPKTBUFDIS_SPEC, 4> {
        USB_TXDPKTBUFDIS_EP4_W::new(self)
    }
    #[doc = "Bit 5 - EP5 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txdpktbufdis_ep5(&mut self) -> USB_TXDPKTBUFDIS_EP5_W<TXDPKTBUFDIS_SPEC, 5> {
        USB_TXDPKTBUFDIS_EP5_W::new(self)
    }
    #[doc = "Bit 6 - EP6 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txdpktbufdis_ep6(&mut self) -> USB_TXDPKTBUFDIS_EP6_W<TXDPKTBUFDIS_SPEC, 6> {
        USB_TXDPKTBUFDIS_EP6_W::new(self)
    }
    #[doc = "Bit 7 - EP7 TX Double-Packet Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txdpktbufdis_ep7(&mut self) -> USB_TXDPKTBUFDIS_EP7_W<TXDPKTBUFDIS_SPEC, 7> {
        USB_TXDPKTBUFDIS_EP7_W::new(self)
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
#[doc = "USB Transmit Double Packet Buffer Disable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdpktbufdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdpktbufdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDPKTBUFDIS_SPEC;
impl crate::RegisterSpec for TXDPKTBUFDIS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`txdpktbufdis::R`](R) reader structure"]
impl crate::Readable for TXDPKTBUFDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdpktbufdis::W`](W) writer structure"]
impl crate::Writable for TXDPKTBUFDIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDPKTBUFDIS to value 0"]
impl crate::Resettable for TXDPKTBUFDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
