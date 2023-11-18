#[doc = "Register `SRTIMER` reader"]
pub type R = crate::R<SRTIMER_SPEC>;
#[doc = "Register `SRTIMER` writer"]
pub type W = crate::W<SRTIMER_SPEC>;
#[doc = "Field `SYSCTL_SRTIMER_R0` reader - Timer 0 Software Reset"]
pub type SYSCTL_SRTIMER_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRTIMER_R0` writer - Timer 0 Software Reset"]
pub type SYSCTL_SRTIMER_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRTIMER_R1` reader - Timer 1 Software Reset"]
pub type SYSCTL_SRTIMER_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRTIMER_R1` writer - Timer 1 Software Reset"]
pub type SYSCTL_SRTIMER_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRTIMER_R2` reader - Timer 2 Software Reset"]
pub type SYSCTL_SRTIMER_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRTIMER_R2` writer - Timer 2 Software Reset"]
pub type SYSCTL_SRTIMER_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRTIMER_R3` reader - Timer 3 Software Reset"]
pub type SYSCTL_SRTIMER_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRTIMER_R3` writer - Timer 3 Software Reset"]
pub type SYSCTL_SRTIMER_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRTIMER_R4` reader - Timer 4 Software Reset"]
pub type SYSCTL_SRTIMER_R4_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRTIMER_R4` writer - Timer 4 Software Reset"]
pub type SYSCTL_SRTIMER_R4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRTIMER_R5` reader - Timer 5 Software Reset"]
pub type SYSCTL_SRTIMER_R5_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRTIMER_R5` writer - Timer 5 Software Reset"]
pub type SYSCTL_SRTIMER_R5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r0(&self) -> SYSCTL_SRTIMER_R0_R {
        SYSCTL_SRTIMER_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r1(&self) -> SYSCTL_SRTIMER_R1_R {
        SYSCTL_SRTIMER_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r2(&self) -> SYSCTL_SRTIMER_R2_R {
        SYSCTL_SRTIMER_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 3 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r3(&self) -> SYSCTL_SRTIMER_R3_R {
        SYSCTL_SRTIMER_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 4 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r4(&self) -> SYSCTL_SRTIMER_R4_R {
        SYSCTL_SRTIMER_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 5 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srtimer_r5(&self) -> SYSCTL_SRTIMER_R5_R {
        SYSCTL_SRTIMER_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srtimer_r0(&mut self) -> SYSCTL_SRTIMER_R0_W<SRTIMER_SPEC, 0> {
        SYSCTL_SRTIMER_R0_W::new(self)
    }
    #[doc = "Bit 1 - Timer 1 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srtimer_r1(&mut self) -> SYSCTL_SRTIMER_R1_W<SRTIMER_SPEC, 1> {
        SYSCTL_SRTIMER_R1_W::new(self)
    }
    #[doc = "Bit 2 - Timer 2 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srtimer_r2(&mut self) -> SYSCTL_SRTIMER_R2_W<SRTIMER_SPEC, 2> {
        SYSCTL_SRTIMER_R2_W::new(self)
    }
    #[doc = "Bit 3 - Timer 3 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srtimer_r3(&mut self) -> SYSCTL_SRTIMER_R3_W<SRTIMER_SPEC, 3> {
        SYSCTL_SRTIMER_R3_W::new(self)
    }
    #[doc = "Bit 4 - Timer 4 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srtimer_r4(&mut self) -> SYSCTL_SRTIMER_R4_W<SRTIMER_SPEC, 4> {
        SYSCTL_SRTIMER_R4_W::new(self)
    }
    #[doc = "Bit 5 - Timer 5 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srtimer_r5(&mut self) -> SYSCTL_SRTIMER_R5_W<SRTIMER_SPEC, 5> {
        SYSCTL_SRTIMER_R5_W::new(self)
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
#[doc = "Timer Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRTIMER_SPEC;
impl crate::RegisterSpec for SRTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srtimer::R`](R) reader structure"]
impl crate::Readable for SRTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srtimer::W`](W) writer structure"]
impl crate::Writable for SRTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRTIMER to value 0"]
impl crate::Resettable for SRTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
