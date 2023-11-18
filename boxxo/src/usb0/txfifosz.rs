#[doc = "Register `TXFIFOSZ` reader"]
pub type R = crate::R<TXFIFOSZ_SPEC>;
#[doc = "Register `TXFIFOSZ` writer"]
pub type W = crate::W<TXFIFOSZ_SPEC>;
#[doc = "Field `USB_TXFIFOSZ_SIZE` reader - Max Packet Size"]
pub type USB_TXFIFOSZ_SIZE_R = crate::FieldReader<USB_TXFIFOSZ_SIZE_A>;
#[doc = "Max Packet Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_TXFIFOSZ_SIZE_A {
    #[doc = "0: 8"]
    USB_TXFIFOSZ_SIZE_8 = 0,
    #[doc = "1: 16"]
    USB_TXFIFOSZ_SIZE_16 = 1,
    #[doc = "2: 32"]
    USB_TXFIFOSZ_SIZE_32 = 2,
    #[doc = "3: 64"]
    USB_TXFIFOSZ_SIZE_64 = 3,
    #[doc = "4: 128"]
    USB_TXFIFOSZ_SIZE_128 = 4,
    #[doc = "5: 256"]
    USB_TXFIFOSZ_SIZE_256 = 5,
    #[doc = "6: 512"]
    USB_TXFIFOSZ_SIZE_512 = 6,
    #[doc = "7: 1024"]
    USB_TXFIFOSZ_SIZE_1024 = 7,
    #[doc = "8: 2048"]
    USB_TXFIFOSZ_SIZE_2048 = 8,
}
impl From<USB_TXFIFOSZ_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_TXFIFOSZ_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USB_TXFIFOSZ_SIZE_A {
    type Ux = u8;
}
impl USB_TXFIFOSZ_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USB_TXFIFOSZ_SIZE_A> {
        match self.bits {
            0 => Some(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_8),
            1 => Some(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_16),
            2 => Some(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_32),
            3 => Some(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_64),
            4 => Some(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_128),
            5 => Some(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_256),
            6 => Some(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_512),
            7 => Some(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_1024),
            8 => Some(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_2048),
            _ => None,
        }
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_usb_txfifosz_size_8(&self) -> bool {
        *self == USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_8
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_usb_txfifosz_size_16(&self) -> bool {
        *self == USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_16
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_usb_txfifosz_size_32(&self) -> bool {
        *self == USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_32
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn is_usb_txfifosz_size_64(&self) -> bool {
        *self == USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_64
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_usb_txfifosz_size_128(&self) -> bool {
        *self == USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_128
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_usb_txfifosz_size_256(&self) -> bool {
        *self == USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_256
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn is_usb_txfifosz_size_512(&self) -> bool {
        *self == USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_512
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn is_usb_txfifosz_size_1024(&self) -> bool {
        *self == USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_1024
    }
    #[doc = "2048"]
    #[inline(always)]
    pub fn is_usb_txfifosz_size_2048(&self) -> bool {
        *self == USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_2048
    }
}
#[doc = "Field `USB_TXFIFOSZ_SIZE` writer - Max Packet Size"]
pub type USB_TXFIFOSZ_SIZE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 4, O, USB_TXFIFOSZ_SIZE_A>;
impl<'a, REG, const O: u8> USB_TXFIFOSZ_SIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8"]
    #[inline(always)]
    pub fn usb_txfifosz_size_8(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_8)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn usb_txfifosz_size_16(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_16)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn usb_txfifosz_size_32(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_32)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn usb_txfifosz_size_64(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_64)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn usb_txfifosz_size_128(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_128)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn usb_txfifosz_size_256(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_256)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn usb_txfifosz_size_512(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_512)
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn usb_txfifosz_size_1024(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_1024)
    }
    #[doc = "2048"]
    #[inline(always)]
    pub fn usb_txfifosz_size_2048(self) -> &'a mut crate::W<REG> {
        self.variant(USB_TXFIFOSZ_SIZE_A::USB_TXFIFOSZ_SIZE_2048)
    }
}
#[doc = "Field `USB_TXFIFOSZ_DPB` reader - Double Packet Buffer Support"]
pub type USB_TXFIFOSZ_DPB_R = crate::BitReader;
#[doc = "Field `USB_TXFIFOSZ_DPB` writer - Double Packet Buffer Support"]
pub type USB_TXFIFOSZ_DPB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Max Packet Size"]
    #[inline(always)]
    pub fn usb_txfifosz_size(&self) -> USB_TXFIFOSZ_SIZE_R {
        USB_TXFIFOSZ_SIZE_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Double Packet Buffer Support"]
    #[inline(always)]
    pub fn usb_txfifosz_dpb(&self) -> USB_TXFIFOSZ_DPB_R {
        USB_TXFIFOSZ_DPB_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Max Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txfifosz_size(&mut self) -> USB_TXFIFOSZ_SIZE_W<TXFIFOSZ_SPEC, 0> {
        USB_TXFIFOSZ_SIZE_W::new(self)
    }
    #[doc = "Bit 4 - Double Packet Buffer Support"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txfifosz_dpb(&mut self) -> USB_TXFIFOSZ_DPB_W<TXFIFOSZ_SPEC, 4> {
        USB_TXFIFOSZ_DPB_W::new(self)
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
#[doc = "USB Transmit Dynamic FIFO Sizing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfifosz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifosz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFIFOSZ_SPEC;
impl crate::RegisterSpec for TXFIFOSZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txfifosz::R`](R) reader structure"]
impl crate::Readable for TXFIFOSZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txfifosz::W`](W) writer structure"]
impl crate::Writable for TXFIFOSZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFIFOSZ to value 0"]
impl crate::Resettable for TXFIFOSZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
