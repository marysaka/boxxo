#[doc = "Register `ACCTL1` reader"]
pub type R = crate::R<ACCTL1_SPEC>;
#[doc = "Register `ACCTL1` writer"]
pub type W = crate::W<ACCTL1_SPEC>;
#[doc = "Field `COMP_ACCTL1_CINV` reader - Comparator Output Invert"]
pub type COMP_ACCTL1_CINV_R = crate::BitReader;
#[doc = "Field `COMP_ACCTL1_CINV` writer - Comparator Output Invert"]
pub type COMP_ACCTL1_CINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP_ACCTL1_ISEN` reader - Interrupt Sense"]
pub type COMP_ACCTL1_ISEN_R = crate::FieldReader<COMP_ACCTL1_ISEN_A>;
#[doc = "Interrupt Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP_ACCTL1_ISEN_A {
    #[doc = "0: Level sense, see ISLVAL"]
    COMP_ACCTL1_ISEN_LEVEL = 0,
    #[doc = "1: Falling edge"]
    COMP_ACCTL1_ISEN_FALL = 1,
    #[doc = "2: Rising edge"]
    COMP_ACCTL1_ISEN_RISE = 2,
    #[doc = "3: Either edge"]
    COMP_ACCTL1_ISEN_BOTH = 3,
}
impl From<COMP_ACCTL1_ISEN_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_ACCTL1_ISEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP_ACCTL1_ISEN_A {
    type Ux = u8;
}
impl COMP_ACCTL1_ISEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP_ACCTL1_ISEN_A {
        match self.bits {
            0 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_LEVEL,
            1 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_FALL,
            2 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_RISE,
            3 => COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Level sense, see ISLVAL"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_level(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_LEVEL
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_fall(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_FALL
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_rise(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_RISE
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn is_comp_acctl1_isen_both(&self) -> bool {
        *self == COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_BOTH
    }
}
#[doc = "Field `COMP_ACCTL1_ISEN` writer - Interrupt Sense"]
pub type COMP_ACCTL1_ISEN_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, COMP_ACCTL1_ISEN_A>;
impl<'a, REG, const O: u8> COMP_ACCTL1_ISEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Level sense, see ISLVAL"]
    #[inline(always)]
    pub fn comp_acctl1_isen_level(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn comp_acctl1_isen_fall(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn comp_acctl1_isen_rise(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn comp_acctl1_isen_both(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_ISEN_A::COMP_ACCTL1_ISEN_BOTH)
    }
}
#[doc = "Field `COMP_ACCTL1_ISLVAL` reader - Interrupt Sense Level Value"]
pub type COMP_ACCTL1_ISLVAL_R = crate::BitReader;
#[doc = "Field `COMP_ACCTL1_ISLVAL` writer - Interrupt Sense Level Value"]
pub type COMP_ACCTL1_ISLVAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP_ACCTL1_TSEN` reader - Trigger Sense"]
pub type COMP_ACCTL1_TSEN_R = crate::FieldReader<COMP_ACCTL1_TSEN_A>;
#[doc = "Trigger Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP_ACCTL1_TSEN_A {
    #[doc = "0: Level sense, see TSLVAL"]
    COMP_ACCTL1_TSEN_LEVEL = 0,
    #[doc = "1: Falling edge"]
    COMP_ACCTL1_TSEN_FALL = 1,
    #[doc = "2: Rising edge"]
    COMP_ACCTL1_TSEN_RISE = 2,
    #[doc = "3: Either edge"]
    COMP_ACCTL1_TSEN_BOTH = 3,
}
impl From<COMP_ACCTL1_TSEN_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_ACCTL1_TSEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP_ACCTL1_TSEN_A {
    type Ux = u8;
}
impl COMP_ACCTL1_TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP_ACCTL1_TSEN_A {
        match self.bits {
            0 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_LEVEL,
            1 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_FALL,
            2 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_RISE,
            3 => COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Level sense, see TSLVAL"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_level(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_LEVEL
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_fall(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_FALL
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_rise(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_RISE
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn is_comp_acctl1_tsen_both(&self) -> bool {
        *self == COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_BOTH
    }
}
#[doc = "Field `COMP_ACCTL1_TSEN` writer - Trigger Sense"]
pub type COMP_ACCTL1_TSEN_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, COMP_ACCTL1_TSEN_A>;
impl<'a, REG, const O: u8> COMP_ACCTL1_TSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Level sense, see TSLVAL"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_level(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_fall(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_rise(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn comp_acctl1_tsen_both(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_TSEN_A::COMP_ACCTL1_TSEN_BOTH)
    }
}
#[doc = "Field `COMP_ACCTL1_TSLVAL` reader - Trigger Sense Level Value"]
pub type COMP_ACCTL1_TSLVAL_R = crate::BitReader;
#[doc = "Field `COMP_ACCTL1_TSLVAL` writer - Trigger Sense Level Value"]
pub type COMP_ACCTL1_TSLVAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP_ACCTL1_ASRCP` reader - Analog Source Positive"]
pub type COMP_ACCTL1_ASRCP_R = crate::FieldReader<COMP_ACCTL1_ASRCP_A>;
#[doc = "Analog Source Positive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP_ACCTL1_ASRCP_A {
    #[doc = "0: Pin value of Cn+"]
    COMP_ACCTL1_ASRCP_PIN = 0,
    #[doc = "1: Pin value of C0+"]
    COMP_ACCTL1_ASRCP_PIN0 = 1,
    #[doc = "2: Internal voltage reference (VIREF)"]
    COMP_ACCTL1_ASRCP_REF = 2,
}
impl From<COMP_ACCTL1_ASRCP_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_ACCTL1_ASRCP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP_ACCTL1_ASRCP_A {
    type Ux = u8;
}
impl COMP_ACCTL1_ASRCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP_ACCTL1_ASRCP_A> {
        match self.bits {
            0 => Some(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN),
            1 => Some(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN0),
            2 => Some(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_REF),
            _ => None,
        }
    }
    #[doc = "Pin value of Cn+"]
    #[inline(always)]
    pub fn is_comp_acctl1_asrcp_pin(&self) -> bool {
        *self == COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN
    }
    #[doc = "Pin value of C0+"]
    #[inline(always)]
    pub fn is_comp_acctl1_asrcp_pin0(&self) -> bool {
        *self == COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN0
    }
    #[doc = "Internal voltage reference (VIREF)"]
    #[inline(always)]
    pub fn is_comp_acctl1_asrcp_ref(&self) -> bool {
        *self == COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_REF
    }
}
#[doc = "Field `COMP_ACCTL1_ASRCP` writer - Analog Source Positive"]
pub type COMP_ACCTL1_ASRCP_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, COMP_ACCTL1_ASRCP_A>;
impl<'a, REG, const O: u8> COMP_ACCTL1_ASRCP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin value of Cn+"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp_pin(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN)
    }
    #[doc = "Pin value of C0+"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp_pin0(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_PIN0)
    }
    #[doc = "Internal voltage reference (VIREF)"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp_ref(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_ACCTL1_ASRCP_A::COMP_ACCTL1_ASRCP_REF)
    }
}
#[doc = "Field `COMP_ACCTL1_TOEN` reader - Trigger Output Enable"]
pub type COMP_ACCTL1_TOEN_R = crate::BitReader;
#[doc = "Field `COMP_ACCTL1_TOEN` writer - Trigger Output Enable"]
pub type COMP_ACCTL1_TOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    pub fn comp_acctl1_cinv(&self) -> COMP_ACCTL1_CINV_R {
        COMP_ACCTL1_CINV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    pub fn comp_acctl1_isen(&self) -> COMP_ACCTL1_ISEN_R {
        COMP_ACCTL1_ISEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_islval(&self) -> COMP_ACCTL1_ISLVAL_R {
        COMP_ACCTL1_ISLVAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    pub fn comp_acctl1_tsen(&self) -> COMP_ACCTL1_TSEN_R {
        COMP_ACCTL1_TSEN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl1_tslval(&self) -> COMP_ACCTL1_TSLVAL_R {
        COMP_ACCTL1_TSLVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    pub fn comp_acctl1_asrcp(&self) -> COMP_ACCTL1_ASRCP_R {
        COMP_ACCTL1_ASRCP_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    pub fn comp_acctl1_toen(&self) -> COMP_ACCTL1_TOEN_R {
        COMP_ACCTL1_TOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acctl1_cinv(&mut self) -> COMP_ACCTL1_CINV_W<ACCTL1_SPEC, 1> {
        COMP_ACCTL1_CINV_W::new(self)
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acctl1_isen(&mut self) -> COMP_ACCTL1_ISEN_W<ACCTL1_SPEC, 2> {
        COMP_ACCTL1_ISEN_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acctl1_islval(&mut self) -> COMP_ACCTL1_ISLVAL_W<ACCTL1_SPEC, 4> {
        COMP_ACCTL1_ISLVAL_W::new(self)
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acctl1_tsen(&mut self) -> COMP_ACCTL1_TSEN_W<ACCTL1_SPEC, 5> {
        COMP_ACCTL1_TSEN_W::new(self)
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acctl1_tslval(&mut self) -> COMP_ACCTL1_TSLVAL_W<ACCTL1_SPEC, 7> {
        COMP_ACCTL1_TSLVAL_W::new(self)
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acctl1_asrcp(&mut self) -> COMP_ACCTL1_ASRCP_W<ACCTL1_SPEC, 9> {
        COMP_ACCTL1_ASRCP_W::new(self)
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acctl1_toen(&mut self) -> COMP_ACCTL1_TOEN_W<ACCTL1_SPEC, 11> {
        COMP_ACCTL1_TOEN_W::new(self)
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
#[doc = "Analog Comparator Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACCTL1_SPEC;
impl crate::RegisterSpec for ACCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acctl1::R`](R) reader structure"]
impl crate::Readable for ACCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acctl1::W`](W) writer structure"]
impl crate::Writable for ACCTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACCTL1 to value 0"]
impl crate::Resettable for ACCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
