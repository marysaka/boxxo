#[doc = "Register `LCRH` reader"]
pub type R = crate::R<LCRH_SPEC>;
#[doc = "Register `LCRH` writer"]
pub type W = crate::W<LCRH_SPEC>;
#[doc = "Field `UART_LCRH_BRK` reader - UART Send Break"]
pub type UART_LCRH_BRK_R = crate::BitReader;
#[doc = "Field `UART_LCRH_BRK` writer - UART Send Break"]
pub type UART_LCRH_BRK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_LCRH_PEN` reader - UART Parity Enable"]
pub type UART_LCRH_PEN_R = crate::BitReader;
#[doc = "Field `UART_LCRH_PEN` writer - UART Parity Enable"]
pub type UART_LCRH_PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_LCRH_EPS` reader - UART Even Parity Select"]
pub type UART_LCRH_EPS_R = crate::BitReader;
#[doc = "Field `UART_LCRH_EPS` writer - UART Even Parity Select"]
pub type UART_LCRH_EPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_LCRH_STP2` reader - UART Two Stop Bits Select"]
pub type UART_LCRH_STP2_R = crate::BitReader;
#[doc = "Field `UART_LCRH_STP2` writer - UART Two Stop Bits Select"]
pub type UART_LCRH_STP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_LCRH_FEN` reader - UART Enable FIFOs"]
pub type UART_LCRH_FEN_R = crate::BitReader;
#[doc = "Field `UART_LCRH_FEN` writer - UART Enable FIFOs"]
pub type UART_LCRH_FEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_LCRH_WLEN` reader - UART Word Length"]
pub type UART_LCRH_WLEN_R = crate::FieldReader<UART_LCRH_WLEN_A>;
#[doc = "UART Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART_LCRH_WLEN_A {
    #[doc = "0: 5 bits (default)"]
    UART_LCRH_WLEN_5 = 0,
    #[doc = "1: 6 bits"]
    UART_LCRH_WLEN_6 = 1,
    #[doc = "2: 7 bits"]
    UART_LCRH_WLEN_7 = 2,
    #[doc = "3: 8 bits"]
    UART_LCRH_WLEN_8 = 3,
}
impl From<UART_LCRH_WLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_LCRH_WLEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UART_LCRH_WLEN_A {
    type Ux = u8;
}
impl UART_LCRH_WLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UART_LCRH_WLEN_A {
        match self.bits {
            0 => UART_LCRH_WLEN_A::UART_LCRH_WLEN_5,
            1 => UART_LCRH_WLEN_A::UART_LCRH_WLEN_6,
            2 => UART_LCRH_WLEN_A::UART_LCRH_WLEN_7,
            3 => UART_LCRH_WLEN_A::UART_LCRH_WLEN_8,
            _ => unreachable!(),
        }
    }
    #[doc = "5 bits (default)"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_5(&self) -> bool {
        *self == UART_LCRH_WLEN_A::UART_LCRH_WLEN_5
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_6(&self) -> bool {
        *self == UART_LCRH_WLEN_A::UART_LCRH_WLEN_6
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_7(&self) -> bool {
        *self == UART_LCRH_WLEN_A::UART_LCRH_WLEN_7
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_uart_lcrh_wlen_8(&self) -> bool {
        *self == UART_LCRH_WLEN_A::UART_LCRH_WLEN_8
    }
}
#[doc = "Field `UART_LCRH_WLEN` writer - UART Word Length"]
pub type UART_LCRH_WLEN_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, UART_LCRH_WLEN_A>;
impl<'a, REG, const O: u8> UART_LCRH_WLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 bits (default)"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_5(self) -> &'a mut crate::W<REG> {
        self.variant(UART_LCRH_WLEN_A::UART_LCRH_WLEN_5)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_6(self) -> &'a mut crate::W<REG> {
        self.variant(UART_LCRH_WLEN_A::UART_LCRH_WLEN_6)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_7(self) -> &'a mut crate::W<REG> {
        self.variant(UART_LCRH_WLEN_A::UART_LCRH_WLEN_7)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn uart_lcrh_wlen_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_LCRH_WLEN_A::UART_LCRH_WLEN_8)
    }
}
#[doc = "Field `UART_LCRH_SPS` reader - UART Stick Parity Select"]
pub type UART_LCRH_SPS_R = crate::BitReader;
#[doc = "Field `UART_LCRH_SPS` writer - UART Stick Parity Select"]
pub type UART_LCRH_SPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UART Send Break"]
    #[inline(always)]
    pub fn uart_lcrh_brk(&self) -> UART_LCRH_BRK_R {
        UART_LCRH_BRK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    pub fn uart_lcrh_pen(&self) -> UART_LCRH_PEN_R {
        UART_LCRH_PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Even Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_eps(&self) -> UART_LCRH_EPS_R {
        UART_LCRH_EPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select"]
    #[inline(always)]
    pub fn uart_lcrh_stp2(&self) -> UART_LCRH_STP2_R {
        UART_LCRH_STP2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Enable FIFOs"]
    #[inline(always)]
    pub fn uart_lcrh_fen(&self) -> UART_LCRH_FEN_R {
        UART_LCRH_FEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - UART Word Length"]
    #[inline(always)]
    pub fn uart_lcrh_wlen(&self) -> UART_LCRH_WLEN_R {
        UART_LCRH_WLEN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - UART Stick Parity Select"]
    #[inline(always)]
    pub fn uart_lcrh_sps(&self) -> UART_LCRH_SPS_R {
        UART_LCRH_SPS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Send Break"]
    #[inline(always)]
    #[must_use]
    pub fn uart_lcrh_brk(&mut self) -> UART_LCRH_BRK_W<LCRH_SPEC, 0> {
        UART_LCRH_BRK_W::new(self)
    }
    #[doc = "Bit 1 - UART Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart_lcrh_pen(&mut self) -> UART_LCRH_PEN_W<LCRH_SPEC, 1> {
        UART_LCRH_PEN_W::new(self)
    }
    #[doc = "Bit 2 - UART Even Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn uart_lcrh_eps(&mut self) -> UART_LCRH_EPS_W<LCRH_SPEC, 2> {
        UART_LCRH_EPS_W::new(self)
    }
    #[doc = "Bit 3 - UART Two Stop Bits Select"]
    #[inline(always)]
    #[must_use]
    pub fn uart_lcrh_stp2(&mut self) -> UART_LCRH_STP2_W<LCRH_SPEC, 3> {
        UART_LCRH_STP2_W::new(self)
    }
    #[doc = "Bit 4 - UART Enable FIFOs"]
    #[inline(always)]
    #[must_use]
    pub fn uart_lcrh_fen(&mut self) -> UART_LCRH_FEN_W<LCRH_SPEC, 4> {
        UART_LCRH_FEN_W::new(self)
    }
    #[doc = "Bits 5:6 - UART Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn uart_lcrh_wlen(&mut self) -> UART_LCRH_WLEN_W<LCRH_SPEC, 5> {
        UART_LCRH_WLEN_W::new(self)
    }
    #[doc = "Bit 7 - UART Stick Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn uart_lcrh_sps(&mut self) -> UART_LCRH_SPS_W<LCRH_SPEC, 7> {
        UART_LCRH_SPS_W::new(self)
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
#[doc = "UART Line Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCRH_SPEC;
impl crate::RegisterSpec for LCRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcrh::R`](R) reader structure"]
impl crate::Readable for LCRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcrh::W`](W) writer structure"]
impl crate::Writable for LCRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCRH to value 0"]
impl crate::Resettable for LCRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
