#[doc = "Register `PP` reader"]
pub type R = crate::R<PP_SPEC>;
#[doc = "Register `PP` writer"]
pub type W = crate::W<PP_SPEC>;
#[doc = "Field `USB_PP_TYPE` reader - Controller Type"]
pub type USB_PP_TYPE_R = crate::FieldReader<USB_PP_TYPE_A>;
#[doc = "Controller Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_PP_TYPE_A {
    #[doc = "0: The first-generation USB controller"]
    USB_PP_TYPE_0 = 0,
}
impl From<USB_PP_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_PP_TYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USB_PP_TYPE_A {
    type Ux = u8;
}
impl USB_PP_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USB_PP_TYPE_A> {
        match self.bits {
            0 => Some(USB_PP_TYPE_A::USB_PP_TYPE_0),
            _ => None,
        }
    }
    #[doc = "The first-generation USB controller"]
    #[inline(always)]
    pub fn is_usb_pp_type_0(&self) -> bool {
        *self == USB_PP_TYPE_A::USB_PP_TYPE_0
    }
}
#[doc = "Field `USB_PP_TYPE` writer - Controller Type"]
pub type USB_PP_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, USB_PP_TYPE_A>;
impl<'a, REG, const O: u8> USB_PP_TYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The first-generation USB controller"]
    #[inline(always)]
    pub fn usb_pp_type_0(self) -> &'a mut crate::W<REG> {
        self.variant(USB_PP_TYPE_A::USB_PP_TYPE_0)
    }
}
#[doc = "Field `USB_PP_PHY` reader - PHY Present"]
pub type USB_PP_PHY_R = crate::BitReader;
#[doc = "Field `USB_PP_PHY` writer - PHY Present"]
pub type USB_PP_PHY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_PP_USB` reader - USB Capability"]
pub type USB_PP_USB_R = crate::FieldReader<USB_PP_USB_A>;
#[doc = "USB Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_PP_USB_A {
    #[doc = "1: DEVICE"]
    USB_PP_USB_DEVICE = 1,
    #[doc = "2: HOST"]
    USB_PP_USB_HOSTDEVICE = 2,
    #[doc = "3: OTG"]
    USB_PP_USB_OTG = 3,
}
impl From<USB_PP_USB_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_PP_USB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USB_PP_USB_A {
    type Ux = u8;
}
impl USB_PP_USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USB_PP_USB_A> {
        match self.bits {
            1 => Some(USB_PP_USB_A::USB_PP_USB_DEVICE),
            2 => Some(USB_PP_USB_A::USB_PP_USB_HOSTDEVICE),
            3 => Some(USB_PP_USB_A::USB_PP_USB_OTG),
            _ => None,
        }
    }
    #[doc = "DEVICE"]
    #[inline(always)]
    pub fn is_usb_pp_usb_device(&self) -> bool {
        *self == USB_PP_USB_A::USB_PP_USB_DEVICE
    }
    #[doc = "HOST"]
    #[inline(always)]
    pub fn is_usb_pp_usb_hostdevice(&self) -> bool {
        *self == USB_PP_USB_A::USB_PP_USB_HOSTDEVICE
    }
    #[doc = "OTG"]
    #[inline(always)]
    pub fn is_usb_pp_usb_otg(&self) -> bool {
        *self == USB_PP_USB_A::USB_PP_USB_OTG
    }
}
#[doc = "Field `USB_PP_USB` writer - USB Capability"]
pub type USB_PP_USB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, USB_PP_USB_A>;
impl<'a, REG, const O: u8> USB_PP_USB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DEVICE"]
    #[inline(always)]
    pub fn usb_pp_usb_device(self) -> &'a mut crate::W<REG> {
        self.variant(USB_PP_USB_A::USB_PP_USB_DEVICE)
    }
    #[doc = "HOST"]
    #[inline(always)]
    pub fn usb_pp_usb_hostdevice(self) -> &'a mut crate::W<REG> {
        self.variant(USB_PP_USB_A::USB_PP_USB_HOSTDEVICE)
    }
    #[doc = "OTG"]
    #[inline(always)]
    pub fn usb_pp_usb_otg(self) -> &'a mut crate::W<REG> {
        self.variant(USB_PP_USB_A::USB_PP_USB_OTG)
    }
}
#[doc = "Field `USB_PP_ECNT` reader - Endpoint Count"]
pub type USB_PP_ECNT_R = crate::FieldReader;
#[doc = "Field `USB_PP_ECNT` writer - Endpoint Count"]
pub type USB_PP_ECNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:3 - Controller Type"]
    #[inline(always)]
    pub fn usb_pp_type(&self) -> USB_PP_TYPE_R {
        USB_PP_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PHY Present"]
    #[inline(always)]
    pub fn usb_pp_phy(&self) -> USB_PP_PHY_R {
        USB_PP_PHY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - USB Capability"]
    #[inline(always)]
    pub fn usb_pp_usb(&self) -> USB_PP_USB_R {
        USB_PP_USB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Endpoint Count"]
    #[inline(always)]
    pub fn usb_pp_ecnt(&self) -> USB_PP_ECNT_R {
        USB_PP_ECNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Controller Type"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pp_type(&mut self) -> USB_PP_TYPE_W<PP_SPEC, 0> {
        USB_PP_TYPE_W::new(self)
    }
    #[doc = "Bit 4 - PHY Present"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pp_phy(&mut self) -> USB_PP_PHY_W<PP_SPEC, 4> {
        USB_PP_PHY_W::new(self)
    }
    #[doc = "Bits 6:7 - USB Capability"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pp_usb(&mut self) -> USB_PP_USB_W<PP_SPEC, 6> {
        USB_PP_USB_W::new(self)
    }
    #[doc = "Bits 8:15 - Endpoint Count"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pp_ecnt(&mut self) -> USB_PP_ECNT_W<PP_SPEC, 8> {
        USB_PP_ECNT_W::new(self)
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
#[doc = "USB Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp::R`](R) reader structure"]
impl crate::Readable for PP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pp::W`](W) writer structure"]
impl crate::Writable for PP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
