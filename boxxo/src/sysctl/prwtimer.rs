#[doc = "Register `PRWTIMER` reader"]
pub type R = crate::R<PRWTIMER_SPEC>;
#[doc = "Register `PRWTIMER` writer"]
pub type W = crate::W<PRWTIMER_SPEC>;
#[doc = "Field `SYSCTL_PRWTIMER_R0` reader - Wide Timer 0 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRWTIMER_R0` writer - Wide Timer 0 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRWTIMER_R1` reader - Wide Timer 1 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRWTIMER_R1` writer - Wide Timer 1 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRWTIMER_R2` reader - Wide Timer 2 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRWTIMER_R2` writer - Wide Timer 2 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRWTIMER_R3` reader - Wide Timer 3 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRWTIMER_R3` writer - Wide Timer 3 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRWTIMER_R4` reader - Wide Timer 4 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R4_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRWTIMER_R4` writer - Wide Timer 4 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRWTIMER_R5` reader - Wide Timer 5 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R5_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRWTIMER_R5` writer - Wide Timer 5 Peripheral Ready"]
pub type SYSCTL_PRWTIMER_R5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Wide Timer 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwtimer_r0(&self) -> SYSCTL_PRWTIMER_R0_R {
        SYSCTL_PRWTIMER_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwtimer_r1(&self) -> SYSCTL_PRWTIMER_R1_R {
        SYSCTL_PRWTIMER_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 2 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwtimer_r2(&self) -> SYSCTL_PRWTIMER_R2_R {
        SYSCTL_PRWTIMER_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 3 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwtimer_r3(&self) -> SYSCTL_PRWTIMER_R3_R {
        SYSCTL_PRWTIMER_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wide Timer 4 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwtimer_r4(&self) -> SYSCTL_PRWTIMER_R4_R {
        SYSCTL_PRWTIMER_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wide Timer 5 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwtimer_r5(&self) -> SYSCTL_PRWTIMER_R5_R {
        SYSCTL_PRWTIMER_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wide Timer 0 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prwtimer_r0(&mut self) -> SYSCTL_PRWTIMER_R0_W<PRWTIMER_SPEC, 0> {
        SYSCTL_PRWTIMER_R0_W::new(self)
    }
    #[doc = "Bit 1 - Wide Timer 1 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prwtimer_r1(&mut self) -> SYSCTL_PRWTIMER_R1_W<PRWTIMER_SPEC, 1> {
        SYSCTL_PRWTIMER_R1_W::new(self)
    }
    #[doc = "Bit 2 - Wide Timer 2 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prwtimer_r2(&mut self) -> SYSCTL_PRWTIMER_R2_W<PRWTIMER_SPEC, 2> {
        SYSCTL_PRWTIMER_R2_W::new(self)
    }
    #[doc = "Bit 3 - Wide Timer 3 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prwtimer_r3(&mut self) -> SYSCTL_PRWTIMER_R3_W<PRWTIMER_SPEC, 3> {
        SYSCTL_PRWTIMER_R3_W::new(self)
    }
    #[doc = "Bit 4 - Wide Timer 4 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prwtimer_r4(&mut self) -> SYSCTL_PRWTIMER_R4_W<PRWTIMER_SPEC, 4> {
        SYSCTL_PRWTIMER_R4_W::new(self)
    }
    #[doc = "Bit 5 - Wide Timer 5 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prwtimer_r5(&mut self) -> SYSCTL_PRWTIMER_R5_W<PRWTIMER_SPEC, 5> {
        SYSCTL_PRWTIMER_R5_W::new(self)
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
#[doc = "Wide Timer Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prwtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prwtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRWTIMER_SPEC;
impl crate::RegisterSpec for PRWTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prwtimer::R`](R) reader structure"]
impl crate::Readable for PRWTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prwtimer::W`](W) writer structure"]
impl crate::Writable for PRWTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRWTIMER to value 0"]
impl crate::Resettable for PRWTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
