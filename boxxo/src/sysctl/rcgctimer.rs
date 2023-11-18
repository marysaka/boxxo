#[doc = "Register `RCGCTIMER` reader"]
pub type R = crate::R<RCGCTIMER_SPEC>;
#[doc = "Register `RCGCTIMER` writer"]
pub type W = crate::W<RCGCTIMER_SPEC>;
#[doc = "Field `SYSCTL_RCGCTIMER_R0` reader - Timer 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCTIMER_R0` writer - Timer 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCTIMER_R1` reader - Timer 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCTIMER_R1` writer - Timer 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCTIMER_R2` reader - Timer 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCTIMER_R2` writer - Timer 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCTIMER_R3` reader - Timer 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCTIMER_R3` writer - Timer 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCTIMER_R4` reader - Timer 4 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R4_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCTIMER_R4` writer - Timer 4 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCTIMER_R5` reader - Timer 5 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R5_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCTIMER_R5` writer - Timer 5 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCTIMER_R5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgctimer_r0(&self) -> SYSCTL_RCGCTIMER_R0_R {
        SYSCTL_RCGCTIMER_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgctimer_r1(&self) -> SYSCTL_RCGCTIMER_R1_R {
        SYSCTL_RCGCTIMER_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgctimer_r2(&self) -> SYSCTL_RCGCTIMER_R2_R {
        SYSCTL_RCGCTIMER_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgctimer_r3(&self) -> SYSCTL_RCGCTIMER_R3_R {
        SYSCTL_RCGCTIMER_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 4 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgctimer_r4(&self) -> SYSCTL_RCGCTIMER_R4_R {
        SYSCTL_RCGCTIMER_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 5 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgctimer_r5(&self) -> SYSCTL_RCGCTIMER_R5_R {
        SYSCTL_RCGCTIMER_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgctimer_r0(&mut self) -> SYSCTL_RCGCTIMER_R0_W<RCGCTIMER_SPEC, 0> {
        SYSCTL_RCGCTIMER_R0_W::new(self)
    }
    #[doc = "Bit 1 - Timer 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgctimer_r1(&mut self) -> SYSCTL_RCGCTIMER_R1_W<RCGCTIMER_SPEC, 1> {
        SYSCTL_RCGCTIMER_R1_W::new(self)
    }
    #[doc = "Bit 2 - Timer 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgctimer_r2(&mut self) -> SYSCTL_RCGCTIMER_R2_W<RCGCTIMER_SPEC, 2> {
        SYSCTL_RCGCTIMER_R2_W::new(self)
    }
    #[doc = "Bit 3 - Timer 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgctimer_r3(&mut self) -> SYSCTL_RCGCTIMER_R3_W<RCGCTIMER_SPEC, 3> {
        SYSCTL_RCGCTIMER_R3_W::new(self)
    }
    #[doc = "Bit 4 - Timer 4 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgctimer_r4(&mut self) -> SYSCTL_RCGCTIMER_R4_W<RCGCTIMER_SPEC, 4> {
        SYSCTL_RCGCTIMER_R4_W::new(self)
    }
    #[doc = "Bit 5 - Timer 5 Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgctimer_r5(&mut self) -> SYSCTL_RCGCTIMER_R5_W<RCGCTIMER_SPEC, 5> {
        SYSCTL_RCGCTIMER_R5_W::new(self)
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
#[doc = "Timer Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgctimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgctimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGCTIMER_SPEC;
impl crate::RegisterSpec for RCGCTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgctimer::R`](R) reader structure"]
impl crate::Readable for RCGCTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgctimer::W`](W) writer structure"]
impl crate::Writable for RCGCTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCTIMER to value 0"]
impl crate::Resettable for RCGCTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
