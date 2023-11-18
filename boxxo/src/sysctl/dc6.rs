#[doc = "Register `DC6` reader"]
pub type R = crate::R<DC6_SPEC>;
#[doc = "Register `DC6` writer"]
pub type W = crate::W<DC6_SPEC>;
#[doc = "Field `SYSCTL_DC6_USB0` reader - USB Module 0 Present"]
pub type SYSCTL_DC6_USB0_R = crate::FieldReader<SYSCTL_DC6_USB0_A>;
#[doc = "USB Module 0 Present\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DC6_USB0_A {
    #[doc = "1: USB0 is Device Only"]
    SYSCTL_DC6_USB0_DEV = 1,
    #[doc = "2: USB is Device or Host"]
    SYSCTL_DC6_USB0_HOSTDEV = 2,
    #[doc = "3: USB0 is OTG"]
    SYSCTL_DC6_USB0_OTG = 3,
}
impl From<SYSCTL_DC6_USB0_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DC6_USB0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DC6_USB0_A {
    type Ux = u8;
}
impl SYSCTL_DC6_USB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DC6_USB0_A> {
        match self.bits {
            1 => Some(SYSCTL_DC6_USB0_A::SYSCTL_DC6_USB0_DEV),
            2 => Some(SYSCTL_DC6_USB0_A::SYSCTL_DC6_USB0_HOSTDEV),
            3 => Some(SYSCTL_DC6_USB0_A::SYSCTL_DC6_USB0_OTG),
            _ => None,
        }
    }
    #[doc = "USB0 is Device Only"]
    #[inline(always)]
    pub fn is_sysctl_dc6_usb0_dev(&self) -> bool {
        *self == SYSCTL_DC6_USB0_A::SYSCTL_DC6_USB0_DEV
    }
    #[doc = "USB is Device or Host"]
    #[inline(always)]
    pub fn is_sysctl_dc6_usb0_hostdev(&self) -> bool {
        *self == SYSCTL_DC6_USB0_A::SYSCTL_DC6_USB0_HOSTDEV
    }
    #[doc = "USB0 is OTG"]
    #[inline(always)]
    pub fn is_sysctl_dc6_usb0_otg(&self) -> bool {
        *self == SYSCTL_DC6_USB0_A::SYSCTL_DC6_USB0_OTG
    }
}
#[doc = "Field `SYSCTL_DC6_USB0` writer - USB Module 0 Present"]
pub type SYSCTL_DC6_USB0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, SYSCTL_DC6_USB0_A>;
impl<'a, REG, const O: u8> SYSCTL_DC6_USB0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB0 is Device Only"]
    #[inline(always)]
    pub fn sysctl_dc6_usb0_dev(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC6_USB0_A::SYSCTL_DC6_USB0_DEV)
    }
    #[doc = "USB is Device or Host"]
    #[inline(always)]
    pub fn sysctl_dc6_usb0_hostdev(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC6_USB0_A::SYSCTL_DC6_USB0_HOSTDEV)
    }
    #[doc = "USB0 is OTG"]
    #[inline(always)]
    pub fn sysctl_dc6_usb0_otg(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC6_USB0_A::SYSCTL_DC6_USB0_OTG)
    }
}
#[doc = "Field `SYSCTL_DC6_USB0PHY` reader - USB Module 0 PHY Present"]
pub type SYSCTL_DC6_USB0PHY_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC6_USB0PHY` writer - USB Module 0 PHY Present"]
pub type SYSCTL_DC6_USB0PHY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - USB Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc6_usb0(&self) -> SYSCTL_DC6_USB0_R {
        SYSCTL_DC6_USB0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - USB Module 0 PHY Present"]
    #[inline(always)]
    pub fn sysctl_dc6_usb0phy(&self) -> SYSCTL_DC6_USB0PHY_R {
        SYSCTL_DC6_USB0PHY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc6_usb0(&mut self) -> SYSCTL_DC6_USB0_W<DC6_SPEC, 0> {
        SYSCTL_DC6_USB0_W::new(self)
    }
    #[doc = "Bit 4 - USB Module 0 PHY Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc6_usb0phy(&mut self) -> SYSCTL_DC6_USB0PHY_W<DC6_SPEC, 4> {
        SYSCTL_DC6_USB0PHY_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Capabilities 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC6_SPEC;
impl crate::RegisterSpec for DC6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc6::R`](R) reader structure"]
impl crate::Readable for DC6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc6::W`](W) writer structure"]
impl crate::Writable for DC6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC6 to value 0"]
impl crate::Resettable for DC6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
