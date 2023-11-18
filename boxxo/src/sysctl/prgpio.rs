#[doc = "Register `PRGPIO` reader"]
pub type R = crate::R<PRGPIO_SPEC>;
#[doc = "Register `PRGPIO` writer"]
pub type W = crate::W<PRGPIO_SPEC>;
#[doc = "Field `SYSCTL_PRGPIO_R0` reader - GPIO Port A Peripheral Ready"]
pub type SYSCTL_PRGPIO_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRGPIO_R0` writer - GPIO Port A Peripheral Ready"]
pub type SYSCTL_PRGPIO_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRGPIO_R1` reader - GPIO Port B Peripheral Ready"]
pub type SYSCTL_PRGPIO_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRGPIO_R1` writer - GPIO Port B Peripheral Ready"]
pub type SYSCTL_PRGPIO_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRGPIO_R2` reader - GPIO Port C Peripheral Ready"]
pub type SYSCTL_PRGPIO_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRGPIO_R2` writer - GPIO Port C Peripheral Ready"]
pub type SYSCTL_PRGPIO_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRGPIO_R3` reader - GPIO Port D Peripheral Ready"]
pub type SYSCTL_PRGPIO_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRGPIO_R3` writer - GPIO Port D Peripheral Ready"]
pub type SYSCTL_PRGPIO_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRGPIO_R4` reader - GPIO Port E Peripheral Ready"]
pub type SYSCTL_PRGPIO_R4_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRGPIO_R4` writer - GPIO Port E Peripheral Ready"]
pub type SYSCTL_PRGPIO_R4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRGPIO_R5` reader - GPIO Port F Peripheral Ready"]
pub type SYSCTL_PRGPIO_R5_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRGPIO_R5` writer - GPIO Port F Peripheral Ready"]
pub type SYSCTL_PRGPIO_R5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r0(&self) -> SYSCTL_PRGPIO_R0_R {
        SYSCTL_PRGPIO_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r1(&self) -> SYSCTL_PRGPIO_R1_R {
        SYSCTL_PRGPIO_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r2(&self) -> SYSCTL_PRGPIO_R2_R {
        SYSCTL_PRGPIO_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r3(&self) -> SYSCTL_PRGPIO_R3_R {
        SYSCTL_PRGPIO_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r4(&self) -> SYSCTL_PRGPIO_R4_R {
        SYSCTL_PRGPIO_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prgpio_r5(&self) -> SYSCTL_PRGPIO_R5_R {
        SYSCTL_PRGPIO_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prgpio_r0(&mut self) -> SYSCTL_PRGPIO_R0_W<PRGPIO_SPEC, 0> {
        SYSCTL_PRGPIO_R0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prgpio_r1(&mut self) -> SYSCTL_PRGPIO_R1_W<PRGPIO_SPEC, 1> {
        SYSCTL_PRGPIO_R1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prgpio_r2(&mut self) -> SYSCTL_PRGPIO_R2_W<PRGPIO_SPEC, 2> {
        SYSCTL_PRGPIO_R2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prgpio_r3(&mut self) -> SYSCTL_PRGPIO_R3_W<PRGPIO_SPEC, 3> {
        SYSCTL_PRGPIO_R3_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prgpio_r4(&mut self) -> SYSCTL_PRGPIO_R4_W<PRGPIO_SPEC, 4> {
        SYSCTL_PRGPIO_R4_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prgpio_r5(&mut self) -> SYSCTL_PRGPIO_R5_W<PRGPIO_SPEC, 5> {
        SYSCTL_PRGPIO_R5_W::new(self)
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
#[doc = "General-Purpose Input/Output Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prgpio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prgpio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRGPIO_SPEC;
impl crate::RegisterSpec for PRGPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prgpio::R`](R) reader structure"]
impl crate::Readable for PRGPIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prgpio::W`](W) writer structure"]
impl crate::Writable for PRGPIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRGPIO to value 0"]
impl crate::Resettable for PRGPIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
