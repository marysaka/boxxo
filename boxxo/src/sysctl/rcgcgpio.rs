#[doc = "Register `RCGCGPIO` reader"]
pub type R = crate::R<RCGCGPIO_SPEC>;
#[doc = "Register `RCGCGPIO` writer"]
pub type W = crate::W<RCGCGPIO_SPEC>;
#[doc = "Field `SYSCTL_RCGCGPIO_R0` reader - GPIO Port A Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCGPIO_R0` writer - GPIO Port A Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCGPIO_R1` reader - GPIO Port B Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCGPIO_R1` writer - GPIO Port B Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCGPIO_R2` reader - GPIO Port C Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCGPIO_R2` writer - GPIO Port C Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCGPIO_R3` reader - GPIO Port D Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCGPIO_R3` writer - GPIO Port D Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCGPIO_R4` reader - GPIO Port E Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R4_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCGPIO_R4` writer - GPIO Port E Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGCGPIO_R5` reader - GPIO Port F Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R5_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGCGPIO_R5` writer - GPIO Port F Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCGPIO_R5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcgpio_r0(&self) -> SYSCTL_RCGCGPIO_R0_R {
        SYSCTL_RCGCGPIO_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcgpio_r1(&self) -> SYSCTL_RCGCGPIO_R1_R {
        SYSCTL_RCGCGPIO_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcgpio_r2(&self) -> SYSCTL_RCGCGPIO_R2_R {
        SYSCTL_RCGCGPIO_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcgpio_r3(&self) -> SYSCTL_RCGCGPIO_R3_R {
        SYSCTL_RCGCGPIO_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcgpio_r4(&self) -> SYSCTL_RCGCGPIO_R4_R {
        SYSCTL_RCGCGPIO_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcgpio_r5(&self) -> SYSCTL_RCGCGPIO_R5_R {
        SYSCTL_RCGCGPIO_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcgpio_r0(&mut self) -> SYSCTL_RCGCGPIO_R0_W<RCGCGPIO_SPEC, 0> {
        SYSCTL_RCGCGPIO_R0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcgpio_r1(&mut self) -> SYSCTL_RCGCGPIO_R1_W<RCGCGPIO_SPEC, 1> {
        SYSCTL_RCGCGPIO_R1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcgpio_r2(&mut self) -> SYSCTL_RCGCGPIO_R2_W<RCGCGPIO_SPEC, 2> {
        SYSCTL_RCGCGPIO_R2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcgpio_r3(&mut self) -> SYSCTL_RCGCGPIO_R3_W<RCGCGPIO_SPEC, 3> {
        SYSCTL_RCGCGPIO_R3_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcgpio_r4(&mut self) -> SYSCTL_RCGCGPIO_R4_W<RCGCGPIO_SPEC, 4> {
        SYSCTL_RCGCGPIO_R4_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Run Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgcgpio_r5(&mut self) -> SYSCTL_RCGCGPIO_R5_W<RCGCGPIO_SPEC, 5> {
        SYSCTL_RCGCGPIO_R5_W::new(self)
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
#[doc = "General-Purpose Input/Output Run Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcgpio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcgpio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGCGPIO_SPEC;
impl crate::RegisterSpec for RCGCGPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgcgpio::R`](R) reader structure"]
impl crate::Readable for RCGCGPIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgcgpio::W`](W) writer structure"]
impl crate::Writable for RCGCGPIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCGPIO to value 0"]
impl crate::Resettable for RCGCGPIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
