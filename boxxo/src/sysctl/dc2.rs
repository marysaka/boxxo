#[doc = "Register `DC2` reader"]
pub type R = crate::R<DC2_SPEC>;
#[doc = "Register `DC2` writer"]
pub type W = crate::W<DC2_SPEC>;
#[doc = "Field `SYSCTL_DC2_UART0` reader - UART Module 0 Present"]
pub type SYSCTL_DC2_UART0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_UART0` writer - UART Module 0 Present"]
pub type SYSCTL_DC2_UART0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_UART1` reader - UART Module 1 Present"]
pub type SYSCTL_DC2_UART1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_UART1` writer - UART Module 1 Present"]
pub type SYSCTL_DC2_UART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_UART2` reader - UART Module 2 Present"]
pub type SYSCTL_DC2_UART2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_UART2` writer - UART Module 2 Present"]
pub type SYSCTL_DC2_UART2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_SSI0` reader - SSI Module 0 Present"]
pub type SYSCTL_DC2_SSI0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_SSI0` writer - SSI Module 0 Present"]
pub type SYSCTL_DC2_SSI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_SSI1` reader - SSI Module 1 Present"]
pub type SYSCTL_DC2_SSI1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_SSI1` writer - SSI Module 1 Present"]
pub type SYSCTL_DC2_SSI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_QEI0` reader - QEI Module 0 Present"]
pub type SYSCTL_DC2_QEI0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_QEI0` writer - QEI Module 0 Present"]
pub type SYSCTL_DC2_QEI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_QEI1` reader - QEI Module 1 Present"]
pub type SYSCTL_DC2_QEI1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_QEI1` writer - QEI Module 1 Present"]
pub type SYSCTL_DC2_QEI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_I2C0` reader - I2C Module 0 Present"]
pub type SYSCTL_DC2_I2C0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_I2C0` writer - I2C Module 0 Present"]
pub type SYSCTL_DC2_I2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_I2C0HS` reader - I2C Module 0 Speed"]
pub type SYSCTL_DC2_I2C0HS_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_I2C0HS` writer - I2C Module 0 Speed"]
pub type SYSCTL_DC2_I2C0HS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_I2C1` reader - I2C Module 1 Present"]
pub type SYSCTL_DC2_I2C1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_I2C1` writer - I2C Module 1 Present"]
pub type SYSCTL_DC2_I2C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_I2C1HS` reader - I2C Module 1 Speed"]
pub type SYSCTL_DC2_I2C1HS_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_I2C1HS` writer - I2C Module 1 Speed"]
pub type SYSCTL_DC2_I2C1HS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_TIMER0` reader - Timer Module 0 Present"]
pub type SYSCTL_DC2_TIMER0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_TIMER0` writer - Timer Module 0 Present"]
pub type SYSCTL_DC2_TIMER0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_TIMER1` reader - Timer Module 1 Present"]
pub type SYSCTL_DC2_TIMER1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_TIMER1` writer - Timer Module 1 Present"]
pub type SYSCTL_DC2_TIMER1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_TIMER2` reader - Timer Module 2 Present"]
pub type SYSCTL_DC2_TIMER2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_TIMER2` writer - Timer Module 2 Present"]
pub type SYSCTL_DC2_TIMER2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_TIMER3` reader - Timer Module 3 Present"]
pub type SYSCTL_DC2_TIMER3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_TIMER3` writer - Timer Module 3 Present"]
pub type SYSCTL_DC2_TIMER3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_COMP0` reader - Analog Comparator 0 Present"]
pub type SYSCTL_DC2_COMP0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_COMP0` writer - Analog Comparator 0 Present"]
pub type SYSCTL_DC2_COMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_COMP1` reader - Analog Comparator 1 Present"]
pub type SYSCTL_DC2_COMP1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_COMP1` writer - Analog Comparator 1 Present"]
pub type SYSCTL_DC2_COMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_COMP2` reader - Analog Comparator 2 Present"]
pub type SYSCTL_DC2_COMP2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_COMP2` writer - Analog Comparator 2 Present"]
pub type SYSCTL_DC2_COMP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_I2S0` reader - I2S Module 0 Present"]
pub type SYSCTL_DC2_I2S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_I2S0` writer - I2S Module 0 Present"]
pub type SYSCTL_DC2_I2S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC2_EPI0` reader - EPI Module 0 Present"]
pub type SYSCTL_DC2_EPI0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC2_EPI0` writer - EPI Module 0 Present"]
pub type SYSCTL_DC2_EPI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_uart0(&self) -> SYSCTL_DC2_UART0_R {
        SYSCTL_DC2_UART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_uart1(&self) -> SYSCTL_DC2_UART1_R {
        SYSCTL_DC2_UART1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_uart2(&self) -> SYSCTL_DC2_UART2_R {
        SYSCTL_DC2_UART2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SSI Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_ssi0(&self) -> SYSCTL_DC2_SSI0_R {
        SYSCTL_DC2_SSI0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SSI Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_ssi1(&self) -> SYSCTL_DC2_SSI1_R {
        SYSCTL_DC2_SSI1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - QEI Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_qei0(&self) -> SYSCTL_DC2_QEI0_R {
        SYSCTL_DC2_QEI0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - QEI Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_qei1(&self) -> SYSCTL_DC2_QEI1_R {
        SYSCTL_DC2_QEI1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_i2c0(&self) -> SYSCTL_DC2_I2C0_R {
        SYSCTL_DC2_I2C0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C Module 0 Speed"]
    #[inline(always)]
    pub fn sysctl_dc2_i2c0hs(&self) -> SYSCTL_DC2_I2C0HS_R {
        SYSCTL_DC2_I2C0HS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_i2c1(&self) -> SYSCTL_DC2_I2C1_R {
        SYSCTL_DC2_I2C1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C Module 1 Speed"]
    #[inline(always)]
    pub fn sysctl_dc2_i2c1hs(&self) -> SYSCTL_DC2_I2C1HS_R {
        SYSCTL_DC2_I2C1HS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_timer0(&self) -> SYSCTL_DC2_TIMER0_R {
        SYSCTL_DC2_TIMER0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_timer1(&self) -> SYSCTL_DC2_TIMER1_R {
        SYSCTL_DC2_TIMER1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_timer2(&self) -> SYSCTL_DC2_TIMER2_R {
        SYSCTL_DC2_TIMER2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_timer3(&self) -> SYSCTL_DC2_TIMER3_R {
        SYSCTL_DC2_TIMER3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog Comparator 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_comp0(&self) -> SYSCTL_DC2_COMP0_R {
        SYSCTL_DC2_COMP0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog Comparator 1 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_comp1(&self) -> SYSCTL_DC2_COMP1_R {
        SYSCTL_DC2_COMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Analog Comparator 2 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_comp2(&self) -> SYSCTL_DC2_COMP2_R {
        SYSCTL_DC2_COMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - I2S Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_i2s0(&self) -> SYSCTL_DC2_I2S0_R {
        SYSCTL_DC2_I2S0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - EPI Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc2_epi0(&self) -> SYSCTL_DC2_EPI0_R {
        SYSCTL_DC2_EPI0_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_uart0(&mut self) -> SYSCTL_DC2_UART0_W<DC2_SPEC, 0> {
        SYSCTL_DC2_UART0_W::new(self)
    }
    #[doc = "Bit 1 - UART Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_uart1(&mut self) -> SYSCTL_DC2_UART1_W<DC2_SPEC, 1> {
        SYSCTL_DC2_UART1_W::new(self)
    }
    #[doc = "Bit 2 - UART Module 2 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_uart2(&mut self) -> SYSCTL_DC2_UART2_W<DC2_SPEC, 2> {
        SYSCTL_DC2_UART2_W::new(self)
    }
    #[doc = "Bit 4 - SSI Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_ssi0(&mut self) -> SYSCTL_DC2_SSI0_W<DC2_SPEC, 4> {
        SYSCTL_DC2_SSI0_W::new(self)
    }
    #[doc = "Bit 5 - SSI Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_ssi1(&mut self) -> SYSCTL_DC2_SSI1_W<DC2_SPEC, 5> {
        SYSCTL_DC2_SSI1_W::new(self)
    }
    #[doc = "Bit 8 - QEI Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_qei0(&mut self) -> SYSCTL_DC2_QEI0_W<DC2_SPEC, 8> {
        SYSCTL_DC2_QEI0_W::new(self)
    }
    #[doc = "Bit 9 - QEI Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_qei1(&mut self) -> SYSCTL_DC2_QEI1_W<DC2_SPEC, 9> {
        SYSCTL_DC2_QEI1_W::new(self)
    }
    #[doc = "Bit 12 - I2C Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_i2c0(&mut self) -> SYSCTL_DC2_I2C0_W<DC2_SPEC, 12> {
        SYSCTL_DC2_I2C0_W::new(self)
    }
    #[doc = "Bit 13 - I2C Module 0 Speed"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_i2c0hs(&mut self) -> SYSCTL_DC2_I2C0HS_W<DC2_SPEC, 13> {
        SYSCTL_DC2_I2C0HS_W::new(self)
    }
    #[doc = "Bit 14 - I2C Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_i2c1(&mut self) -> SYSCTL_DC2_I2C1_W<DC2_SPEC, 14> {
        SYSCTL_DC2_I2C1_W::new(self)
    }
    #[doc = "Bit 15 - I2C Module 1 Speed"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_i2c1hs(&mut self) -> SYSCTL_DC2_I2C1HS_W<DC2_SPEC, 15> {
        SYSCTL_DC2_I2C1HS_W::new(self)
    }
    #[doc = "Bit 16 - Timer Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_timer0(&mut self) -> SYSCTL_DC2_TIMER0_W<DC2_SPEC, 16> {
        SYSCTL_DC2_TIMER0_W::new(self)
    }
    #[doc = "Bit 17 - Timer Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_timer1(&mut self) -> SYSCTL_DC2_TIMER1_W<DC2_SPEC, 17> {
        SYSCTL_DC2_TIMER1_W::new(self)
    }
    #[doc = "Bit 18 - Timer Module 2 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_timer2(&mut self) -> SYSCTL_DC2_TIMER2_W<DC2_SPEC, 18> {
        SYSCTL_DC2_TIMER2_W::new(self)
    }
    #[doc = "Bit 19 - Timer Module 3 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_timer3(&mut self) -> SYSCTL_DC2_TIMER3_W<DC2_SPEC, 19> {
        SYSCTL_DC2_TIMER3_W::new(self)
    }
    #[doc = "Bit 24 - Analog Comparator 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_comp0(&mut self) -> SYSCTL_DC2_COMP0_W<DC2_SPEC, 24> {
        SYSCTL_DC2_COMP0_W::new(self)
    }
    #[doc = "Bit 25 - Analog Comparator 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_comp1(&mut self) -> SYSCTL_DC2_COMP1_W<DC2_SPEC, 25> {
        SYSCTL_DC2_COMP1_W::new(self)
    }
    #[doc = "Bit 26 - Analog Comparator 2 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_comp2(&mut self) -> SYSCTL_DC2_COMP2_W<DC2_SPEC, 26> {
        SYSCTL_DC2_COMP2_W::new(self)
    }
    #[doc = "Bit 28 - I2S Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_i2s0(&mut self) -> SYSCTL_DC2_I2S0_W<DC2_SPEC, 28> {
        SYSCTL_DC2_I2S0_W::new(self)
    }
    #[doc = "Bit 30 - EPI Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc2_epi0(&mut self) -> SYSCTL_DC2_EPI0_W<DC2_SPEC, 30> {
        SYSCTL_DC2_EPI0_W::new(self)
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
#[doc = "Device Capabilities 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC2_SPEC;
impl crate::RegisterSpec for DC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc2::R`](R) reader structure"]
impl crate::Readable for DC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc2::W`](W) writer structure"]
impl crate::Writable for DC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC2 to value 0"]
impl crate::Resettable for DC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
