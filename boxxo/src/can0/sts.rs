#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `CAN_STS_LEC` reader - Last Error Code"]
pub type CAN_STS_LEC_R = crate::FieldReader<CAN_STS_LEC_A>;
#[doc = "Last Error Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAN_STS_LEC_A {
    #[doc = "0: No Error"]
    CAN_STS_LEC_NONE = 0,
    #[doc = "1: Stuff Error"]
    CAN_STS_LEC_STUFF = 1,
    #[doc = "2: Format Error"]
    CAN_STS_LEC_FORM = 2,
    #[doc = "3: ACK Error"]
    CAN_STS_LEC_ACK = 3,
    #[doc = "4: Bit 1 Error"]
    CAN_STS_LEC_BIT1 = 4,
    #[doc = "5: Bit 0 Error"]
    CAN_STS_LEC_BIT0 = 5,
    #[doc = "6: CRC Error"]
    CAN_STS_LEC_CRC = 6,
    #[doc = "7: No Event"]
    CAN_STS_LEC_NOEVENT = 7,
}
impl From<CAN_STS_LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: CAN_STS_LEC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAN_STS_LEC_A {
    type Ux = u8;
}
impl CAN_STS_LEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAN_STS_LEC_A {
        match self.bits {
            0 => CAN_STS_LEC_A::CAN_STS_LEC_NONE,
            1 => CAN_STS_LEC_A::CAN_STS_LEC_STUFF,
            2 => CAN_STS_LEC_A::CAN_STS_LEC_FORM,
            3 => CAN_STS_LEC_A::CAN_STS_LEC_ACK,
            4 => CAN_STS_LEC_A::CAN_STS_LEC_BIT1,
            5 => CAN_STS_LEC_A::CAN_STS_LEC_BIT0,
            6 => CAN_STS_LEC_A::CAN_STS_LEC_CRC,
            7 => CAN_STS_LEC_A::CAN_STS_LEC_NOEVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_can_sts_lec_none(&self) -> bool {
        *self == CAN_STS_LEC_A::CAN_STS_LEC_NONE
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn is_can_sts_lec_stuff(&self) -> bool {
        *self == CAN_STS_LEC_A::CAN_STS_LEC_STUFF
    }
    #[doc = "Format Error"]
    #[inline(always)]
    pub fn is_can_sts_lec_form(&self) -> bool {
        *self == CAN_STS_LEC_A::CAN_STS_LEC_FORM
    }
    #[doc = "ACK Error"]
    #[inline(always)]
    pub fn is_can_sts_lec_ack(&self) -> bool {
        *self == CAN_STS_LEC_A::CAN_STS_LEC_ACK
    }
    #[doc = "Bit 1 Error"]
    #[inline(always)]
    pub fn is_can_sts_lec_bit1(&self) -> bool {
        *self == CAN_STS_LEC_A::CAN_STS_LEC_BIT1
    }
    #[doc = "Bit 0 Error"]
    #[inline(always)]
    pub fn is_can_sts_lec_bit0(&self) -> bool {
        *self == CAN_STS_LEC_A::CAN_STS_LEC_BIT0
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn is_can_sts_lec_crc(&self) -> bool {
        *self == CAN_STS_LEC_A::CAN_STS_LEC_CRC
    }
    #[doc = "No Event"]
    #[inline(always)]
    pub fn is_can_sts_lec_noevent(&self) -> bool {
        *self == CAN_STS_LEC_A::CAN_STS_LEC_NOEVENT
    }
}
#[doc = "Field `CAN_STS_LEC` writer - Last Error Code"]
pub type CAN_STS_LEC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CAN_STS_LEC_A>;
impl<'a, REG, const O: u8> CAN_STS_LEC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn can_sts_lec_none(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_STS_LEC_A::CAN_STS_LEC_NONE)
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn can_sts_lec_stuff(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_STS_LEC_A::CAN_STS_LEC_STUFF)
    }
    #[doc = "Format Error"]
    #[inline(always)]
    pub fn can_sts_lec_form(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_STS_LEC_A::CAN_STS_LEC_FORM)
    }
    #[doc = "ACK Error"]
    #[inline(always)]
    pub fn can_sts_lec_ack(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_STS_LEC_A::CAN_STS_LEC_ACK)
    }
    #[doc = "Bit 1 Error"]
    #[inline(always)]
    pub fn can_sts_lec_bit1(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_STS_LEC_A::CAN_STS_LEC_BIT1)
    }
    #[doc = "Bit 0 Error"]
    #[inline(always)]
    pub fn can_sts_lec_bit0(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_STS_LEC_A::CAN_STS_LEC_BIT0)
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn can_sts_lec_crc(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_STS_LEC_A::CAN_STS_LEC_CRC)
    }
    #[doc = "No Event"]
    #[inline(always)]
    pub fn can_sts_lec_noevent(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_STS_LEC_A::CAN_STS_LEC_NOEVENT)
    }
}
#[doc = "Field `CAN_STS_TXOK` reader - Transmitted a Message Successfully"]
pub type CAN_STS_TXOK_R = crate::BitReader;
#[doc = "Field `CAN_STS_TXOK` writer - Transmitted a Message Successfully"]
pub type CAN_STS_TXOK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_STS_RXOK` reader - Received a Message Successfully"]
pub type CAN_STS_RXOK_R = crate::BitReader;
#[doc = "Field `CAN_STS_RXOK` writer - Received a Message Successfully"]
pub type CAN_STS_RXOK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_STS_EPASS` reader - Error Passive"]
pub type CAN_STS_EPASS_R = crate::BitReader;
#[doc = "Field `CAN_STS_EPASS` writer - Error Passive"]
pub type CAN_STS_EPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_STS_EWARN` reader - Warning Status"]
pub type CAN_STS_EWARN_R = crate::BitReader;
#[doc = "Field `CAN_STS_EWARN` writer - Warning Status"]
pub type CAN_STS_EWARN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_STS_BOFF` reader - Bus-Off Status"]
pub type CAN_STS_BOFF_R = crate::BitReader;
#[doc = "Field `CAN_STS_BOFF` writer - Bus-Off Status"]
pub type CAN_STS_BOFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn can_sts_lec(&self) -> CAN_STS_LEC_R {
        CAN_STS_LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn can_sts_txok(&self) -> CAN_STS_TXOK_R {
        CAN_STS_TXOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn can_sts_rxok(&self) -> CAN_STS_RXOK_R {
        CAN_STS_RXOK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn can_sts_epass(&self) -> CAN_STS_EPASS_R {
        CAN_STS_EPASS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn can_sts_ewarn(&self) -> CAN_STS_EWARN_R {
        CAN_STS_EWARN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus-Off Status"]
    #[inline(always)]
    pub fn can_sts_boff(&self) -> CAN_STS_BOFF_R {
        CAN_STS_BOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn can_sts_lec(&mut self) -> CAN_STS_LEC_W<STS_SPEC, 0> {
        CAN_STS_LEC_W::new(self)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn can_sts_txok(&mut self) -> CAN_STS_TXOK_W<STS_SPEC, 3> {
        CAN_STS_TXOK_W::new(self)
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn can_sts_rxok(&mut self) -> CAN_STS_RXOK_W<STS_SPEC, 4> {
        CAN_STS_RXOK_W::new(self)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn can_sts_epass(&mut self) -> CAN_STS_EPASS_W<STS_SPEC, 5> {
        CAN_STS_EPASS_W::new(self)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    #[must_use]
    pub fn can_sts_ewarn(&mut self) -> CAN_STS_EWARN_W<STS_SPEC, 6> {
        CAN_STS_EWARN_W::new(self)
    }
    #[doc = "Bit 7 - Bus-Off Status"]
    #[inline(always)]
    #[must_use]
    pub fn can_sts_boff(&mut self) -> CAN_STS_BOFF_W<STS_SPEC, 7> {
        CAN_STS_BOFF_W::new(self)
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
#[doc = "CAN Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
