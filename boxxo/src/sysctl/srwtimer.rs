#[doc = "Register `SRWTIMER` reader"]
pub type R = crate::R<SRWTIMER_SPEC>;
#[doc = "Register `SRWTIMER` writer"]
pub type W = crate::W<SRWTIMER_SPEC>;
#[doc = "Field `SYSCTL_SRWTIMER_R0` reader - Wide Timer 0 Software Reset"]
pub type SYSCTL_SRWTIMER_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRWTIMER_R0` writer - Wide Timer 0 Software Reset"]
pub type SYSCTL_SRWTIMER_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRWTIMER_R1` reader - Wide Timer 1 Software Reset"]
pub type SYSCTL_SRWTIMER_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRWTIMER_R1` writer - Wide Timer 1 Software Reset"]
pub type SYSCTL_SRWTIMER_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRWTIMER_R2` reader - Wide Timer 2 Software Reset"]
pub type SYSCTL_SRWTIMER_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRWTIMER_R2` writer - Wide Timer 2 Software Reset"]
pub type SYSCTL_SRWTIMER_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRWTIMER_R3` reader - Wide Timer 3 Software Reset"]
pub type SYSCTL_SRWTIMER_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRWTIMER_R3` writer - Wide Timer 3 Software Reset"]
pub type SYSCTL_SRWTIMER_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRWTIMER_R4` reader - Wide Timer 4 Software Reset"]
pub type SYSCTL_SRWTIMER_R4_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRWTIMER_R4` writer - Wide Timer 4 Software Reset"]
pub type SYSCTL_SRWTIMER_R4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRWTIMER_R5` reader - Wide Timer 5 Software Reset"]
pub type SYSCTL_SRWTIMER_R5_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRWTIMER_R5` writer - Wide Timer 5 Software Reset"]
pub type SYSCTL_SRWTIMER_R5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Wide Timer 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srwtimer_r0(&self) -> SYSCTL_SRWTIMER_R0_R {
        SYSCTL_SRWTIMER_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srwtimer_r1(&self) -> SYSCTL_SRWTIMER_R1_R {
        SYSCTL_SRWTIMER_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 2 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srwtimer_r2(&self) -> SYSCTL_SRWTIMER_R2_R {
        SYSCTL_SRWTIMER_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 3 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srwtimer_r3(&self) -> SYSCTL_SRWTIMER_R3_R {
        SYSCTL_SRWTIMER_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wide Timer 4 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srwtimer_r4(&self) -> SYSCTL_SRWTIMER_R4_R {
        SYSCTL_SRWTIMER_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wide Timer 5 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srwtimer_r5(&self) -> SYSCTL_SRWTIMER_R5_R {
        SYSCTL_SRWTIMER_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wide Timer 0 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srwtimer_r0(&mut self) -> SYSCTL_SRWTIMER_R0_W<SRWTIMER_SPEC, 0> {
        SYSCTL_SRWTIMER_R0_W::new(self)
    }
    #[doc = "Bit 1 - Wide Timer 1 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srwtimer_r1(&mut self) -> SYSCTL_SRWTIMER_R1_W<SRWTIMER_SPEC, 1> {
        SYSCTL_SRWTIMER_R1_W::new(self)
    }
    #[doc = "Bit 2 - Wide Timer 2 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srwtimer_r2(&mut self) -> SYSCTL_SRWTIMER_R2_W<SRWTIMER_SPEC, 2> {
        SYSCTL_SRWTIMER_R2_W::new(self)
    }
    #[doc = "Bit 3 - Wide Timer 3 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srwtimer_r3(&mut self) -> SYSCTL_SRWTIMER_R3_W<SRWTIMER_SPEC, 3> {
        SYSCTL_SRWTIMER_R3_W::new(self)
    }
    #[doc = "Bit 4 - Wide Timer 4 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srwtimer_r4(&mut self) -> SYSCTL_SRWTIMER_R4_W<SRWTIMER_SPEC, 4> {
        SYSCTL_SRWTIMER_R4_W::new(self)
    }
    #[doc = "Bit 5 - Wide Timer 5 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srwtimer_r5(&mut self) -> SYSCTL_SRWTIMER_R5_W<SRWTIMER_SPEC, 5> {
        SYSCTL_SRWTIMER_R5_W::new(self)
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
#[doc = "Wide Timer Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srwtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srwtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRWTIMER_SPEC;
impl crate::RegisterSpec for SRWTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srwtimer::R`](R) reader structure"]
impl crate::Readable for SRWTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srwtimer::W`](W) writer structure"]
impl crate::Writable for SRWTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRWTIMER to value 0"]
impl crate::Resettable for SRWTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
