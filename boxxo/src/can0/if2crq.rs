#[doc = "Register `IF2CRQ` reader"]
pub type R = crate::R<IF2CRQ_SPEC>;
#[doc = "Register `IF2CRQ` writer"]
pub type W = crate::W<IF2CRQ_SPEC>;
#[doc = "Field `CAN_IF2CRQ_MNUM` reader - Message Number"]
pub type CAN_IF2CRQ_MNUM_R = crate::FieldReader;
#[doc = "Field `CAN_IF2CRQ_MNUM` writer - Message Number"]
pub type CAN_IF2CRQ_MNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CAN_IF2CRQ_BUSY` reader - Busy Flag"]
pub type CAN_IF2CRQ_BUSY_R = crate::BitReader;
#[doc = "Field `CAN_IF2CRQ_BUSY` writer - Busy Flag"]
pub type CAN_IF2CRQ_BUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn can_if2crq_mnum(&self) -> CAN_IF2CRQ_MNUM_R {
        CAN_IF2CRQ_MNUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn can_if2crq_busy(&self) -> CAN_IF2CRQ_BUSY_R {
        CAN_IF2CRQ_BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2crq_mnum(&mut self) -> CAN_IF2CRQ_MNUM_W<IF2CRQ_SPEC, 0> {
        CAN_IF2CRQ_MNUM_W::new(self)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2crq_busy(&mut self) -> CAN_IF2CRQ_BUSY_W<IF2CRQ_SPEC, 15> {
        CAN_IF2CRQ_BUSY_W::new(self)
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
#[doc = "CAN IF2 Command Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2crq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2crq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF2CRQ_SPEC;
impl crate::RegisterSpec for IF2CRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if2crq::R`](R) reader structure"]
impl crate::Readable for IF2CRQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`if2crq::W`](W) writer structure"]
impl crate::Writable for IF2CRQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF2CRQ to value 0"]
impl crate::Resettable for IF2CRQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
