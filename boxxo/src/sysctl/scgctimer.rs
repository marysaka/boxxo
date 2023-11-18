#[doc = "Register `SCGCTIMER` reader"]
pub type R = crate::R<SCGCTIMER_SPEC>;
#[doc = "Register `SCGCTIMER` writer"]
pub type W = crate::W<SCGCTIMER_SPEC>;
#[doc = "Field `SYSCTL_SCGCTIMER_S0` reader - Timer 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCTIMER_S0` writer - Timer 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCTIMER_S1` reader - Timer 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCTIMER_S1` writer - Timer 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCTIMER_S2` reader - Timer 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCTIMER_S2` writer - Timer 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCTIMER_S3` reader - Timer 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCTIMER_S3` writer - Timer 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCTIMER_S4` reader - Timer 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S4_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCTIMER_S4` writer - Timer 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCTIMER_S5` reader - Timer 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S5_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCTIMER_S5` writer - Timer 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCTIMER_S5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgctimer_s0(&self) -> SYSCTL_SCGCTIMER_S0_R {
        SYSCTL_SCGCTIMER_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgctimer_s1(&self) -> SYSCTL_SCGCTIMER_S1_R {
        SYSCTL_SCGCTIMER_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgctimer_s2(&self) -> SYSCTL_SCGCTIMER_S2_R {
        SYSCTL_SCGCTIMER_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgctimer_s3(&self) -> SYSCTL_SCGCTIMER_S3_R {
        SYSCTL_SCGCTIMER_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgctimer_s4(&self) -> SYSCTL_SCGCTIMER_S4_R {
        SYSCTL_SCGCTIMER_S4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgctimer_s5(&self) -> SYSCTL_SCGCTIMER_S5_R {
        SYSCTL_SCGCTIMER_S5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgctimer_s0(&mut self) -> SYSCTL_SCGCTIMER_S0_W<SCGCTIMER_SPEC, 0> {
        SYSCTL_SCGCTIMER_S0_W::new(self)
    }
    #[doc = "Bit 1 - Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgctimer_s1(&mut self) -> SYSCTL_SCGCTIMER_S1_W<SCGCTIMER_SPEC, 1> {
        SYSCTL_SCGCTIMER_S1_W::new(self)
    }
    #[doc = "Bit 2 - Timer 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgctimer_s2(&mut self) -> SYSCTL_SCGCTIMER_S2_W<SCGCTIMER_SPEC, 2> {
        SYSCTL_SCGCTIMER_S2_W::new(self)
    }
    #[doc = "Bit 3 - Timer 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgctimer_s3(&mut self) -> SYSCTL_SCGCTIMER_S3_W<SCGCTIMER_SPEC, 3> {
        SYSCTL_SCGCTIMER_S3_W::new(self)
    }
    #[doc = "Bit 4 - Timer 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgctimer_s4(&mut self) -> SYSCTL_SCGCTIMER_S4_W<SCGCTIMER_SPEC, 4> {
        SYSCTL_SCGCTIMER_S4_W::new(self)
    }
    #[doc = "Bit 5 - Timer 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgctimer_s5(&mut self) -> SYSCTL_SCGCTIMER_S5_W<SCGCTIMER_SPEC, 5> {
        SYSCTL_SCGCTIMER_S5_W::new(self)
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
#[doc = "Timer Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgctimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgctimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCTIMER_SPEC;
impl crate::RegisterSpec for SCGCTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgctimer::R`](R) reader structure"]
impl crate::Readable for SCGCTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgctimer::W`](W) writer structure"]
impl crate::Writable for SCGCTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCTIMER to value 0"]
impl crate::Resettable for SCGCTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
