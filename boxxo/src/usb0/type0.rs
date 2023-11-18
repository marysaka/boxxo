#[doc = "Register `TYPE0` reader"]
pub type R = crate::R<TYPE0_SPEC>;
#[doc = "Register `TYPE0` writer"]
pub type W = crate::W<TYPE0_SPEC>;
#[doc = "Field `USB_TYPE0_SPEED` reader - Operating Speed"]
pub type USB_TYPE0_SPEED_R = crate::FieldReader<USB_TYPE0_SPEED_A>;
#[doc = "Operating Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_TYPE0_SPEED_A {
    #[doc = "2: Full"]
    USB_TYPE0_SPEED_FULL = 2,
    #[doc = "3: Low"]
    USB_TYPE0_SPEED_LOW = 3,
}
impl From<USB_TYPE0_SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_TYPE0_SPEED_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USB_TYPE0_SPEED_A {
    type Ux = u8;
}
impl USB_TYPE0_SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USB_TYPE0_SPEED_A> {
        match self.bits {
            2 => Some(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_FULL),
            3 => Some(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_LOW),
            _ => None,
        }
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn is_usb_type0_speed_full(&self) -> bool {
        *self == USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_FULL
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_usb_type0_speed_low(&self) -> bool {
        *self == USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_LOW
    }
}
#[doc = "Field `USB_TYPE0_SPEED` writer - Operating Speed"]
pub type USB_TYPE0_SPEED_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, USB_TYPE0_SPEED_A>;
impl<'a, REG, const O: u8> USB_TYPE0_SPEED_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Full"]
    #[inline(always)]
    pub fn usb_type0_speed_full(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_FULL)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn usb_type0_speed_low(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_LOW)
    }
}
impl R {
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_type0_speed(&self) -> USB_TYPE0_SPEED_R {
        USB_TYPE0_SPEED_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    #[must_use]
    pub fn usb_type0_speed(&mut self) -> USB_TYPE0_SPEED_W<TYPE0_SPEC, 6> {
        USB_TYPE0_SPEED_W::new(self)
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
#[doc = "USB Type Endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`type0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TYPE0_SPEC;
impl crate::RegisterSpec for TYPE0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`type0::R`](R) reader structure"]
impl crate::Readable for TYPE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`type0::W`](W) writer structure"]
impl crate::Writable for TYPE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TYPE0 to value 0"]
impl crate::Resettable for TYPE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
