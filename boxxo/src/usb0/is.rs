#[doc = "Register `IS` reader"]
pub type R = crate::R<IS_SPEC>;
#[doc = "Register `IS` writer"]
pub type W = crate::W<IS_SPEC>;
#[doc = "Field `USB_IS_SUSPEND` reader - SUSPEND Signaling Detected"]
pub type USB_IS_SUSPEND_R = crate::BitReader;
#[doc = "Field `USB_IS_SUSPEND` writer - SUSPEND Signaling Detected"]
pub type USB_IS_SUSPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IS_RESUME` reader - RESUME Signaling Detected"]
pub type USB_IS_RESUME_R = crate::BitReader;
#[doc = "Field `USB_IS_RESUME` writer - RESUME Signaling Detected"]
pub type USB_IS_RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IS_BABBLE` reader - Babble Detected"]
pub type USB_IS_BABBLE_R = crate::BitReader;
#[doc = "Field `USB_IS_BABBLE` writer - Babble Detected"]
pub type USB_IS_BABBLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IS_SOF` reader - Start of Frame"]
pub type USB_IS_SOF_R = crate::BitReader;
#[doc = "Field `USB_IS_SOF` writer - Start of Frame"]
pub type USB_IS_SOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IS_CONN` reader - Session Connect"]
pub type USB_IS_CONN_R = crate::BitReader;
#[doc = "Field `USB_IS_CONN` writer - Session Connect"]
pub type USB_IS_CONN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IS_DISCON` reader - Session Disconnect"]
pub type USB_IS_DISCON_R = crate::BitReader;
#[doc = "Field `USB_IS_DISCON` writer - Session Disconnect"]
pub type USB_IS_DISCON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IS_SESREQ` reader - SESSION REQUEST"]
pub type USB_IS_SESREQ_R = crate::BitReader;
#[doc = "Field `USB_IS_SESREQ` writer - SESSION REQUEST"]
pub type USB_IS_SESREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IS_VBUSERR` reader - VBUS Error"]
pub type USB_IS_VBUSERR_R = crate::BitReader;
#[doc = "Field `USB_IS_VBUSERR` writer - VBUS Error"]
pub type USB_IS_VBUSERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SUSPEND Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_suspend(&self) -> USB_IS_SUSPEND_R {
        USB_IS_SUSPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RESUME Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_resume(&self) -> USB_IS_RESUME_R {
        USB_IS_RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Babble Detected"]
    #[inline(always)]
    pub fn usb_is_babble(&self) -> USB_IS_BABBLE_R {
        USB_IS_BABBLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn usb_is_sof(&self) -> USB_IS_SOF_R {
        USB_IS_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Session Connect"]
    #[inline(always)]
    pub fn usb_is_conn(&self) -> USB_IS_CONN_R {
        USB_IS_CONN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Session Disconnect"]
    #[inline(always)]
    pub fn usb_is_discon(&self) -> USB_IS_DISCON_R {
        USB_IS_DISCON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SESSION REQUEST"]
    #[inline(always)]
    pub fn usb_is_sesreq(&self) -> USB_IS_SESREQ_R {
        USB_IS_SESREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VBUS Error"]
    #[inline(always)]
    pub fn usb_is_vbuserr(&self) -> USB_IS_VBUSERR_R {
        USB_IS_VBUSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SUSPEND Signaling Detected"]
    #[inline(always)]
    #[must_use]
    pub fn usb_is_suspend(&mut self) -> USB_IS_SUSPEND_W<IS_SPEC, 0> {
        USB_IS_SUSPEND_W::new(self)
    }
    #[doc = "Bit 1 - RESUME Signaling Detected"]
    #[inline(always)]
    #[must_use]
    pub fn usb_is_resume(&mut self) -> USB_IS_RESUME_W<IS_SPEC, 1> {
        USB_IS_RESUME_W::new(self)
    }
    #[doc = "Bit 2 - Babble Detected"]
    #[inline(always)]
    #[must_use]
    pub fn usb_is_babble(&mut self) -> USB_IS_BABBLE_W<IS_SPEC, 2> {
        USB_IS_BABBLE_W::new(self)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn usb_is_sof(&mut self) -> USB_IS_SOF_W<IS_SPEC, 3> {
        USB_IS_SOF_W::new(self)
    }
    #[doc = "Bit 4 - Session Connect"]
    #[inline(always)]
    #[must_use]
    pub fn usb_is_conn(&mut self) -> USB_IS_CONN_W<IS_SPEC, 4> {
        USB_IS_CONN_W::new(self)
    }
    #[doc = "Bit 5 - Session Disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn usb_is_discon(&mut self) -> USB_IS_DISCON_W<IS_SPEC, 5> {
        USB_IS_DISCON_W::new(self)
    }
    #[doc = "Bit 6 - SESSION REQUEST"]
    #[inline(always)]
    #[must_use]
    pub fn usb_is_sesreq(&mut self) -> USB_IS_SESREQ_W<IS_SPEC, 6> {
        USB_IS_SESREQ_W::new(self)
    }
    #[doc = "Bit 7 - VBUS Error"]
    #[inline(always)]
    #[must_use]
    pub fn usb_is_vbuserr(&mut self) -> USB_IS_VBUSERR_W<IS_SPEC, 7> {
        USB_IS_VBUSERR_W::new(self)
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
#[doc = "USB General Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IS_SPEC;
impl crate::RegisterSpec for IS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`is::R`](R) reader structure"]
impl crate::Readable for IS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`is::W`](W) writer structure"]
impl crate::Writable for IS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IS to value 0"]
impl crate::Resettable for IS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
