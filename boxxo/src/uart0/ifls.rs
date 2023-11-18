#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IFLS_SPEC>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IFLS_SPEC>;
#[doc = "Field `UART_IFLS_TX` reader - UART Transmit Interrupt FIFO Level Select"]
pub type UART_IFLS_TX_R = crate::FieldReader<UART_IFLS_TX_A>;
#[doc = "UART Transmit Interrupt FIFO Level Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART_IFLS_TX_A {
    #[doc = "0: TX FIFO &amp;lt;= 1/8 full"]
    UART_IFLS_TX1_8 = 0,
    #[doc = "1: TX FIFO &amp;lt;= 1/4 full"]
    UART_IFLS_TX2_8 = 1,
    #[doc = "2: TX FIFO &amp;lt;= 1/2 full (default)"]
    UART_IFLS_TX4_8 = 2,
    #[doc = "3: TX FIFO &amp;lt;= 3/4 full"]
    UART_IFLS_TX6_8 = 3,
    #[doc = "4: TX FIFO &amp;lt;= 7/8 full"]
    UART_IFLS_TX7_8 = 4,
}
impl From<UART_IFLS_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_IFLS_TX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UART_IFLS_TX_A {
    type Ux = u8;
}
impl UART_IFLS_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UART_IFLS_TX_A> {
        match self.bits {
            0 => Some(UART_IFLS_TX_A::UART_IFLS_TX1_8),
            1 => Some(UART_IFLS_TX_A::UART_IFLS_TX2_8),
            2 => Some(UART_IFLS_TX_A::UART_IFLS_TX4_8),
            3 => Some(UART_IFLS_TX_A::UART_IFLS_TX6_8),
            4 => Some(UART_IFLS_TX_A::UART_IFLS_TX7_8),
            _ => None,
        }
    }
    #[doc = "TX FIFO &amp;lt;= 1/8 full"]
    #[inline(always)]
    pub fn is_uart_ifls_tx1_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX1_8
    }
    #[doc = "TX FIFO &amp;lt;= 1/4 full"]
    #[inline(always)]
    pub fn is_uart_ifls_tx2_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX2_8
    }
    #[doc = "TX FIFO &amp;lt;= 1/2 full (default)"]
    #[inline(always)]
    pub fn is_uart_ifls_tx4_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX4_8
    }
    #[doc = "TX FIFO &amp;lt;= 3/4 full"]
    #[inline(always)]
    pub fn is_uart_ifls_tx6_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX6_8
    }
    #[doc = "TX FIFO &amp;lt;= 7/8 full"]
    #[inline(always)]
    pub fn is_uart_ifls_tx7_8(&self) -> bool {
        *self == UART_IFLS_TX_A::UART_IFLS_TX7_8
    }
}
#[doc = "Field `UART_IFLS_TX` writer - UART Transmit Interrupt FIFO Level Select"]
pub type UART_IFLS_TX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, UART_IFLS_TX_A>;
impl<'a, REG, const O: u8> UART_IFLS_TX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TX FIFO &amp;lt;= 1/8 full"]
    #[inline(always)]
    pub fn uart_ifls_tx1_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX1_8)
    }
    #[doc = "TX FIFO &amp;lt;= 1/4 full"]
    #[inline(always)]
    pub fn uart_ifls_tx2_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX2_8)
    }
    #[doc = "TX FIFO &amp;lt;= 1/2 full (default)"]
    #[inline(always)]
    pub fn uart_ifls_tx4_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX4_8)
    }
    #[doc = "TX FIFO &amp;lt;= 3/4 full"]
    #[inline(always)]
    pub fn uart_ifls_tx6_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX6_8)
    }
    #[doc = "TX FIFO &amp;lt;= 7/8 full"]
    #[inline(always)]
    pub fn uart_ifls_tx7_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_TX_A::UART_IFLS_TX7_8)
    }
}
#[doc = "Field `UART_IFLS_RX` reader - UART Receive Interrupt FIFO Level Select"]
pub type UART_IFLS_RX_R = crate::FieldReader<UART_IFLS_RX_A>;
#[doc = "UART Receive Interrupt FIFO Level Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART_IFLS_RX_A {
    #[doc = "0: RX FIFO >= 1/8 full"]
    UART_IFLS_RX1_8 = 0,
    #[doc = "1: RX FIFO >= 1/4 full"]
    UART_IFLS_RX2_8 = 1,
    #[doc = "2: RX FIFO >= 1/2 full (default)"]
    UART_IFLS_RX4_8 = 2,
    #[doc = "3: RX FIFO >= 3/4 full"]
    UART_IFLS_RX6_8 = 3,
    #[doc = "4: RX FIFO >= 7/8 full"]
    UART_IFLS_RX7_8 = 4,
}
impl From<UART_IFLS_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_IFLS_RX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UART_IFLS_RX_A {
    type Ux = u8;
}
impl UART_IFLS_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UART_IFLS_RX_A> {
        match self.bits {
            0 => Some(UART_IFLS_RX_A::UART_IFLS_RX1_8),
            1 => Some(UART_IFLS_RX_A::UART_IFLS_RX2_8),
            2 => Some(UART_IFLS_RX_A::UART_IFLS_RX4_8),
            3 => Some(UART_IFLS_RX_A::UART_IFLS_RX6_8),
            4 => Some(UART_IFLS_RX_A::UART_IFLS_RX7_8),
            _ => None,
        }
    }
    #[doc = "RX FIFO >= 1/8 full"]
    #[inline(always)]
    pub fn is_uart_ifls_rx1_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX1_8
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn is_uart_ifls_rx2_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX2_8
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn is_uart_ifls_rx4_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX4_8
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn is_uart_ifls_rx6_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX6_8
    }
    #[doc = "RX FIFO >= 7/8 full"]
    #[inline(always)]
    pub fn is_uart_ifls_rx7_8(&self) -> bool {
        *self == UART_IFLS_RX_A::UART_IFLS_RX7_8
    }
}
#[doc = "Field `UART_IFLS_RX` writer - UART Receive Interrupt FIFO Level Select"]
pub type UART_IFLS_RX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, UART_IFLS_RX_A>;
impl<'a, REG, const O: u8> UART_IFLS_RX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RX FIFO >= 1/8 full"]
    #[inline(always)]
    pub fn uart_ifls_rx1_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX1_8)
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn uart_ifls_rx2_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX2_8)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn uart_ifls_rx4_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX4_8)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn uart_ifls_rx6_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX6_8)
    }
    #[doc = "RX FIFO >= 7/8 full"]
    #[inline(always)]
    pub fn uart_ifls_rx7_8(self) -> &'a mut crate::W<REG> {
        self.variant(UART_IFLS_RX_A::UART_IFLS_RX7_8)
    }
}
impl R {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_tx(&self) -> UART_IFLS_TX_R {
        UART_IFLS_TX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_rx(&self) -> UART_IFLS_RX_R {
        UART_IFLS_RX_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ifls_tx(&mut self) -> UART_IFLS_TX_W<IFLS_SPEC, 0> {
        UART_IFLS_TX_W::new(self)
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ifls_rx(&mut self) -> UART_IFLS_RX_W<IFLS_SPEC, 3> {
        UART_IFLS_RX_W::new(self)
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
#[doc = "UART Interrupt FIFO Level Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFLS_SPEC;
impl crate::RegisterSpec for IFLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IFLS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IFLS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFLS to value 0"]
impl crate::Resettable for IFLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
