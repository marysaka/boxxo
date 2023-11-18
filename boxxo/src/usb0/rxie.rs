#[doc = "Register `RXIE` reader"]
pub type R = crate::R<RXIE_SPEC>;
#[doc = "Register `RXIE` writer"]
pub type W = crate::W<RXIE_SPEC>;
#[doc = "Field `USB_RXIE_EP1` reader - RX Endpoint 1 Interrupt Enable"]
pub type USB_RXIE_EP1_R = crate::BitReader;
#[doc = "Field `USB_RXIE_EP1` writer - RX Endpoint 1 Interrupt Enable"]
pub type USB_RXIE_EP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXIE_EP2` reader - RX Endpoint 2 Interrupt Enable"]
pub type USB_RXIE_EP2_R = crate::BitReader;
#[doc = "Field `USB_RXIE_EP2` writer - RX Endpoint 2 Interrupt Enable"]
pub type USB_RXIE_EP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXIE_EP3` reader - RX Endpoint 3 Interrupt Enable"]
pub type USB_RXIE_EP3_R = crate::BitReader;
#[doc = "Field `USB_RXIE_EP3` writer - RX Endpoint 3 Interrupt Enable"]
pub type USB_RXIE_EP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXIE_EP4` reader - RX Endpoint 4 Interrupt Enable"]
pub type USB_RXIE_EP4_R = crate::BitReader;
#[doc = "Field `USB_RXIE_EP4` writer - RX Endpoint 4 Interrupt Enable"]
pub type USB_RXIE_EP4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXIE_EP5` reader - RX Endpoint 5 Interrupt Enable"]
pub type USB_RXIE_EP5_R = crate::BitReader;
#[doc = "Field `USB_RXIE_EP5` writer - RX Endpoint 5 Interrupt Enable"]
pub type USB_RXIE_EP5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXIE_EP6` reader - RX Endpoint 6 Interrupt Enable"]
pub type USB_RXIE_EP6_R = crate::BitReader;
#[doc = "Field `USB_RXIE_EP6` writer - RX Endpoint 6 Interrupt Enable"]
pub type USB_RXIE_EP6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_RXIE_EP7` reader - RX Endpoint 7 Interrupt Enable"]
pub type USB_RXIE_EP7_R = crate::BitReader;
#[doc = "Field `USB_RXIE_EP7` writer - RX Endpoint 7 Interrupt Enable"]
pub type USB_RXIE_EP7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - RX Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_rxie_ep1(&self) -> USB_RXIE_EP1_R {
        USB_RXIE_EP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_rxie_ep2(&self) -> USB_RXIE_EP2_R {
        USB_RXIE_EP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_rxie_ep3(&self) -> USB_RXIE_EP3_R {
        USB_RXIE_EP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_rxie_ep4(&self) -> USB_RXIE_EP4_R {
        USB_RXIE_EP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_rxie_ep5(&self) -> USB_RXIE_EP5_R {
        USB_RXIE_EP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_rxie_ep6(&self) -> USB_RXIE_EP6_R {
        USB_RXIE_EP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Endpoint 7 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_rxie_ep7(&self) -> USB_RXIE_EP7_R {
        USB_RXIE_EP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RX Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxie_ep1(&mut self) -> USB_RXIE_EP1_W<RXIE_SPEC, 1> {
        USB_RXIE_EP1_W::new(self)
    }
    #[doc = "Bit 2 - RX Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxie_ep2(&mut self) -> USB_RXIE_EP2_W<RXIE_SPEC, 2> {
        USB_RXIE_EP2_W::new(self)
    }
    #[doc = "Bit 3 - RX Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxie_ep3(&mut self) -> USB_RXIE_EP3_W<RXIE_SPEC, 3> {
        USB_RXIE_EP3_W::new(self)
    }
    #[doc = "Bit 4 - RX Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxie_ep4(&mut self) -> USB_RXIE_EP4_W<RXIE_SPEC, 4> {
        USB_RXIE_EP4_W::new(self)
    }
    #[doc = "Bit 5 - RX Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxie_ep5(&mut self) -> USB_RXIE_EP5_W<RXIE_SPEC, 5> {
        USB_RXIE_EP5_W::new(self)
    }
    #[doc = "Bit 6 - RX Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxie_ep6(&mut self) -> USB_RXIE_EP6_W<RXIE_SPEC, 6> {
        USB_RXIE_EP6_W::new(self)
    }
    #[doc = "Bit 7 - RX Endpoint 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rxie_ep7(&mut self) -> USB_RXIE_EP7_W<RXIE_SPEC, 7> {
        USB_RXIE_EP7_W::new(self)
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
#[doc = "USB Receive Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIE_SPEC;
impl crate::RegisterSpec for RXIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rxie::R`](R) reader structure"]
impl crate::Readable for RXIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxie::W`](W) writer structure"]
impl crate::Writable for RXIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXIE to value 0"]
impl crate::Resettable for RXIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
