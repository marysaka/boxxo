#[doc = "Register `RCGCWTIMER` reader"]
pub type R = crate::R<RCGCWTIMER_SPEC>;
#[doc = "Register `RCGCWTIMER` writer"]
pub type W = crate::W<RCGCWTIMER_SPEC>;
#[doc = "Field `SYSCTL_RCGCWTIMER_R0` reader - Wide Timer 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCWTIMER_R0` writer - Wide Timer 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCWTIMER_R1` reader - Wide Timer 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCWTIMER_R1` writer - Wide Timer 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCWTIMER_R2` reader - Wide Timer 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCWTIMER_R2` writer - Wide Timer 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCWTIMER_R3` reader - Wide Timer 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCWTIMER_R3` writer - Wide Timer 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCWTIMER_R4` reader - Wide Timer 4 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R4_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCWTIMER_R4` writer - Wide Timer 4 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCWTIMER_R5` reader - Wide Timer 5 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R5_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCWTIMER_R5` writer - Wide Timer 5 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCWTIMER_R5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Wide Timer 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcwtimer_r0(&self) -> SYSCTL_RCGCWTIMER_R0_R {
        SYSCTL_RCGCWTIMER_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcwtimer_r1(&self) -> SYSCTL_RCGCWTIMER_R1_R {
        SYSCTL_RCGCWTIMER_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcwtimer_r2(&self) -> SYSCTL_RCGCWTIMER_R2_R {
        SYSCTL_RCGCWTIMER_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcwtimer_r3(&self) -> SYSCTL_RCGCWTIMER_R3_R {
        SYSCTL_RCGCWTIMER_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wide Timer 4 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcwtimer_r4(&self) -> SYSCTL_RCGCWTIMER_R4_R {
        SYSCTL_RCGCWTIMER_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wide Timer 5 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcwtimer_r5(&self) -> SYSCTL_RCGCWTIMER_R5_R {
        SYSCTL_RCGCWTIMER_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wide Timer 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcwtimer_r0(&mut self) -> SYSCTL_RCGCWTIMER_R0_W<RCGCWTIMER_SPEC, 0> {
        SYSCTL_RCGCWTIMER_R0_W::new(self)
    }
    #[doc = "Bit 1 - Wide Timer 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcwtimer_r1(&mut self) -> SYSCTL_RCGCWTIMER_R1_W<RCGCWTIMER_SPEC, 1> {
        SYSCTL_RCGCWTIMER_R1_W::new(self)
    }
    #[doc = "Bit 2 - Wide Timer 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcwtimer_r2(&mut self) -> SYSCTL_RCGCWTIMER_R2_W<RCGCWTIMER_SPEC, 2> {
        SYSCTL_RCGCWTIMER_R2_W::new(self)
    }
    #[doc = "Bit 3 - Wide Timer 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcwtimer_r3(&mut self) -> SYSCTL_RCGCWTIMER_R3_W<RCGCWTIMER_SPEC, 3> {
        SYSCTL_RCGCWTIMER_R3_W::new(self)
    }
    #[doc = "Bit 4 - Wide Timer 4 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcwtimer_r4(&mut self) -> SYSCTL_RCGCWTIMER_R4_W<RCGCWTIMER_SPEC, 4> {
        SYSCTL_RCGCWTIMER_R4_W::new(self)
    }
    #[doc = "Bit 5 - Wide Timer 5 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcwtimer_r5(&mut self) -> SYSCTL_RCGCWTIMER_R5_W<RCGCWTIMER_SPEC, 5> {
        SYSCTL_RCGCWTIMER_R5_W::new(self)
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
#[doc = "Wide Timer Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcwtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcwtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGCWTIMER_SPEC;
impl crate::RegisterSpec for RCGCWTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgcwtimer::R`](R) reader structure"]
impl crate::Readable for RCGCWTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgcwtimer::W`](W) writer structure"]
impl crate::Writable for RCGCWTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCWTIMER to value 0"]
impl crate::Resettable for RCGCWTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
