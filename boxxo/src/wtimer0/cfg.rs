#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `TIMER_CFG` reader - GPTM Configuration"]
pub type TIMER_CFG_R = crate::FieldReader<TIMER_CFG_A>;
#[doc = "GPTM Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_CFG_A {
    #[doc = "0: 32-bit timer configuration"]
    TIMER_CFG_32_BIT_TIMER = 0,
    #[doc = "1: 32-bit real-time clock (RTC) counter configuration"]
    TIMER_CFG_32_BIT_RTC = 1,
    #[doc = "4: 16-bit timer configuration. The function is controlled by bits 1:0 of GPTMTAMR and GPTMTBMR"]
    TIMER_CFG_16_BIT = 4,
}
impl From<TIMER_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_CFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_CFG_A {
    type Ux = u8;
}
impl TIMER_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMER_CFG_A> {
        match self.bits {
            0 => Some(TIMER_CFG_A::TIMER_CFG_32_BIT_TIMER),
            1 => Some(TIMER_CFG_A::TIMER_CFG_32_BIT_RTC),
            4 => Some(TIMER_CFG_A::TIMER_CFG_16_BIT),
            _ => None,
        }
    }
    #[doc = "32-bit timer configuration"]
    #[inline(always)]
    pub fn is_timer_cfg_32_bit_timer(&self) -> bool {
        *self == TIMER_CFG_A::TIMER_CFG_32_BIT_TIMER
    }
    #[doc = "32-bit real-time clock (RTC) counter configuration"]
    #[inline(always)]
    pub fn is_timer_cfg_32_bit_rtc(&self) -> bool {
        *self == TIMER_CFG_A::TIMER_CFG_32_BIT_RTC
    }
    #[doc = "16-bit timer configuration. The function is controlled by bits 1:0 of GPTMTAMR and GPTMTBMR"]
    #[inline(always)]
    pub fn is_timer_cfg_16_bit(&self) -> bool {
        *self == TIMER_CFG_A::TIMER_CFG_16_BIT
    }
}
#[doc = "Field `TIMER_CFG` writer - GPTM Configuration"]
pub type TIMER_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TIMER_CFG_A>;
impl<'a, REG, const O: u8> TIMER_CFG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit timer configuration"]
    #[inline(always)]
    pub fn timer_cfg_32_bit_timer(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_CFG_A::TIMER_CFG_32_BIT_TIMER)
    }
    #[doc = "32-bit real-time clock (RTC) counter configuration"]
    #[inline(always)]
    pub fn timer_cfg_32_bit_rtc(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_CFG_A::TIMER_CFG_32_BIT_RTC)
    }
    #[doc = "16-bit timer configuration. The function is controlled by bits 1:0 of GPTMTAMR and GPTMTBMR"]
    #[inline(always)]
    pub fn timer_cfg_16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_CFG_A::TIMER_CFG_16_BIT)
    }
}
impl R {
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    pub fn timer_cfg(&self) -> TIMER_CFG_R {
        TIMER_CFG_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn timer_cfg(&mut self) -> TIMER_CFG_W<CFG_SPEC, 0> {
        TIMER_CFG_W::new(self)
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
#[doc = "GPTM Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
