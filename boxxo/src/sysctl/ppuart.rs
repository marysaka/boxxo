#[doc = "Register `PPUART` reader"]
pub type R = crate::R<PPUART_SPEC>;
#[doc = "Register `PPUART` writer"]
pub type W = crate::W<PPUART_SPEC>;
#[doc = "Field `SYSCTL_PPUART_P0` reader - UART Module 0 Present"]
pub type SYSCTL_PPUART_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPUART_P0` writer - UART Module 0 Present"]
pub type SYSCTL_PPUART_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPUART_P1` reader - UART Module 1 Present"]
pub type SYSCTL_PPUART_P1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPUART_P1` writer - UART Module 1 Present"]
pub type SYSCTL_PPUART_P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPUART_P2` reader - UART Module 2 Present"]
pub type SYSCTL_PPUART_P2_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPUART_P2` writer - UART Module 2 Present"]
pub type SYSCTL_PPUART_P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPUART_P3` reader - UART Module 3 Present"]
pub type SYSCTL_PPUART_P3_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPUART_P3` writer - UART Module 3 Present"]
pub type SYSCTL_PPUART_P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPUART_P4` reader - UART Module 4 Present"]
pub type SYSCTL_PPUART_P4_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPUART_P4` writer - UART Module 4 Present"]
pub type SYSCTL_PPUART_P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPUART_P5` reader - UART Module 5 Present"]
pub type SYSCTL_PPUART_P5_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPUART_P5` writer - UART Module 5 Present"]
pub type SYSCTL_PPUART_P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPUART_P6` reader - UART Module 6 Present"]
pub type SYSCTL_PPUART_P6_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPUART_P6` writer - UART Module 6 Present"]
pub type SYSCTL_PPUART_P6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPUART_P7` reader - UART Module 7 Present"]
pub type SYSCTL_PPUART_P7_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPUART_P7` writer - UART Module 7 Present"]
pub type SYSCTL_PPUART_P7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppuart_p0(&self) -> SYSCTL_PPUART_P0_R {
        SYSCTL_PPUART_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppuart_p1(&self) -> SYSCTL_PPUART_P1_R {
        SYSCTL_PPUART_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppuart_p2(&self) -> SYSCTL_PPUART_P2_R {
        SYSCTL_PPUART_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppuart_p3(&self) -> SYSCTL_PPUART_P3_R {
        SYSCTL_PPUART_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Module 4 Present"]
    #[inline(always)]
    pub fn sysctl_ppuart_p4(&self) -> SYSCTL_PPUART_P4_R {
        SYSCTL_PPUART_P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Module 5 Present"]
    #[inline(always)]
    pub fn sysctl_ppuart_p5(&self) -> SYSCTL_PPUART_P5_R {
        SYSCTL_PPUART_P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Module 6 Present"]
    #[inline(always)]
    pub fn sysctl_ppuart_p6(&self) -> SYSCTL_PPUART_P6_R {
        SYSCTL_PPUART_P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Module 7 Present"]
    #[inline(always)]
    pub fn sysctl_ppuart_p7(&self) -> SYSCTL_PPUART_P7_R {
        SYSCTL_PPUART_P7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppuart_p0(&mut self) -> SYSCTL_PPUART_P0_W<PPUART_SPEC, 0> {
        SYSCTL_PPUART_P0_W::new(self)
    }
    #[doc = "Bit 1 - UART Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppuart_p1(&mut self) -> SYSCTL_PPUART_P1_W<PPUART_SPEC, 1> {
        SYSCTL_PPUART_P1_W::new(self)
    }
    #[doc = "Bit 2 - UART Module 2 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppuart_p2(&mut self) -> SYSCTL_PPUART_P2_W<PPUART_SPEC, 2> {
        SYSCTL_PPUART_P2_W::new(self)
    }
    #[doc = "Bit 3 - UART Module 3 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppuart_p3(&mut self) -> SYSCTL_PPUART_P3_W<PPUART_SPEC, 3> {
        SYSCTL_PPUART_P3_W::new(self)
    }
    #[doc = "Bit 4 - UART Module 4 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppuart_p4(&mut self) -> SYSCTL_PPUART_P4_W<PPUART_SPEC, 4> {
        SYSCTL_PPUART_P4_W::new(self)
    }
    #[doc = "Bit 5 - UART Module 5 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppuart_p5(&mut self) -> SYSCTL_PPUART_P5_W<PPUART_SPEC, 5> {
        SYSCTL_PPUART_P5_W::new(self)
    }
    #[doc = "Bit 6 - UART Module 6 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppuart_p6(&mut self) -> SYSCTL_PPUART_P6_W<PPUART_SPEC, 6> {
        SYSCTL_PPUART_P6_W::new(self)
    }
    #[doc = "Bit 7 - UART Module 7 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppuart_p7(&mut self) -> SYSCTL_PPUART_P7_W<PPUART_SPEC, 7> {
        SYSCTL_PPUART_P7_W::new(self)
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
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppuart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppuart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPUART_SPEC;
impl crate::RegisterSpec for PPUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppuart::R`](R) reader structure"]
impl crate::Readable for PPUART_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppuart::W`](W) writer structure"]
impl crate::Writable for PPUART_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPUART to value 0"]
impl crate::Resettable for PPUART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
