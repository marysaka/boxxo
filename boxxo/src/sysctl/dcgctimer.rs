#[doc = "Register `DCGCTIMER` reader"]
pub type R = crate::R<DCGCTIMER_SPEC>;
#[doc = "Register `DCGCTIMER` writer"]
pub type W = crate::W<DCGCTIMER_SPEC>;
#[doc = "Field `SYSCTL_DCGCTIMER_D0` reader - Timer 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCTIMER_D0` writer - Timer 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCTIMER_D1` reader - Timer 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCTIMER_D1` writer - Timer 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCTIMER_D2` reader - Timer 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCTIMER_D2` writer - Timer 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCTIMER_D3` reader - Timer 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCTIMER_D3` writer - Timer 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCTIMER_D4` reader - Timer 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D4_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCTIMER_D4` writer - Timer 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCTIMER_D5` reader - Timer 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D5_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCTIMER_D5` writer - Timer 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCTIMER_D5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d0(&self) -> SYSCTL_DCGCTIMER_D0_R {
        SYSCTL_DCGCTIMER_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d1(&self) -> SYSCTL_DCGCTIMER_D1_R {
        SYSCTL_DCGCTIMER_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d2(&self) -> SYSCTL_DCGCTIMER_D2_R {
        SYSCTL_DCGCTIMER_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d3(&self) -> SYSCTL_DCGCTIMER_D3_R {
        SYSCTL_DCGCTIMER_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d4(&self) -> SYSCTL_DCGCTIMER_D4_R {
        SYSCTL_DCGCTIMER_D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d5(&self) -> SYSCTL_DCGCTIMER_D5_R {
        SYSCTL_DCGCTIMER_D5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgctimer_d0(&mut self) -> SYSCTL_DCGCTIMER_D0_W<DCGCTIMER_SPEC, 0> {
        SYSCTL_DCGCTIMER_D0_W::new(self)
    }
    #[doc = "Bit 1 - Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgctimer_d1(&mut self) -> SYSCTL_DCGCTIMER_D1_W<DCGCTIMER_SPEC, 1> {
        SYSCTL_DCGCTIMER_D1_W::new(self)
    }
    #[doc = "Bit 2 - Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgctimer_d2(&mut self) -> SYSCTL_DCGCTIMER_D2_W<DCGCTIMER_SPEC, 2> {
        SYSCTL_DCGCTIMER_D2_W::new(self)
    }
    #[doc = "Bit 3 - Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgctimer_d3(&mut self) -> SYSCTL_DCGCTIMER_D3_W<DCGCTIMER_SPEC, 3> {
        SYSCTL_DCGCTIMER_D3_W::new(self)
    }
    #[doc = "Bit 4 - Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgctimer_d4(&mut self) -> SYSCTL_DCGCTIMER_D4_W<DCGCTIMER_SPEC, 4> {
        SYSCTL_DCGCTIMER_D4_W::new(self)
    }
    #[doc = "Bit 5 - Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgctimer_d5(&mut self) -> SYSCTL_DCGCTIMER_D5_W<DCGCTIMER_SPEC, 5> {
        SYSCTL_DCGCTIMER_D5_W::new(self)
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
#[doc = "Timer Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgctimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgctimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCTIMER_SPEC;
impl crate::RegisterSpec for DCGCTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgctimer::R`](R) reader structure"]
impl crate::Readable for DCGCTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgctimer::W`](W) writer structure"]
impl crate::Writable for DCGCTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCTIMER to value 0"]
impl crate::Resettable for DCGCTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
