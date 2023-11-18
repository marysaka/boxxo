#[doc = "Register `SCGCGPIO` reader"]
pub type R = crate::R<SCGCGPIO_SPEC>;
#[doc = "Register `SCGCGPIO` writer"]
pub type W = crate::W<SCGCGPIO_SPEC>;
#[doc = "Field `SYSCTL_SCGCGPIO_S0` reader - GPIO Port A Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCGPIO_S0` writer - GPIO Port A Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCGPIO_S1` reader - GPIO Port B Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCGPIO_S1` writer - GPIO Port B Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCGPIO_S2` reader - GPIO Port C Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCGPIO_S2` writer - GPIO Port C Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCGPIO_S3` reader - GPIO Port D Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCGPIO_S3` writer - GPIO Port D Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCGPIO_S4` reader - GPIO Port E Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S4_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCGPIO_S4` writer - GPIO Port E Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCGPIO_S5` reader - GPIO Port F Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S5_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCGPIO_S5` writer - GPIO Port F Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCGPIO_S5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s0(&self) -> SYSCTL_SCGCGPIO_S0_R {
        SYSCTL_SCGCGPIO_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s1(&self) -> SYSCTL_SCGCGPIO_S1_R {
        SYSCTL_SCGCGPIO_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s2(&self) -> SYSCTL_SCGCGPIO_S2_R {
        SYSCTL_SCGCGPIO_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s3(&self) -> SYSCTL_SCGCGPIO_S3_R {
        SYSCTL_SCGCGPIO_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s4(&self) -> SYSCTL_SCGCGPIO_S4_R {
        SYSCTL_SCGCGPIO_S4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s5(&self) -> SYSCTL_SCGCGPIO_S5_R {
        SYSCTL_SCGCGPIO_S5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcgpio_s0(&mut self) -> SYSCTL_SCGCGPIO_S0_W<SCGCGPIO_SPEC, 0> {
        SYSCTL_SCGCGPIO_S0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcgpio_s1(&mut self) -> SYSCTL_SCGCGPIO_S1_W<SCGCGPIO_SPEC, 1> {
        SYSCTL_SCGCGPIO_S1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcgpio_s2(&mut self) -> SYSCTL_SCGCGPIO_S2_W<SCGCGPIO_SPEC, 2> {
        SYSCTL_SCGCGPIO_S2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcgpio_s3(&mut self) -> SYSCTL_SCGCGPIO_S3_W<SCGCGPIO_SPEC, 3> {
        SYSCTL_SCGCGPIO_S3_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcgpio_s4(&mut self) -> SYSCTL_SCGCGPIO_S4_W<SCGCGPIO_SPEC, 4> {
        SYSCTL_SCGCGPIO_S4_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcgpio_s5(&mut self) -> SYSCTL_SCGCGPIO_S5_W<SCGCGPIO_SPEC, 5> {
        SYSCTL_SCGCGPIO_S5_W::new(self)
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
#[doc = "General-Purpose Input/Output Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcgpio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcgpio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCGPIO_SPEC;
impl crate::RegisterSpec for SCGCGPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcgpio::R`](R) reader structure"]
impl crate::Readable for SCGCGPIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcgpio::W`](W) writer structure"]
impl crate::Writable for SCGCGPIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCGPIO to value 0"]
impl crate::Resettable for SCGCGPIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
