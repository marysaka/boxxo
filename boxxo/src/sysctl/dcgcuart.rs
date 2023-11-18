#[doc = "Register `DCGCUART` reader"]
pub type R = crate::R<DCGCUART_SPEC>;
#[doc = "Register `DCGCUART` writer"]
pub type W = crate::W<DCGCUART_SPEC>;
#[doc = "Field `SYSCTL_DCGCUART_D0` reader - UART Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCUART_D0` writer - UART Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCUART_D1` reader - UART Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCUART_D1` writer - UART Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCUART_D2` reader - UART Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCUART_D2` writer - UART Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCUART_D3` reader - UART Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCUART_D3` writer - UART Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCUART_D4` reader - UART Module 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D4_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCUART_D4` writer - UART Module 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCUART_D5` reader - UART Module 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D5_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCUART_D5` writer - UART Module 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCUART_D6` reader - UART Module 6 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D6_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCUART_D6` writer - UART Module 6 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCUART_D7` reader - UART Module 7 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D7_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCUART_D7` writer - UART Module 7 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUART_D7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d0(&self) -> SYSCTL_DCGCUART_D0_R {
        SYSCTL_DCGCUART_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d1(&self) -> SYSCTL_DCGCUART_D1_R {
        SYSCTL_DCGCUART_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d2(&self) -> SYSCTL_DCGCUART_D2_R {
        SYSCTL_DCGCUART_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d3(&self) -> SYSCTL_DCGCUART_D3_R {
        SYSCTL_DCGCUART_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d4(&self) -> SYSCTL_DCGCUART_D4_R {
        SYSCTL_DCGCUART_D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d5(&self) -> SYSCTL_DCGCUART_D5_R {
        SYSCTL_DCGCUART_D5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d6(&self) -> SYSCTL_DCGCUART_D6_R {
        SYSCTL_DCGCUART_D6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcuart_d7(&self) -> SYSCTL_DCGCUART_D7_R {
        SYSCTL_DCGCUART_D7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcuart_d0(&mut self) -> SYSCTL_DCGCUART_D0_W<DCGCUART_SPEC, 0> {
        SYSCTL_DCGCUART_D0_W::new(self)
    }
    #[doc = "Bit 1 - UART Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcuart_d1(&mut self) -> SYSCTL_DCGCUART_D1_W<DCGCUART_SPEC, 1> {
        SYSCTL_DCGCUART_D1_W::new(self)
    }
    #[doc = "Bit 2 - UART Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcuart_d2(&mut self) -> SYSCTL_DCGCUART_D2_W<DCGCUART_SPEC, 2> {
        SYSCTL_DCGCUART_D2_W::new(self)
    }
    #[doc = "Bit 3 - UART Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcuart_d3(&mut self) -> SYSCTL_DCGCUART_D3_W<DCGCUART_SPEC, 3> {
        SYSCTL_DCGCUART_D3_W::new(self)
    }
    #[doc = "Bit 4 - UART Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcuart_d4(&mut self) -> SYSCTL_DCGCUART_D4_W<DCGCUART_SPEC, 4> {
        SYSCTL_DCGCUART_D4_W::new(self)
    }
    #[doc = "Bit 5 - UART Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcuart_d5(&mut self) -> SYSCTL_DCGCUART_D5_W<DCGCUART_SPEC, 5> {
        SYSCTL_DCGCUART_D5_W::new(self)
    }
    #[doc = "Bit 6 - UART Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcuart_d6(&mut self) -> SYSCTL_DCGCUART_D6_W<DCGCUART_SPEC, 6> {
        SYSCTL_DCGCUART_D6_W::new(self)
    }
    #[doc = "Bit 7 - UART Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcuart_d7(&mut self) -> SYSCTL_DCGCUART_D7_W<DCGCUART_SPEC, 7> {
        SYSCTL_DCGCUART_D7_W::new(self)
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
#[doc = "Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcuart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcuart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCUART_SPEC;
impl crate::RegisterSpec for DCGCUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcuart::R`](R) reader structure"]
impl crate::Readable for DCGCUART_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgcuart::W`](W) writer structure"]
impl crate::Writable for DCGCUART_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCUART to value 0"]
impl crate::Resettable for DCGCUART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
