#[doc = "Register `DCGCWTIMER` reader"]
pub type R = crate::R<DCGCWTIMER_SPEC>;
#[doc = "Register `DCGCWTIMER` writer"]
pub type W = crate::W<DCGCWTIMER_SPEC>;
#[doc = "Field `SYSCTL_DCGCWTIMER_D0` reader - Wide Timer 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCWTIMER_D0` writer - Wide Timer 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCWTIMER_D1` reader - Wide Timer 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCWTIMER_D1` writer - Wide Timer 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCWTIMER_D2` reader - Wide Timer 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCWTIMER_D2` writer - Wide Timer 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCWTIMER_D3` reader - Wide Timer 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCWTIMER_D3` writer - Wide Timer 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCWTIMER_D4` reader - Wide Timer 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D4_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCWTIMER_D4` writer - Wide Timer 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCWTIMER_D5` reader - Wide Timer 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D5_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCWTIMER_D5` writer - Wide Timer 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCWTIMER_D5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Wide Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcwtimer_d0(&self) -> SYSCTL_DCGCWTIMER_D0_R {
        SYSCTL_DCGCWTIMER_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcwtimer_d1(&self) -> SYSCTL_DCGCWTIMER_D1_R {
        SYSCTL_DCGCWTIMER_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcwtimer_d2(&self) -> SYSCTL_DCGCWTIMER_D2_R {
        SYSCTL_DCGCWTIMER_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcwtimer_d3(&self) -> SYSCTL_DCGCWTIMER_D3_R {
        SYSCTL_DCGCWTIMER_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wide Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcwtimer_d4(&self) -> SYSCTL_DCGCWTIMER_D4_R {
        SYSCTL_DCGCWTIMER_D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wide Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcwtimer_d5(&self) -> SYSCTL_DCGCWTIMER_D5_R {
        SYSCTL_DCGCWTIMER_D5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wide Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcwtimer_d0(&mut self) -> SYSCTL_DCGCWTIMER_D0_W<DCGCWTIMER_SPEC, 0> {
        SYSCTL_DCGCWTIMER_D0_W::new(self)
    }
    #[doc = "Bit 1 - Wide Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcwtimer_d1(&mut self) -> SYSCTL_DCGCWTIMER_D1_W<DCGCWTIMER_SPEC, 1> {
        SYSCTL_DCGCWTIMER_D1_W::new(self)
    }
    #[doc = "Bit 2 - Wide Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcwtimer_d2(&mut self) -> SYSCTL_DCGCWTIMER_D2_W<DCGCWTIMER_SPEC, 2> {
        SYSCTL_DCGCWTIMER_D2_W::new(self)
    }
    #[doc = "Bit 3 - Wide Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcwtimer_d3(&mut self) -> SYSCTL_DCGCWTIMER_D3_W<DCGCWTIMER_SPEC, 3> {
        SYSCTL_DCGCWTIMER_D3_W::new(self)
    }
    #[doc = "Bit 4 - Wide Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcwtimer_d4(&mut self) -> SYSCTL_DCGCWTIMER_D4_W<DCGCWTIMER_SPEC, 4> {
        SYSCTL_DCGCWTIMER_D4_W::new(self)
    }
    #[doc = "Bit 5 - Wide Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcwtimer_d5(&mut self) -> SYSCTL_DCGCWTIMER_D5_W<DCGCWTIMER_SPEC, 5> {
        SYSCTL_DCGCWTIMER_D5_W::new(self)
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
#[doc = "Wide Timer Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcwtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcwtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCWTIMER_SPEC;
impl crate::RegisterSpec for DCGCWTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcwtimer::R`](R) reader structure"]
impl crate::Readable for DCGCWTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgcwtimer::W`](W) writer structure"]
impl crate::Writable for DCGCWTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCWTIMER to value 0"]
impl crate::Resettable for DCGCWTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
