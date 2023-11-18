#[doc = "Register `IF2ARB2` reader"]
pub type R = crate::R<IF2ARB2_SPEC>;
#[doc = "Register `IF2ARB2` writer"]
pub type W = crate::W<IF2ARB2_SPEC>;
#[doc = "Field `CAN_IF2ARB2_ID` reader - Message Identifier"]
pub type CAN_IF2ARB2_ID_R = crate::FieldReader<u16>;
#[doc = "Field `CAN_IF2ARB2_ID` writer - Message Identifier"]
pub type CAN_IF2ARB2_ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
#[doc = "Field `CAN_IF2ARB2_DIR` reader - Message Direction"]
pub type CAN_IF2ARB2_DIR_R = crate::BitReader;
#[doc = "Field `CAN_IF2ARB2_DIR` writer - Message Direction"]
pub type CAN_IF2ARB2_DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF2ARB2_XTD` reader - Extended Identifier"]
pub type CAN_IF2ARB2_XTD_R = crate::BitReader;
#[doc = "Field `CAN_IF2ARB2_XTD` writer - Extended Identifier"]
pub type CAN_IF2ARB2_XTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF2ARB2_MSGVAL` reader - Message Valid"]
pub type CAN_IF2ARB2_MSGVAL_R = crate::BitReader;
#[doc = "Field `CAN_IF2ARB2_MSGVAL` writer - Message Valid"]
pub type CAN_IF2ARB2_MSGVAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:12 - Message Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_id(&self) -> CAN_IF2ARB2_ID_R {
        CAN_IF2ARB2_ID_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Message Direction"]
    #[inline(always)]
    pub fn can_if2arb2_dir(&self) -> CAN_IF2ARB2_DIR_R {
        CAN_IF2ARB2_DIR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Extended Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_xtd(&self) -> CAN_IF2ARB2_XTD_R {
        CAN_IF2ARB2_XTD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Message Valid"]
    #[inline(always)]
    pub fn can_if2arb2_msgval(&self) -> CAN_IF2ARB2_MSGVAL_R {
        CAN_IF2ARB2_MSGVAL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Message Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2arb2_id(&mut self) -> CAN_IF2ARB2_ID_W<IF2ARB2_SPEC, 0> {
        CAN_IF2ARB2_ID_W::new(self)
    }
    #[doc = "Bit 13 - Message Direction"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2arb2_dir(&mut self) -> CAN_IF2ARB2_DIR_W<IF2ARB2_SPEC, 13> {
        CAN_IF2ARB2_DIR_W::new(self)
    }
    #[doc = "Bit 14 - Extended Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2arb2_xtd(&mut self) -> CAN_IF2ARB2_XTD_W<IF2ARB2_SPEC, 14> {
        CAN_IF2ARB2_XTD_W::new(self)
    }
    #[doc = "Bit 15 - Message Valid"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2arb2_msgval(&mut self) -> CAN_IF2ARB2_MSGVAL_W<IF2ARB2_SPEC, 15> {
        CAN_IF2ARB2_MSGVAL_W::new(self)
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
#[doc = "CAN IF2 Arbitration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2arb2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2arb2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF2ARB2_SPEC;
impl crate::RegisterSpec for IF2ARB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if2arb2::R`](R) reader structure"]
impl crate::Readable for IF2ARB2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`if2arb2::W`](W) writer structure"]
impl crate::Writable for IF2ARB2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF2ARB2 to value 0"]
impl crate::Resettable for IF2ARB2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
