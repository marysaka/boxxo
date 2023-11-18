#[doc = "Register `SCGCUART` reader"]
pub type R = crate::R<SCGCUART_SPEC>;
#[doc = "Register `SCGCUART` writer"]
pub type W = crate::W<SCGCUART_SPEC>;
#[doc = "Field `SYSCTL_SCGCUART_S0` reader - UART Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCUART_S0` writer - UART Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCUART_S1` reader - UART Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCUART_S1` writer - UART Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCUART_S2` reader - UART Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCUART_S2` writer - UART Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCUART_S3` reader - UART Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCUART_S3` writer - UART Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCUART_S4` reader - UART Module 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S4_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCUART_S4` writer - UART Module 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCUART_S5` reader - UART Module 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S5_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCUART_S5` writer - UART Module 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCUART_S6` reader - UART Module 6 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S6_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCUART_S6` writer - UART Module 6 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCUART_S7` reader - UART Module 7 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S7_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCUART_S7` writer - UART Module 7 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUART_S7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s0(&self) -> SYSCTL_SCGCUART_S0_R {
        SYSCTL_SCGCUART_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s1(&self) -> SYSCTL_SCGCUART_S1_R {
        SYSCTL_SCGCUART_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s2(&self) -> SYSCTL_SCGCUART_S2_R {
        SYSCTL_SCGCUART_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s3(&self) -> SYSCTL_SCGCUART_S3_R {
        SYSCTL_SCGCUART_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Module 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s4(&self) -> SYSCTL_SCGCUART_S4_R {
        SYSCTL_SCGCUART_S4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Module 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s5(&self) -> SYSCTL_SCGCUART_S5_R {
        SYSCTL_SCGCUART_S5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Module 6 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s6(&self) -> SYSCTL_SCGCUART_S6_R {
        SYSCTL_SCGCUART_S6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Module 7 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcuart_s7(&self) -> SYSCTL_SCGCUART_S7_R {
        SYSCTL_SCGCUART_S7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcuart_s0(&mut self) -> SYSCTL_SCGCUART_S0_W<SCGCUART_SPEC, 0> {
        SYSCTL_SCGCUART_S0_W::new(self)
    }
    #[doc = "Bit 1 - UART Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcuart_s1(&mut self) -> SYSCTL_SCGCUART_S1_W<SCGCUART_SPEC, 1> {
        SYSCTL_SCGCUART_S1_W::new(self)
    }
    #[doc = "Bit 2 - UART Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcuart_s2(&mut self) -> SYSCTL_SCGCUART_S2_W<SCGCUART_SPEC, 2> {
        SYSCTL_SCGCUART_S2_W::new(self)
    }
    #[doc = "Bit 3 - UART Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcuart_s3(&mut self) -> SYSCTL_SCGCUART_S3_W<SCGCUART_SPEC, 3> {
        SYSCTL_SCGCUART_S3_W::new(self)
    }
    #[doc = "Bit 4 - UART Module 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcuart_s4(&mut self) -> SYSCTL_SCGCUART_S4_W<SCGCUART_SPEC, 4> {
        SYSCTL_SCGCUART_S4_W::new(self)
    }
    #[doc = "Bit 5 - UART Module 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcuart_s5(&mut self) -> SYSCTL_SCGCUART_S5_W<SCGCUART_SPEC, 5> {
        SYSCTL_SCGCUART_S5_W::new(self)
    }
    #[doc = "Bit 6 - UART Module 6 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcuart_s6(&mut self) -> SYSCTL_SCGCUART_S6_W<SCGCUART_SPEC, 6> {
        SYSCTL_SCGCUART_S6_W::new(self)
    }
    #[doc = "Bit 7 - UART Module 7 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcuart_s7(&mut self) -> SYSCTL_SCGCUART_S7_W<SCGCUART_SPEC, 7> {
        SYSCTL_SCGCUART_S7_W::new(self)
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
#[doc = "Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcuart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcuart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCUART_SPEC;
impl crate::RegisterSpec for SCGCUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcuart::R`](R) reader structure"]
impl crate::Readable for SCGCUART_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcuart::W`](W) writer structure"]
impl crate::Writable for SCGCUART_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCUART to value 0"]
impl crate::Resettable for SCGCUART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
