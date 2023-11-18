#[doc = "Register `INT` reader"]
pub type R = crate::R<INT_SPEC>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<INT_SPEC>;
#[doc = "Field `CAN_INT_INTID` reader - Interrupt Identifier"]
pub type CAN_INT_INTID_R = crate::FieldReader<CAN_INT_INTID_A>;
#[doc = "Interrupt Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CAN_INT_INTID_A {
    #[doc = "0: No interrupt pending"]
    CAN_INT_INTID_NONE = 0,
    #[doc = "32768: Status Interrupt"]
    CAN_INT_INTID_STATUS = 32768,
}
impl From<CAN_INT_INTID_A> for u16 {
    #[inline(always)]
    fn from(variant: CAN_INT_INTID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAN_INT_INTID_A {
    type Ux = u16;
}
impl CAN_INT_INTID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAN_INT_INTID_A> {
        match self.bits {
            0 => Some(CAN_INT_INTID_A::CAN_INT_INTID_NONE),
            32768 => Some(CAN_INT_INTID_A::CAN_INT_INTID_STATUS),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_can_int_intid_none(&self) -> bool {
        *self == CAN_INT_INTID_A::CAN_INT_INTID_NONE
    }
    #[doc = "Status Interrupt"]
    #[inline(always)]
    pub fn is_can_int_intid_status(&self) -> bool {
        *self == CAN_INT_INTID_A::CAN_INT_INTID_STATUS
    }
}
#[doc = "Field `CAN_INT_INTID` writer - Interrupt Identifier"]
pub type CAN_INT_INTID_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, CAN_INT_INTID_A>;
impl<'a, REG, const O: u8> CAN_INT_INTID_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn can_int_intid_none(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_INT_INTID_A::CAN_INT_INTID_NONE)
    }
    #[doc = "Status Interrupt"]
    #[inline(always)]
    pub fn can_int_intid_status(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_INT_INTID_A::CAN_INT_INTID_STATUS)
    }
}
impl R {
    #[doc = "Bits 0:15 - Interrupt Identifier"]
    #[inline(always)]
    pub fn can_int_intid(&self) -> CAN_INT_INTID_R {
        CAN_INT_INTID_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn can_int_intid(&mut self) -> CAN_INT_INTID_W<INT_SPEC, 0> {
        CAN_INT_INTID_W::new(self)
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
#[doc = "CAN Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
