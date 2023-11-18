#[doc = "Register `SCGCWTIMER` reader"]
pub type R = crate::R<SCGCWTIMER_SPEC>;
#[doc = "Register `SCGCWTIMER` writer"]
pub type W = crate::W<SCGCWTIMER_SPEC>;
#[doc = "Field `SYSCTL_SCGCWTIMER_S0` reader - Wide Timer 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCWTIMER_S0` writer - Wide Timer 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCWTIMER_S1` reader - Wide Timer 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCWTIMER_S1` writer - Wide Timer 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCWTIMER_S2` reader - Wide Timer 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCWTIMER_S2` writer - Wide Timer 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCWTIMER_S3` reader - Wide Timer 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCWTIMER_S3` writer - Wide Timer 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCWTIMER_S4` reader - Wide Timer 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S4_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCWTIMER_S4` writer - Wide Timer 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCWTIMER_S5` reader - Wide Timer 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S5_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCWTIMER_S5` writer - Wide Timer 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWTIMER_S5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Wide Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwtimer_s0(&self) -> SYSCTL_SCGCWTIMER_S0_R {
        SYSCTL_SCGCWTIMER_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwtimer_s1(&self) -> SYSCTL_SCGCWTIMER_S1_R {
        SYSCTL_SCGCWTIMER_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwtimer_s2(&self) -> SYSCTL_SCGCWTIMER_S2_R {
        SYSCTL_SCGCWTIMER_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwtimer_s3(&self) -> SYSCTL_SCGCWTIMER_S3_R {
        SYSCTL_SCGCWTIMER_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wide Timer 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwtimer_s4(&self) -> SYSCTL_SCGCWTIMER_S4_R {
        SYSCTL_SCGCWTIMER_S4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wide Timer 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwtimer_s5(&self) -> SYSCTL_SCGCWTIMER_S5_R {
        SYSCTL_SCGCWTIMER_S5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wide Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcwtimer_s0(&mut self) -> SYSCTL_SCGCWTIMER_S0_W<SCGCWTIMER_SPEC, 0> {
        SYSCTL_SCGCWTIMER_S0_W::new(self)
    }
    #[doc = "Bit 1 - Wide Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcwtimer_s1(&mut self) -> SYSCTL_SCGCWTIMER_S1_W<SCGCWTIMER_SPEC, 1> {
        SYSCTL_SCGCWTIMER_S1_W::new(self)
    }
    #[doc = "Bit 2 - Wide Timer 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcwtimer_s2(&mut self) -> SYSCTL_SCGCWTIMER_S2_W<SCGCWTIMER_SPEC, 2> {
        SYSCTL_SCGCWTIMER_S2_W::new(self)
    }
    #[doc = "Bit 3 - Wide Timer 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcwtimer_s3(&mut self) -> SYSCTL_SCGCWTIMER_S3_W<SCGCWTIMER_SPEC, 3> {
        SYSCTL_SCGCWTIMER_S3_W::new(self)
    }
    #[doc = "Bit 4 - Wide Timer 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcwtimer_s4(&mut self) -> SYSCTL_SCGCWTIMER_S4_W<SCGCWTIMER_SPEC, 4> {
        SYSCTL_SCGCWTIMER_S4_W::new(self)
    }
    #[doc = "Bit 5 - Wide Timer 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcwtimer_s5(&mut self) -> SYSCTL_SCGCWTIMER_S5_W<SCGCWTIMER_SPEC, 5> {
        SYSCTL_SCGCWTIMER_S5_W::new(self)
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
#[doc = "Wide Timer Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcwtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcwtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCWTIMER_SPEC;
impl crate::RegisterSpec for SCGCWTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcwtimer::R`](R) reader structure"]
impl crate::Readable for SCGCWTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcwtimer::W`](W) writer structure"]
impl crate::Writable for SCGCWTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCWTIMER to value 0"]
impl crate::Resettable for SCGCWTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
