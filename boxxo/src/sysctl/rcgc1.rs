#[doc = "Register `RCGC1` reader"]
pub type R = crate::R<RCGC1_SPEC>;
#[doc = "Register `RCGC1` writer"]
pub type W = crate::W<RCGC1_SPEC>;
#[doc = "Field `SYSCTL_RCGC1_UART0` reader - UART0 Clock Gating Control"]
pub type SYSCTL_RCGC1_UART0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_UART0` writer - UART0 Clock Gating Control"]
pub type SYSCTL_RCGC1_UART0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_UART1` reader - UART1 Clock Gating Control"]
pub type SYSCTL_RCGC1_UART1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_UART1` writer - UART1 Clock Gating Control"]
pub type SYSCTL_RCGC1_UART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_UART2` reader - UART2 Clock Gating Control"]
pub type SYSCTL_RCGC1_UART2_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_UART2` writer - UART2 Clock Gating Control"]
pub type SYSCTL_RCGC1_UART2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_SSI0` reader - SSI0 Clock Gating Control"]
pub type SYSCTL_RCGC1_SSI0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_SSI0` writer - SSI0 Clock Gating Control"]
pub type SYSCTL_RCGC1_SSI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_SSI1` reader - SSI1 Clock Gating Control"]
pub type SYSCTL_RCGC1_SSI1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_SSI1` writer - SSI1 Clock Gating Control"]
pub type SYSCTL_RCGC1_SSI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_QEI0` reader - QEI0 Clock Gating Control"]
pub type SYSCTL_RCGC1_QEI0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_QEI0` writer - QEI0 Clock Gating Control"]
pub type SYSCTL_RCGC1_QEI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_QEI1` reader - QEI1 Clock Gating Control"]
pub type SYSCTL_RCGC1_QEI1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_QEI1` writer - QEI1 Clock Gating Control"]
pub type SYSCTL_RCGC1_QEI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_I2C0` reader - I2C0 Clock Gating Control"]
pub type SYSCTL_RCGC1_I2C0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_I2C0` writer - I2C0 Clock Gating Control"]
pub type SYSCTL_RCGC1_I2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_I2C1` reader - I2C1 Clock Gating Control"]
pub type SYSCTL_RCGC1_I2C1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_I2C1` writer - I2C1 Clock Gating Control"]
pub type SYSCTL_RCGC1_I2C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_TIMER0` reader - Timer 0 Clock Gating Control"]
pub type SYSCTL_RCGC1_TIMER0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_TIMER0` writer - Timer 0 Clock Gating Control"]
pub type SYSCTL_RCGC1_TIMER0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_TIMER1` reader - Timer 1 Clock Gating Control"]
pub type SYSCTL_RCGC1_TIMER1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_TIMER1` writer - Timer 1 Clock Gating Control"]
pub type SYSCTL_RCGC1_TIMER1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_TIMER2` reader - Timer 2 Clock Gating Control"]
pub type SYSCTL_RCGC1_TIMER2_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_TIMER2` writer - Timer 2 Clock Gating Control"]
pub type SYSCTL_RCGC1_TIMER2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_TIMER3` reader - Timer 3 Clock Gating Control"]
pub type SYSCTL_RCGC1_TIMER3_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_TIMER3` writer - Timer 3 Clock Gating Control"]
pub type SYSCTL_RCGC1_TIMER3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_COMP0` reader - Analog Comparator 0 Clock Gating"]
pub type SYSCTL_RCGC1_COMP0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_COMP0` writer - Analog Comparator 0 Clock Gating"]
pub type SYSCTL_RCGC1_COMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC1_COMP1` reader - Analog Comparator 1 Clock Gating"]
pub type SYSCTL_RCGC1_COMP1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC1_COMP1` writer - Analog Comparator 1 Clock Gating"]
pub type SYSCTL_RCGC1_COMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UART0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_uart0(&self) -> SYSCTL_RCGC1_UART0_R {
        SYSCTL_RCGC1_UART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_uart1(&self) -> SYSCTL_RCGC1_UART1_R {
        SYSCTL_RCGC1_UART1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART2 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_uart2(&self) -> SYSCTL_RCGC1_UART2_R {
        SYSCTL_RCGC1_UART2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SSI0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_ssi0(&self) -> SYSCTL_RCGC1_SSI0_R {
        SYSCTL_RCGC1_SSI0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SSI1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_ssi1(&self) -> SYSCTL_RCGC1_SSI1_R {
        SYSCTL_RCGC1_SSI1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - QEI0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_qei0(&self) -> SYSCTL_RCGC1_QEI0_R {
        SYSCTL_RCGC1_QEI0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - QEI1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_qei1(&self) -> SYSCTL_RCGC1_QEI1_R {
        SYSCTL_RCGC1_QEI1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_i2c0(&self) -> SYSCTL_RCGC1_I2C0_R {
        SYSCTL_RCGC1_I2C0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_i2c1(&self) -> SYSCTL_RCGC1_I2C1_R {
        SYSCTL_RCGC1_I2C1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer 0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_timer0(&self) -> SYSCTL_RCGC1_TIMER0_R {
        SYSCTL_RCGC1_TIMER0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer 1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_timer1(&self) -> SYSCTL_RCGC1_TIMER1_R {
        SYSCTL_RCGC1_TIMER1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 2 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_timer2(&self) -> SYSCTL_RCGC1_TIMER2_R {
        SYSCTL_RCGC1_TIMER2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer 3 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc1_timer3(&self) -> SYSCTL_RCGC1_TIMER3_R {
        SYSCTL_RCGC1_TIMER3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog Comparator 0 Clock Gating"]
    #[inline(always)]
    pub fn sysctl_rcgc1_comp0(&self) -> SYSCTL_RCGC1_COMP0_R {
        SYSCTL_RCGC1_COMP0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog Comparator 1 Clock Gating"]
    #[inline(always)]
    pub fn sysctl_rcgc1_comp1(&self) -> SYSCTL_RCGC1_COMP1_R {
        SYSCTL_RCGC1_COMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_uart0(&mut self) -> SYSCTL_RCGC1_UART0_W<RCGC1_SPEC, 0> {
        SYSCTL_RCGC1_UART0_W::new(self)
    }
    #[doc = "Bit 1 - UART1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_uart1(&mut self) -> SYSCTL_RCGC1_UART1_W<RCGC1_SPEC, 1> {
        SYSCTL_RCGC1_UART1_W::new(self)
    }
    #[doc = "Bit 2 - UART2 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_uart2(&mut self) -> SYSCTL_RCGC1_UART2_W<RCGC1_SPEC, 2> {
        SYSCTL_RCGC1_UART2_W::new(self)
    }
    #[doc = "Bit 4 - SSI0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_ssi0(&mut self) -> SYSCTL_RCGC1_SSI0_W<RCGC1_SPEC, 4> {
        SYSCTL_RCGC1_SSI0_W::new(self)
    }
    #[doc = "Bit 5 - SSI1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_ssi1(&mut self) -> SYSCTL_RCGC1_SSI1_W<RCGC1_SPEC, 5> {
        SYSCTL_RCGC1_SSI1_W::new(self)
    }
    #[doc = "Bit 8 - QEI0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_qei0(&mut self) -> SYSCTL_RCGC1_QEI0_W<RCGC1_SPEC, 8> {
        SYSCTL_RCGC1_QEI0_W::new(self)
    }
    #[doc = "Bit 9 - QEI1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_qei1(&mut self) -> SYSCTL_RCGC1_QEI1_W<RCGC1_SPEC, 9> {
        SYSCTL_RCGC1_QEI1_W::new(self)
    }
    #[doc = "Bit 12 - I2C0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_i2c0(&mut self) -> SYSCTL_RCGC1_I2C0_W<RCGC1_SPEC, 12> {
        SYSCTL_RCGC1_I2C0_W::new(self)
    }
    #[doc = "Bit 14 - I2C1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_i2c1(&mut self) -> SYSCTL_RCGC1_I2C1_W<RCGC1_SPEC, 14> {
        SYSCTL_RCGC1_I2C1_W::new(self)
    }
    #[doc = "Bit 16 - Timer 0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_timer0(&mut self) -> SYSCTL_RCGC1_TIMER0_W<RCGC1_SPEC, 16> {
        SYSCTL_RCGC1_TIMER0_W::new(self)
    }
    #[doc = "Bit 17 - Timer 1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_timer1(&mut self) -> SYSCTL_RCGC1_TIMER1_W<RCGC1_SPEC, 17> {
        SYSCTL_RCGC1_TIMER1_W::new(self)
    }
    #[doc = "Bit 18 - Timer 2 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_timer2(&mut self) -> SYSCTL_RCGC1_TIMER2_W<RCGC1_SPEC, 18> {
        SYSCTL_RCGC1_TIMER2_W::new(self)
    }
    #[doc = "Bit 19 - Timer 3 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_timer3(&mut self) -> SYSCTL_RCGC1_TIMER3_W<RCGC1_SPEC, 19> {
        SYSCTL_RCGC1_TIMER3_W::new(self)
    }
    #[doc = "Bit 24 - Analog Comparator 0 Clock Gating"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_comp0(&mut self) -> SYSCTL_RCGC1_COMP0_W<RCGC1_SPEC, 24> {
        SYSCTL_RCGC1_COMP0_W::new(self)
    }
    #[doc = "Bit 25 - Analog Comparator 1 Clock Gating"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc1_comp1(&mut self) -> SYSCTL_RCGC1_COMP1_W<RCGC1_SPEC, 25> {
        SYSCTL_RCGC1_COMP1_W::new(self)
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
#[doc = "Run Mode Clock Gating Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGC1_SPEC;
impl crate::RegisterSpec for RCGC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgc1::R`](R) reader structure"]
impl crate::Readable for RCGC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgc1::W`](W) writer structure"]
impl crate::Writable for RCGC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGC1 to value 0"]
impl crate::Resettable for RCGC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
