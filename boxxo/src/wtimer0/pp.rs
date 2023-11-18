#[doc = "Register `PP` reader"]
pub type R = crate::R<PP_SPEC>;
#[doc = "Register `PP` writer"]
pub type W = crate::W<PP_SPEC>;
#[doc = "Field `TIMER_PP_SIZE` reader - Count Size"]
pub type TIMER_PP_SIZE_R = crate::FieldReader<TIMER_PP_SIZE_A>;
#[doc = "Count Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_PP_SIZE_A {
    #[doc = "0: Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter"]
    TIMER_PP_SIZE_16 = 0,
    #[doc = "1: Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter"]
    TIMER_PP_SIZE_32 = 1,
}
impl From<TIMER_PP_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_PP_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_PP_SIZE_A {
    type Ux = u8;
}
impl TIMER_PP_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMER_PP_SIZE_A> {
        match self.bits {
            0 => Some(TIMER_PP_SIZE_A::TIMER_PP_SIZE_16),
            1 => Some(TIMER_PP_SIZE_A::TIMER_PP_SIZE_32),
            _ => None,
        }
    }
    #[doc = "Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter"]
    #[inline(always)]
    pub fn is_timer_pp_size_16(&self) -> bool {
        *self == TIMER_PP_SIZE_A::TIMER_PP_SIZE_16
    }
    #[doc = "Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter"]
    #[inline(always)]
    pub fn is_timer_pp_size_32(&self) -> bool {
        *self == TIMER_PP_SIZE_A::TIMER_PP_SIZE_32
    }
}
#[doc = "Field `TIMER_PP_SIZE` writer - Count Size"]
pub type TIMER_PP_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, TIMER_PP_SIZE_A>;
impl<'a, REG, const O: u8> TIMER_PP_SIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter"]
    #[inline(always)]
    pub fn timer_pp_size_16(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_PP_SIZE_A::TIMER_PP_SIZE_16)
    }
    #[doc = "Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter"]
    #[inline(always)]
    pub fn timer_pp_size_32(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_PP_SIZE_A::TIMER_PP_SIZE_32)
    }
}
impl R {
    #[doc = "Bits 0:3 - Count Size"]
    #[inline(always)]
    pub fn timer_pp_size(&self) -> TIMER_PP_SIZE_R {
        TIMER_PP_SIZE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Count Size"]
    #[inline(always)]
    #[must_use]
    pub fn timer_pp_size(&mut self) -> TIMER_PP_SIZE_W<PP_SPEC, 0> {
        TIMER_PP_SIZE_W::new(self)
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
#[doc = "GPTM Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp::R`](R) reader structure"]
impl crate::Readable for PP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pp::W`](W) writer structure"]
impl crate::Writable for PP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
