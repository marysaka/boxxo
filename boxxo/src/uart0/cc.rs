#[doc = "Register `CC` reader"]
pub type R = crate::R<CC_SPEC>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CC_SPEC>;
#[doc = "Field `UART_CC_CS` reader - UART Baud Clock Source"]
pub type UART_CC_CS_R = crate::FieldReader<UART_CC_CS_A>;
#[doc = "UART Baud Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART_CC_CS_A {
    #[doc = "0: The system clock (default)"]
    UART_CC_CS_SYSCLK = 0,
    #[doc = "5: PIOSC"]
    UART_CC_CS_PIOSC = 5,
}
impl From<UART_CC_CS_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_CC_CS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UART_CC_CS_A {
    type Ux = u8;
}
impl UART_CC_CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UART_CC_CS_A> {
        match self.bits {
            0 => Some(UART_CC_CS_A::UART_CC_CS_SYSCLK),
            5 => Some(UART_CC_CS_A::UART_CC_CS_PIOSC),
            _ => None,
        }
    }
    #[doc = "The system clock (default)"]
    #[inline(always)]
    pub fn is_uart_cc_cs_sysclk(&self) -> bool {
        *self == UART_CC_CS_A::UART_CC_CS_SYSCLK
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn is_uart_cc_cs_piosc(&self) -> bool {
        *self == UART_CC_CS_A::UART_CC_CS_PIOSC
    }
}
#[doc = "Field `UART_CC_CS` writer - UART Baud Clock Source"]
pub type UART_CC_CS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, UART_CC_CS_A>;
impl<'a, REG, const O: u8> UART_CC_CS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The system clock (default)"]
    #[inline(always)]
    pub fn uart_cc_cs_sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(UART_CC_CS_A::UART_CC_CS_SYSCLK)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn uart_cc_cs_piosc(self) -> &'a mut crate::W<REG> {
        self.variant(UART_CC_CS_A::UART_CC_CS_PIOSC)
    }
}
impl R {
    #[doc = "Bits 0:3 - UART Baud Clock Source"]
    #[inline(always)]
    pub fn uart_cc_cs(&self) -> UART_CC_CS_R {
        UART_CC_CS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - UART Baud Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn uart_cc_cs(&mut self) -> UART_CC_CS_W<CC_SPEC, 0> {
        UART_CC_CS_W::new(self)
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
#[doc = "UART Clock Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
