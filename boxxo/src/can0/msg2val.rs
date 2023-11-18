#[doc = "Register `MSG2VAL` reader"]
pub type R = crate::R<MSG2VAL_SPEC>;
#[doc = "Register `MSG2VAL` writer"]
pub type W = crate::W<MSG2VAL_SPEC>;
#[doc = "Field `CAN_MSG2VAL_MSGVAL` reader - Message Valid Bits"]
pub type CAN_MSG2VAL_MSGVAL_R = crate::FieldReader<u16>;
#[doc = "Field `CAN_MSG2VAL_MSGVAL` writer - Message Valid Bits"]
pub type CAN_MSG2VAL_MSGVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Message Valid Bits"]
    #[inline(always)]
    pub fn can_msg2val_msgval(&self) -> CAN_MSG2VAL_MSGVAL_R {
        CAN_MSG2VAL_MSGVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Message Valid Bits"]
    #[inline(always)]
    #[must_use]
    pub fn can_msg2val_msgval(&mut self) -> CAN_MSG2VAL_MSGVAL_W<MSG2VAL_SPEC, 0> {
        CAN_MSG2VAL_MSGVAL_W::new(self)
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
#[doc = "CAN Message 2 Valid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg2val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg2val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSG2VAL_SPEC;
impl crate::RegisterSpec for MSG2VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msg2val::R`](R) reader structure"]
impl crate::Readable for MSG2VAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msg2val::W`](W) writer structure"]
impl crate::Writable for MSG2VAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSG2VAL to value 0"]
impl crate::Resettable for MSG2VAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
