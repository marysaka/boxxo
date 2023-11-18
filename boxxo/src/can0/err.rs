#[doc = "Register `ERR` reader"]
pub type R = crate::R<ERR_SPEC>;
#[doc = "Register `ERR` writer"]
pub type W = crate::W<ERR_SPEC>;
#[doc = "Field `CAN_ERR_TEC` reader - Transmit Error Counter"]
pub type CAN_ERR_TEC_R = crate::FieldReader;
#[doc = "Field `CAN_ERR_TEC` writer - Transmit Error Counter"]
pub type CAN_ERR_TEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CAN_ERR_REC` reader - Receive Error Counter"]
pub type CAN_ERR_REC_R = crate::FieldReader;
#[doc = "Field `CAN_ERR_REC` writer - Receive Error Counter"]
pub type CAN_ERR_REC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `CAN_ERR_RP` reader - Received Error Passive"]
pub type CAN_ERR_RP_R = crate::BitReader;
#[doc = "Field `CAN_ERR_RP` writer - Received Error Passive"]
pub type CAN_ERR_RP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn can_err_tec(&self) -> CAN_ERR_TEC_R {
        CAN_ERR_TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn can_err_rec(&self) -> CAN_ERR_REC_R {
        CAN_ERR_REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Received Error Passive"]
    #[inline(always)]
    pub fn can_err_rp(&self) -> CAN_ERR_RP_R {
        CAN_ERR_RP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn can_err_tec(&mut self) -> CAN_ERR_TEC_W<ERR_SPEC, 0> {
        CAN_ERR_TEC_W::new(self)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn can_err_rec(&mut self) -> CAN_ERR_REC_W<ERR_SPEC, 8> {
        CAN_ERR_REC_W::new(self)
    }
    #[doc = "Bit 15 - Received Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn can_err_rp(&mut self) -> CAN_ERR_RP_W<ERR_SPEC, 15> {
        CAN_ERR_RP_W::new(self)
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
#[doc = "CAN Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR_SPEC;
impl crate::RegisterSpec for ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err::R`](R) reader structure"]
impl crate::Readable for ERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`err::W`](W) writer structure"]
impl crate::Writable for ERR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
