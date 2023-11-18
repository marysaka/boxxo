#[doc = "Register `POWER` reader"]
pub type R = crate::R<POWER_SPEC>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<POWER_SPEC>;
#[doc = "Field `USB_POWER_PWRDNPHY` reader - Power Down PHY"]
pub type USB_POWER_PWRDNPHY_R = crate::BitReader;
#[doc = "Field `USB_POWER_PWRDNPHY` writer - Power Down PHY"]
pub type USB_POWER_PWRDNPHY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_POWER_SUSPEND` reader - SUSPEND Mode"]
pub type USB_POWER_SUSPEND_R = crate::BitReader;
#[doc = "Field `USB_POWER_SUSPEND` writer - SUSPEND Mode"]
pub type USB_POWER_SUSPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_POWER_RESUME` reader - RESUME Signaling"]
pub type USB_POWER_RESUME_R = crate::BitReader;
#[doc = "Field `USB_POWER_RESUME` writer - RESUME Signaling"]
pub type USB_POWER_RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_POWER_RESET` reader - RESET Signaling"]
pub type USB_POWER_RESET_R = crate::BitReader;
#[doc = "Field `USB_POWER_RESET` writer - RESET Signaling"]
pub type USB_POWER_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_POWER_SOFTCONN` reader - Soft Connect/Disconnect"]
pub type USB_POWER_SOFTCONN_R = crate::BitReader;
#[doc = "Field `USB_POWER_SOFTCONN` writer - Soft Connect/Disconnect"]
pub type USB_POWER_SOFTCONN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_POWER_ISOUP` reader - Isochronous Update"]
pub type USB_POWER_ISOUP_R = crate::BitReader;
#[doc = "Field `USB_POWER_ISOUP` writer - Isochronous Update"]
pub type USB_POWER_ISOUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Power Down PHY"]
    #[inline(always)]
    pub fn usb_power_pwrdnphy(&self) -> USB_POWER_PWRDNPHY_R {
        USB_POWER_PWRDNPHY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SUSPEND Mode"]
    #[inline(always)]
    pub fn usb_power_suspend(&self) -> USB_POWER_SUSPEND_R {
        USB_POWER_SUSPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RESUME Signaling"]
    #[inline(always)]
    pub fn usb_power_resume(&self) -> USB_POWER_RESUME_R {
        USB_POWER_RESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RESET Signaling"]
    #[inline(always)]
    pub fn usb_power_reset(&self) -> USB_POWER_RESET_R {
        USB_POWER_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Soft Connect/Disconnect"]
    #[inline(always)]
    pub fn usb_power_softconn(&self) -> USB_POWER_SOFTCONN_R {
        USB_POWER_SOFTCONN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Isochronous Update"]
    #[inline(always)]
    pub fn usb_power_isoup(&self) -> USB_POWER_ISOUP_R {
        USB_POWER_ISOUP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down PHY"]
    #[inline(always)]
    #[must_use]
    pub fn usb_power_pwrdnphy(&mut self) -> USB_POWER_PWRDNPHY_W<POWER_SPEC, 0> {
        USB_POWER_PWRDNPHY_W::new(self)
    }
    #[doc = "Bit 1 - SUSPEND Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_power_suspend(&mut self) -> USB_POWER_SUSPEND_W<POWER_SPEC, 1> {
        USB_POWER_SUSPEND_W::new(self)
    }
    #[doc = "Bit 2 - RESUME Signaling"]
    #[inline(always)]
    #[must_use]
    pub fn usb_power_resume(&mut self) -> USB_POWER_RESUME_W<POWER_SPEC, 2> {
        USB_POWER_RESUME_W::new(self)
    }
    #[doc = "Bit 3 - RESET Signaling"]
    #[inline(always)]
    #[must_use]
    pub fn usb_power_reset(&mut self) -> USB_POWER_RESET_W<POWER_SPEC, 3> {
        USB_POWER_RESET_W::new(self)
    }
    #[doc = "Bit 6 - Soft Connect/Disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn usb_power_softconn(&mut self) -> USB_POWER_SOFTCONN_W<POWER_SPEC, 6> {
        USB_POWER_SOFTCONN_W::new(self)
    }
    #[doc = "Bit 7 - Isochronous Update"]
    #[inline(always)]
    #[must_use]
    pub fn usb_power_isoup(&mut self) -> USB_POWER_ISOUP_W<POWER_SPEC, 7> {
        USB_POWER_ISOUP_W::new(self)
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
#[doc = "USB Power\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_SPEC;
impl crate::RegisterSpec for POWER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for POWER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
