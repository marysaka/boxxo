#[doc = "Register `SRGPIO` reader"]
pub type R = crate::R<SRGPIO_SPEC>;
#[doc = "Register `SRGPIO` writer"]
pub type W = crate::W<SRGPIO_SPEC>;
#[doc = "Field `SYSCTL_SRGPIO_R0` reader - GPIO Port A Software Reset"]
pub type SYSCTL_SRGPIO_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRGPIO_R0` writer - GPIO Port A Software Reset"]
pub type SYSCTL_SRGPIO_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRGPIO_R1` reader - GPIO Port B Software Reset"]
pub type SYSCTL_SRGPIO_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRGPIO_R1` writer - GPIO Port B Software Reset"]
pub type SYSCTL_SRGPIO_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRGPIO_R2` reader - GPIO Port C Software Reset"]
pub type SYSCTL_SRGPIO_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRGPIO_R2` writer - GPIO Port C Software Reset"]
pub type SYSCTL_SRGPIO_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRGPIO_R3` reader - GPIO Port D Software Reset"]
pub type SYSCTL_SRGPIO_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRGPIO_R3` writer - GPIO Port D Software Reset"]
pub type SYSCTL_SRGPIO_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRGPIO_R4` reader - GPIO Port E Software Reset"]
pub type SYSCTL_SRGPIO_R4_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRGPIO_R4` writer - GPIO Port E Software Reset"]
pub type SYSCTL_SRGPIO_R4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRGPIO_R5` reader - GPIO Port F Software Reset"]
pub type SYSCTL_SRGPIO_R5_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRGPIO_R5` writer - GPIO Port F Software Reset"]
pub type SYSCTL_SRGPIO_R5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r0(&self) -> SYSCTL_SRGPIO_R0_R {
        SYSCTL_SRGPIO_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r1(&self) -> SYSCTL_SRGPIO_R1_R {
        SYSCTL_SRGPIO_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r2(&self) -> SYSCTL_SRGPIO_R2_R {
        SYSCTL_SRGPIO_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r3(&self) -> SYSCTL_SRGPIO_R3_R {
        SYSCTL_SRGPIO_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r4(&self) -> SYSCTL_SRGPIO_R4_R {
        SYSCTL_SRGPIO_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r5(&self) -> SYSCTL_SRGPIO_R5_R {
        SYSCTL_SRGPIO_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srgpio_r0(&mut self) -> SYSCTL_SRGPIO_R0_W<SRGPIO_SPEC, 0> {
        SYSCTL_SRGPIO_R0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srgpio_r1(&mut self) -> SYSCTL_SRGPIO_R1_W<SRGPIO_SPEC, 1> {
        SYSCTL_SRGPIO_R1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srgpio_r2(&mut self) -> SYSCTL_SRGPIO_R2_W<SRGPIO_SPEC, 2> {
        SYSCTL_SRGPIO_R2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srgpio_r3(&mut self) -> SYSCTL_SRGPIO_R3_W<SRGPIO_SPEC, 3> {
        SYSCTL_SRGPIO_R3_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srgpio_r4(&mut self) -> SYSCTL_SRGPIO_R4_W<SRGPIO_SPEC, 4> {
        SYSCTL_SRGPIO_R4_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srgpio_r5(&mut self) -> SYSCTL_SRGPIO_R5_W<SRGPIO_SPEC, 5> {
        SYSCTL_SRGPIO_R5_W::new(self)
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
#[doc = "General-Purpose Input/Output Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srgpio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srgpio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRGPIO_SPEC;
impl crate::RegisterSpec for SRGPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srgpio::R`](R) reader structure"]
impl crate::Readable for SRGPIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srgpio::W`](W) writer structure"]
impl crate::Writable for SRGPIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRGPIO to value 0"]
impl crate::Resettable for SRGPIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
