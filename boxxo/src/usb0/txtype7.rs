#[doc = "Register `TXTYPE7` reader"]
pub type R = crate::R<TXTYPE7_SPEC>;
#[doc = "Register `TXTYPE7` writer"]
pub type W = crate::W<TXTYPE7_SPEC>;
#[doc = "Field `USB_TXTYPE7_TEP` reader - Target Endpoint Number"]
pub type USB_TXTYPE7_TEP_R = crate::FieldReader;
#[doc = "Field `USB_TXTYPE7_TEP` writer - Target Endpoint Number"]
pub type USB_TXTYPE7_TEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USB_TXTYPE7_PROTO` reader - Protocol"]
pub type USB_TXTYPE7_PROTO_R = crate::FieldReader<USB_TXTYPE7_PROTO_A>;
#[doc = "Protocol\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_TXTYPE7_PROTO_A {
    #[doc = "0: Control"]
    USB_TXTYPE7_PROTO_CTRL = 0,
    #[doc = "1: Isochronous"]
    USB_TXTYPE7_PROTO_ISOC = 1,
    #[doc = "2: Bulk"]
    USB_TXTYPE7_PROTO_BULK = 2,
    #[doc = "3: Interrupt"]
    USB_TXTYPE7_PROTO_INT = 3,
}
impl From<USB_TXTYPE7_PROTO_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_TXTYPE7_PROTO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USB_TXTYPE7_PROTO_A {
    type Ux = u8;
}
impl USB_TXTYPE7_PROTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USB_TXTYPE7_PROTO_A {
        match self.bits {
            0 => USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_CTRL,
            1 => USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_ISOC,
            2 => USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_BULK,
            3 => USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_usb_txtype7_proto_ctrl(&self) -> bool {
        *self == USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_CTRL
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_usb_txtype7_proto_isoc(&self) -> bool {
        *self == USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_ISOC
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_usb_txtype7_proto_bulk(&self) -> bool {
        *self == USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_BULK
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_usb_txtype7_proto_int(&self) -> bool {
        *self == USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_INT
    }
}
#[doc = "Field `USB_TXTYPE7_PROTO` writer - Protocol"]
pub type USB_TXTYPE7_PROTO_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, USB_TXTYPE7_PROTO_A>;
impl<'a, REG, const O: u8> USB_TXTYPE7_PROTO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn usb_txtype7_proto_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_CTRL)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn usb_txtype7_proto_isoc(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_ISOC)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn usb_txtype7_proto_bulk(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_BULK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn usb_txtype7_proto_int(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXTYPE7_PROTO_A::USB_TXTYPE7_PROTO_INT)
    }
}
#[doc = "Field `USB_TXTYPE7_SPEED` reader - Operating Speed"]
pub type USB_TXTYPE7_SPEED_R = crate::FieldReader<USB_TXTYPE7_SPEED_A>;
#[doc = "Operating Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_TXTYPE7_SPEED_A {
    #[doc = "0: Default"]
    USB_TXTYPE7_SPEED_DFLT = 0,
    #[doc = "2: Full"]
    USB_TXTYPE7_SPEED_FULL = 2,
    #[doc = "3: Low"]
    USB_TXTYPE7_SPEED_LOW = 3,
}
impl From<USB_TXTYPE7_SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_TXTYPE7_SPEED_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USB_TXTYPE7_SPEED_A {
    type Ux = u8;
}
impl USB_TXTYPE7_SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USB_TXTYPE7_SPEED_A> {
        match self.bits {
            0 => Some(USB_TXTYPE7_SPEED_A::USB_TXTYPE7_SPEED_DFLT),
            2 => Some(USB_TXTYPE7_SPEED_A::USB_TXTYPE7_SPEED_FULL),
            3 => Some(USB_TXTYPE7_SPEED_A::USB_TXTYPE7_SPEED_LOW),
            _ => None,
        }
    }
    #[doc = "Default"]
    #[inline(always)]
    pub fn is_usb_txtype7_speed_dflt(&self) -> bool {
        *self == USB_TXTYPE7_SPEED_A::USB_TXTYPE7_SPEED_DFLT
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn is_usb_txtype7_speed_full(&self) -> bool {
        *self == USB_TXTYPE7_SPEED_A::USB_TXTYPE7_SPEED_FULL
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_usb_txtype7_speed_low(&self) -> bool {
        *self == USB_TXTYPE7_SPEED_A::USB_TXTYPE7_SPEED_LOW
    }
}
#[doc = "Field `USB_TXTYPE7_SPEED` writer - Operating Speed"]
pub type USB_TXTYPE7_SPEED_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, USB_TXTYPE7_SPEED_A>;
impl<'a, REG, const O: u8> USB_TXTYPE7_SPEED_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default"]
    #[inline(always)]
    pub fn usb_txtype7_speed_dflt(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXTYPE7_SPEED_A::USB_TXTYPE7_SPEED_DFLT)
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn usb_txtype7_speed_full(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXTYPE7_SPEED_A::USB_TXTYPE7_SPEED_FULL)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn usb_txtype7_speed_low(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXTYPE7_SPEED_A::USB_TXTYPE7_SPEED_LOW)
    }
}
impl R {
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline(always)]
    pub fn usb_txtype7_tep(&self) -> USB_TXTYPE7_TEP_R {
        USB_TXTYPE7_TEP_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    pub fn usb_txtype7_proto(&self) -> USB_TXTYPE7_PROTO_R {
        USB_TXTYPE7_PROTO_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_txtype7_speed(&self) -> USB_TXTYPE7_SPEED_R {
        USB_TXTYPE7_SPEED_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txtype7_tep(&mut self) -> USB_TXTYPE7_TEP_W<TXTYPE7_SPEC, 0> {
        USB_TXTYPE7_TEP_W::new(self)
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txtype7_proto(&mut self) -> USB_TXTYPE7_PROTO_W<TXTYPE7_SPEC, 4> {
        USB_TXTYPE7_PROTO_W::new(self)
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txtype7_speed(&mut self) -> USB_TXTYPE7_SPEED_W<TXTYPE7_SPEC, 6> {
        USB_TXTYPE7_SPEED_W::new(self)
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
#[doc = "USB Host Transmit Configure Type Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtype7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtype7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXTYPE7_SPEC;
impl crate::RegisterSpec for TXTYPE7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txtype7::R`](R) reader structure"]
impl crate::Readable for TXTYPE7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txtype7::W`](W) writer structure"]
impl crate::Writable for TXTYPE7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXTYPE7 to value 0"]
impl crate::Resettable for TXTYPE7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
