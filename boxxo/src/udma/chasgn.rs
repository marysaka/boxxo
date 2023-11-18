#[doc = "Register `CHASGN` reader"]
pub type R = crate::R<CHASGN_SPEC>;
#[doc = "Register `CHASGN` writer"]
pub type W = crate::W<CHASGN_SPEC>;
#[doc = "Field `UDMA_CHASGN` reader - Channel \\[n\\]
Assignment Select"]
pub type UDMA_CHASGN_R = crate::FieldReader<UDMA_CHASGN_A>;
#[doc = "Channel \\[n\\]
Assignment Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum UDMA_CHASGN_A {
    #[doc = "0: Use the primary channel assignment"]
    UDMA_CHASGN_PRIMARY = 0,
    #[doc = "1: Use the secondary channel assignment"]
    UDMA_CHASGN_SECONDARY = 1,
}
impl From<UDMA_CHASGN_A> for u32 {
    #[inline(always)]
    fn from(variant: UDMA_CHASGN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UDMA_CHASGN_A {
    type Ux = u32;
}
impl UDMA_CHASGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UDMA_CHASGN_A> {
        match self.bits {
            0 => Some(UDMA_CHASGN_A::UDMA_CHASGN_PRIMARY),
            1 => Some(UDMA_CHASGN_A::UDMA_CHASGN_SECONDARY),
            _ => None,
        }
    }
    #[doc = "Use the primary channel assignment"]
    #[inline(always)]
    pub fn is_udma_chasgn_primary(&self) -> bool {
        *self == UDMA_CHASGN_A::UDMA_CHASGN_PRIMARY
    }
    #[doc = "Use the secondary channel assignment"]
    #[inline(always)]
    pub fn is_udma_chasgn_secondary(&self) -> bool {
        *self == UDMA_CHASGN_A::UDMA_CHASGN_SECONDARY
    }
}
#[doc = "Field `UDMA_CHASGN` writer - Channel \\[n\\]
Assignment Select"]
pub type UDMA_CHASGN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, UDMA_CHASGN_A>;
impl<'a, REG, const O: u8> UDMA_CHASGN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Use the primary channel assignment"]
    #[inline(always)]
    pub fn udma_chasgn_primary(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_CHASGN_A::UDMA_CHASGN_PRIMARY)
    }
    #[doc = "Use the secondary channel assignment"]
    #[inline(always)]
    pub fn udma_chasgn_secondary(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_CHASGN_A::UDMA_CHASGN_SECONDARY)
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Assignment Select"]
    #[inline(always)]
    pub fn udma_chasgn(&self) -> UDMA_CHASGN_R {
        UDMA_CHASGN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Assignment Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chasgn(&mut self) -> UDMA_CHASGN_W<CHASGN_SPEC, 0> {
        UDMA_CHASGN_W::new(self)
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
#[doc = "DMA Channel Assignment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chasgn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chasgn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHASGN_SPEC;
impl crate::RegisterSpec for CHASGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chasgn::R`](R) reader structure"]
impl crate::Readable for CHASGN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chasgn::W`](W) writer structure"]
impl crate::Writable for CHASGN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHASGN to value 0"]
impl crate::Resettable for CHASGN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
