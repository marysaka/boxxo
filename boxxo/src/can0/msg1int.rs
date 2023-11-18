#[doc = "Register `MSG1INT` reader"]
pub type R = crate::R<MSG1INT_SPEC>;
#[doc = "Register `MSG1INT` writer"]
pub type W = crate::W<MSG1INT_SPEC>;
#[doc = "Field `CAN_MSG1INT_INTPND` reader - Interrupt Pending Bits"]
pub type CAN_MSG1INT_INTPND_R = crate::FieldReader<u16>;
#[doc = "Field `CAN_MSG1INT_INTPND` writer - Interrupt Pending Bits"]
pub type CAN_MSG1INT_INTPND_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt Pending Bits"]
    #[inline(always)]
    pub fn can_msg1int_intpnd(&self) -> CAN_MSG1INT_INTPND_R {
        CAN_MSG1INT_INTPND_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt Pending Bits"]
    #[inline(always)]
    #[must_use]
    pub fn can_msg1int_intpnd(&mut self) -> CAN_MSG1INT_INTPND_W<MSG1INT_SPEC, 0> {
        CAN_MSG1INT_INTPND_W::new(self)
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
#[doc = "CAN Message 1 Interrupt Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg1int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg1int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSG1INT_SPEC;
impl crate::RegisterSpec for MSG1INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msg1int::R`](R) reader structure"]
impl crate::Readable for MSG1INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msg1int::W`](W) writer structure"]
impl crate::Writable for MSG1INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSG1INT to value 0"]
impl crate::Resettable for MSG1INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
