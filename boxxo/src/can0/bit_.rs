#[doc = "Register `BIT` reader"]
pub type R = crate::R<BIT_SPEC>;
#[doc = "Register `BIT` writer"]
pub type W = crate::W<BIT_SPEC>;
#[doc = "Field `CAN_BIT_BRP` reader - Baud Rate Prescaler"]
pub type CAN_BIT_BRP_R = crate::FieldReader;
#[doc = "Field `CAN_BIT_BRP` writer - Baud Rate Prescaler"]
pub type CAN_BIT_BRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CAN_BIT_SJW` reader - (Re)Synchronization Jump Width"]
pub type CAN_BIT_SJW_R = crate::FieldReader;
#[doc = "Field `CAN_BIT_SJW` writer - (Re)Synchronization Jump Width"]
pub type CAN_BIT_SJW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CAN_BIT_TSEG1` reader - Time Segment Before Sample Point"]
pub type CAN_BIT_TSEG1_R = crate::FieldReader;
#[doc = "Field `CAN_BIT_TSEG1` writer - Time Segment Before Sample Point"]
pub type CAN_BIT_TSEG1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CAN_BIT_TSEG2` reader - Time Segment after Sample Point"]
pub type CAN_BIT_TSEG2_R = crate::FieldReader;
#[doc = "Field `CAN_BIT_TSEG2` writer - Time Segment after Sample Point"]
pub type CAN_BIT_TSEG2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn can_bit_brp(&self) -> CAN_BIT_BRP_R {
        CAN_BIT_BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn can_bit_sjw(&self) -> CAN_BIT_SJW_R {
        CAN_BIT_SJW_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg1(&self) -> CAN_BIT_TSEG1_R {
        CAN_BIT_TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment after Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg2(&self) -> CAN_BIT_TSEG2_R {
        CAN_BIT_TSEG2_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn can_bit_brp(&mut self) -> CAN_BIT_BRP_W<BIT_SPEC, 0> {
        CAN_BIT_BRP_W::new(self)
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn can_bit_sjw(&mut self) -> CAN_BIT_SJW_W<BIT_SPEC, 6> {
        CAN_BIT_SJW_W::new(self)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn can_bit_tseg1(&mut self) -> CAN_BIT_TSEG1_W<BIT_SPEC, 8> {
        CAN_BIT_TSEG1_W::new(self)
    }
    #[doc = "Bits 12:14 - Time Segment after Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn can_bit_tseg2(&mut self) -> CAN_BIT_TSEG2_W<BIT_SPEC, 12> {
        CAN_BIT_TSEG2_W::new(self)
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
#[doc = "CAN Bit Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bit_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bit_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIT_SPEC;
impl crate::RegisterSpec for BIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bit_::R`](R) reader structure"]
impl crate::Readable for BIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bit_::W`](W) writer structure"]
impl crate::Writable for BIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIT to value 0"]
impl crate::Resettable for BIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
