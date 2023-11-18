#[doc = "Register `PPWTIMER` reader"]
pub type R = crate::R<PPWTIMER_SPEC>;
#[doc = "Register `PPWTIMER` writer"]
pub type W = crate::W<PPWTIMER_SPEC>;
#[doc = "Field `SYSCTL_PPWTIMER_P0` reader - Wide Timer 0 Present"]
pub type SYSCTL_PPWTIMER_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPWTIMER_P0` writer - Wide Timer 0 Present"]
pub type SYSCTL_PPWTIMER_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPWTIMER_P1` reader - Wide Timer 1 Present"]
pub type SYSCTL_PPWTIMER_P1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPWTIMER_P1` writer - Wide Timer 1 Present"]
pub type SYSCTL_PPWTIMER_P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPWTIMER_P2` reader - Wide Timer 2 Present"]
pub type SYSCTL_PPWTIMER_P2_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPWTIMER_P2` writer - Wide Timer 2 Present"]
pub type SYSCTL_PPWTIMER_P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPWTIMER_P3` reader - Wide Timer 3 Present"]
pub type SYSCTL_PPWTIMER_P3_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPWTIMER_P3` writer - Wide Timer 3 Present"]
pub type SYSCTL_PPWTIMER_P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPWTIMER_P4` reader - Wide Timer 4 Present"]
pub type SYSCTL_PPWTIMER_P4_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPWTIMER_P4` writer - Wide Timer 4 Present"]
pub type SYSCTL_PPWTIMER_P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPWTIMER_P5` reader - Wide Timer 5 Present"]
pub type SYSCTL_PPWTIMER_P5_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPWTIMER_P5` writer - Wide Timer 5 Present"]
pub type SYSCTL_PPWTIMER_P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Wide Timer 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppwtimer_p0(&self) -> SYSCTL_PPWTIMER_P0_R {
        SYSCTL_PPWTIMER_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppwtimer_p1(&self) -> SYSCTL_PPWTIMER_P1_R {
        SYSCTL_PPWTIMER_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppwtimer_p2(&self) -> SYSCTL_PPWTIMER_P2_R {
        SYSCTL_PPWTIMER_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppwtimer_p3(&self) -> SYSCTL_PPWTIMER_P3_R {
        SYSCTL_PPWTIMER_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wide Timer 4 Present"]
    #[inline(always)]
    pub fn sysctl_ppwtimer_p4(&self) -> SYSCTL_PPWTIMER_P4_R {
        SYSCTL_PPWTIMER_P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wide Timer 5 Present"]
    #[inline(always)]
    pub fn sysctl_ppwtimer_p5(&self) -> SYSCTL_PPWTIMER_P5_R {
        SYSCTL_PPWTIMER_P5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wide Timer 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppwtimer_p0(&mut self) -> SYSCTL_PPWTIMER_P0_W<PPWTIMER_SPEC, 0> {
        SYSCTL_PPWTIMER_P0_W::new(self)
    }
    #[doc = "Bit 1 - Wide Timer 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppwtimer_p1(&mut self) -> SYSCTL_PPWTIMER_P1_W<PPWTIMER_SPEC, 1> {
        SYSCTL_PPWTIMER_P1_W::new(self)
    }
    #[doc = "Bit 2 - Wide Timer 2 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppwtimer_p2(&mut self) -> SYSCTL_PPWTIMER_P2_W<PPWTIMER_SPEC, 2> {
        SYSCTL_PPWTIMER_P2_W::new(self)
    }
    #[doc = "Bit 3 - Wide Timer 3 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppwtimer_p3(&mut self) -> SYSCTL_PPWTIMER_P3_W<PPWTIMER_SPEC, 3> {
        SYSCTL_PPWTIMER_P3_W::new(self)
    }
    #[doc = "Bit 4 - Wide Timer 4 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppwtimer_p4(&mut self) -> SYSCTL_PPWTIMER_P4_W<PPWTIMER_SPEC, 4> {
        SYSCTL_PPWTIMER_P4_W::new(self)
    }
    #[doc = "Bit 5 - Wide Timer 5 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppwtimer_p5(&mut self) -> SYSCTL_PPWTIMER_P5_W<PPWTIMER_SPEC, 5> {
        SYSCTL_PPWTIMER_P5_W::new(self)
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
#[doc = "Wide Timer Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppwtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppwtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPWTIMER_SPEC;
impl crate::RegisterSpec for PPWTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppwtimer::R`](R) reader structure"]
impl crate::Readable for PPWTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppwtimer::W`](W) writer structure"]
impl crate::Writable for PPWTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPWTIMER to value 0"]
impl crate::Resettable for PPWTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
