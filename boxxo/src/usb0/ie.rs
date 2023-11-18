#[doc = "Register `IE` reader"]
pub type R = crate::R<IE_SPEC>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IE_SPEC>;
#[doc = "Field `USB_IE_SUSPND` reader - Enable SUSPEND Interrupt"]
pub type USB_IE_SUSPND_R = crate::BitReader;
#[doc = "Field `USB_IE_SUSPND` writer - Enable SUSPEND Interrupt"]
pub type USB_IE_SUSPND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IE_RESUME` reader - Enable RESUME Interrupt"]
pub type USB_IE_RESUME_R = crate::BitReader;
#[doc = "Field `USB_IE_RESUME` writer - Enable RESUME Interrupt"]
pub type USB_IE_RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IE_BABBLE` reader - Enable Babble Interrupt"]
pub type USB_IE_BABBLE_R = crate::BitReader;
#[doc = "Field `USB_IE_BABBLE` writer - Enable Babble Interrupt"]
pub type USB_IE_BABBLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IE_SOF` reader - Enable Start-of-Frame Interrupt"]
pub type USB_IE_SOF_R = crate::BitReader;
#[doc = "Field `USB_IE_SOF` writer - Enable Start-of-Frame Interrupt"]
pub type USB_IE_SOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IE_CONN` reader - Enable Connect Interrupt"]
pub type USB_IE_CONN_R = crate::BitReader;
#[doc = "Field `USB_IE_CONN` writer - Enable Connect Interrupt"]
pub type USB_IE_CONN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IE_DISCON` reader - Enable Disconnect Interrupt"]
pub type USB_IE_DISCON_R = crate::BitReader;
#[doc = "Field `USB_IE_DISCON` writer - Enable Disconnect Interrupt"]
pub type USB_IE_DISCON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IE_SESREQ` reader - Enable Session Request"]
pub type USB_IE_SESREQ_R = crate::BitReader;
#[doc = "Field `USB_IE_SESREQ` writer - Enable Session Request"]
pub type USB_IE_SESREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IE_VBUSERR` reader - Enable VBUS Error Interrupt"]
pub type USB_IE_VBUSERR_R = crate::BitReader;
#[doc = "Field `USB_IE_VBUSERR` writer - Enable VBUS Error Interrupt"]
pub type USB_IE_VBUSERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable SUSPEND Interrupt"]
    #[inline(always)]
    pub fn usb_ie_suspnd(&self) -> USB_IE_SUSPND_R {
        USB_IE_SUSPND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RESUME Interrupt"]
    #[inline(always)]
    pub fn usb_ie_resume(&self) -> USB_IE_RESUME_R {
        USB_IE_RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Babble Interrupt"]
    #[inline(always)]
    pub fn usb_ie_babble(&self) -> USB_IE_BABBLE_R {
        USB_IE_BABBLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Start-of-Frame Interrupt"]
    #[inline(always)]
    pub fn usb_ie_sof(&self) -> USB_IE_SOF_R {
        USB_IE_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Connect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_conn(&self) -> USB_IE_CONN_R {
        USB_IE_CONN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Disconnect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_discon(&self) -> USB_IE_DISCON_R {
        USB_IE_DISCON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Session Request"]
    #[inline(always)]
    pub fn usb_ie_sesreq(&self) -> USB_IE_SESREQ_R {
        USB_IE_SESREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable VBUS Error Interrupt"]
    #[inline(always)]
    pub fn usb_ie_vbuserr(&self) -> USB_IE_VBUSERR_R {
        USB_IE_VBUSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SUSPEND Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ie_suspnd(&mut self) -> USB_IE_SUSPND_W<IE_SPEC, 0> {
        USB_IE_SUSPND_W::new(self)
    }
    #[doc = "Bit 1 - Enable RESUME Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ie_resume(&mut self) -> USB_IE_RESUME_W<IE_SPEC, 1> {
        USB_IE_RESUME_W::new(self)
    }
    #[doc = "Bit 2 - Enable Babble Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ie_babble(&mut self) -> USB_IE_BABBLE_W<IE_SPEC, 2> {
        USB_IE_BABBLE_W::new(self)
    }
    #[doc = "Bit 3 - Enable Start-of-Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ie_sof(&mut self) -> USB_IE_SOF_W<IE_SPEC, 3> {
        USB_IE_SOF_W::new(self)
    }
    #[doc = "Bit 4 - Enable Connect Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ie_conn(&mut self) -> USB_IE_CONN_W<IE_SPEC, 4> {
        USB_IE_CONN_W::new(self)
    }
    #[doc = "Bit 5 - Enable Disconnect Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ie_discon(&mut self) -> USB_IE_DISCON_W<IE_SPEC, 5> {
        USB_IE_DISCON_W::new(self)
    }
    #[doc = "Bit 6 - Enable Session Request"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ie_sesreq(&mut self) -> USB_IE_SESREQ_W<IE_SPEC, 6> {
        USB_IE_SESREQ_W::new(self)
    }
    #[doc = "Bit 7 - Enable VBUS Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ie_vbuserr(&mut self) -> USB_IE_VBUSERR_W<IE_SPEC, 7> {
        USB_IE_VBUSERR_W::new(self)
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
#[doc = "USB Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
