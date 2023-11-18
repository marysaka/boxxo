#[doc = "Register `DEVCTL` reader"]
pub type R = crate::R<DEVCTL_SPEC>;
#[doc = "Register `DEVCTL` writer"]
pub type W = crate::W<DEVCTL_SPEC>;
#[doc = "Field `USB_DEVCTL_SESSION` reader - Session Start/End"]
pub type USB_DEVCTL_SESSION_R = crate::BitReader;
#[doc = "Field `USB_DEVCTL_SESSION` writer - Session Start/End"]
pub type USB_DEVCTL_SESSION_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_DEVCTL_HOSTREQ` reader - Host Request"]
pub type USB_DEVCTL_HOSTREQ_R = crate::BitReader;
#[doc = "Field `USB_DEVCTL_HOSTREQ` writer - Host Request"]
pub type USB_DEVCTL_HOSTREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_DEVCTL_HOST` reader - Host Mode"]
pub type USB_DEVCTL_HOST_R = crate::BitReader;
#[doc = "Field `USB_DEVCTL_HOST` writer - Host Mode"]
pub type USB_DEVCTL_HOST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_DEVCTL_VBUS` reader - VBUS Level"]
pub type USB_DEVCTL_VBUS_R = crate::FieldReader<USB_DEVCTL_VBUS_A>;
#[doc = "VBUS Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_DEVCTL_VBUS_A {
    #[doc = "0: Below SessionEnd"]
    USB_DEVCTL_VBUS_NONE = 0,
    #[doc = "1: Above SessionEnd, below AValid"]
    USB_DEVCTL_VBUS_SEND = 1,
    #[doc = "2: Above AValid, below VBUSValid"]
    USB_DEVCTL_VBUS_AVALID = 2,
    #[doc = "3: Above VBUSValid"]
    USB_DEVCTL_VBUS_VALID = 3,
}
impl From<USB_DEVCTL_VBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_DEVCTL_VBUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USB_DEVCTL_VBUS_A {
    type Ux = u8;
}
impl USB_DEVCTL_VBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USB_DEVCTL_VBUS_A {
        match self.bits {
            0 => USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_NONE,
            1 => USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_SEND,
            2 => USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_AVALID,
            3 => USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_VALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Below SessionEnd"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_none(&self) -> bool {
        *self == USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_NONE
    }
    #[doc = "Above SessionEnd, below AValid"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_send(&self) -> bool {
        *self == USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_SEND
    }
    #[doc = "Above AValid, below VBUSValid"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_avalid(&self) -> bool {
        *self == USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_AVALID
    }
    #[doc = "Above VBUSValid"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_valid(&self) -> bool {
        *self == USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_VALID
    }
}
#[doc = "Field `USB_DEVCTL_VBUS` writer - VBUS Level"]
pub type USB_DEVCTL_VBUS_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, USB_DEVCTL_VBUS_A>;
impl<'a, REG, const O: u8> USB_DEVCTL_VBUS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Below SessionEnd"]
    #[inline(always)]
    pub fn usb_devctl_vbus_none(self) -> &'a mut crate::W<REG> {
        self.variant(USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_NONE)
    }
    #[doc = "Above SessionEnd, below AValid"]
    #[inline(always)]
    pub fn usb_devctl_vbus_send(self) -> &'a mut crate::W<REG> {
        self.variant(USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_SEND)
    }
    #[doc = "Above AValid, below VBUSValid"]
    #[inline(always)]
    pub fn usb_devctl_vbus_avalid(self) -> &'a mut crate::W<REG> {
        self.variant(USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_AVALID)
    }
    #[doc = "Above VBUSValid"]
    #[inline(always)]
    pub fn usb_devctl_vbus_valid(self) -> &'a mut crate::W<REG> {
        self.variant(USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_VALID)
    }
}
#[doc = "Field `USB_DEVCTL_LSDEV` reader - Low-Speed Device Detected"]
pub type USB_DEVCTL_LSDEV_R = crate::BitReader;
#[doc = "Field `USB_DEVCTL_LSDEV` writer - Low-Speed Device Detected"]
pub type USB_DEVCTL_LSDEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_DEVCTL_FSDEV` reader - Full-Speed Device Detected"]
pub type USB_DEVCTL_FSDEV_R = crate::BitReader;
#[doc = "Field `USB_DEVCTL_FSDEV` writer - Full-Speed Device Detected"]
pub type USB_DEVCTL_FSDEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_DEVCTL_DEV` reader - Device Mode"]
pub type USB_DEVCTL_DEV_R = crate::BitReader;
#[doc = "Field `USB_DEVCTL_DEV` writer - Device Mode"]
pub type USB_DEVCTL_DEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Session Start/End"]
    #[inline(always)]
    pub fn usb_devctl_session(&self) -> USB_DEVCTL_SESSION_R {
        USB_DEVCTL_SESSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Request"]
    #[inline(always)]
    pub fn usb_devctl_hostreq(&self) -> USB_DEVCTL_HOSTREQ_R {
        USB_DEVCTL_HOSTREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline(always)]
    pub fn usb_devctl_host(&self) -> USB_DEVCTL_HOST_R {
        USB_DEVCTL_HOST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - VBUS Level"]
    #[inline(always)]
    pub fn usb_devctl_vbus(&self) -> USB_DEVCTL_VBUS_R {
        USB_DEVCTL_VBUS_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_lsdev(&self) -> USB_DEVCTL_LSDEV_R {
        USB_DEVCTL_LSDEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_fsdev(&self) -> USB_DEVCTL_FSDEV_R {
        USB_DEVCTL_FSDEV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Device Mode"]
    #[inline(always)]
    pub fn usb_devctl_dev(&self) -> USB_DEVCTL_DEV_R {
        USB_DEVCTL_DEV_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Session Start/End"]
    #[inline(always)]
    #[must_use]
    pub fn usb_devctl_session(&mut self) -> USB_DEVCTL_SESSION_W<DEVCTL_SPEC, 0> {
        USB_DEVCTL_SESSION_W::new(self)
    }
    #[doc = "Bit 1 - Host Request"]
    #[inline(always)]
    #[must_use]
    pub fn usb_devctl_hostreq(&mut self) -> USB_DEVCTL_HOSTREQ_W<DEVCTL_SPEC, 1> {
        USB_DEVCTL_HOSTREQ_W::new(self)
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_devctl_host(&mut self) -> USB_DEVCTL_HOST_W<DEVCTL_SPEC, 2> {
        USB_DEVCTL_HOST_W::new(self)
    }
    #[doc = "Bits 3:4 - VBUS Level"]
    #[inline(always)]
    #[must_use]
    pub fn usb_devctl_vbus(&mut self) -> USB_DEVCTL_VBUS_W<DEVCTL_SPEC, 3> {
        USB_DEVCTL_VBUS_W::new(self)
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline(always)]
    #[must_use]
    pub fn usb_devctl_lsdev(&mut self) -> USB_DEVCTL_LSDEV_W<DEVCTL_SPEC, 5> {
        USB_DEVCTL_LSDEV_W::new(self)
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline(always)]
    #[must_use]
    pub fn usb_devctl_fsdev(&mut self) -> USB_DEVCTL_FSDEV_W<DEVCTL_SPEC, 6> {
        USB_DEVCTL_FSDEV_W::new(self)
    }
    #[doc = "Bit 7 - Device Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_devctl_dev(&mut self) -> USB_DEVCTL_DEV_W<DEVCTL_SPEC, 7> {
        USB_DEVCTL_DEV_W::new(self)
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
#[doc = "USB Device Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVCTL_SPEC;
impl crate::RegisterSpec for DEVCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devctl::R`](R) reader structure"]
impl crate::Readable for DEVCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devctl::W`](W) writer structure"]
impl crate::Writable for DEVCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVCTL to value 0"]
impl crate::Resettable for DEVCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
