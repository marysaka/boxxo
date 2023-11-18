#[doc = "Register `DCGCGPIO` reader"]
pub type R = crate::R<DCGCGPIO_SPEC>;
#[doc = "Register `DCGCGPIO` writer"]
pub type W = crate::W<DCGCGPIO_SPEC>;
#[doc = "Field `SYSCTL_DCGCGPIO_D0` reader - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCGPIO_D0` writer - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCGPIO_D1` reader - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCGPIO_D1` writer - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCGPIO_D2` reader - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCGPIO_D2` writer - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCGPIO_D3` reader - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCGPIO_D3` writer - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCGPIO_D4` reader - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D4_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCGPIO_D4` writer - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCGPIO_D5` reader - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D5_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCGPIO_D5` writer - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d0(&self) -> SYSCTL_DCGCGPIO_D0_R {
        SYSCTL_DCGCGPIO_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d1(&self) -> SYSCTL_DCGCGPIO_D1_R {
        SYSCTL_DCGCGPIO_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d2(&self) -> SYSCTL_DCGCGPIO_D2_R {
        SYSCTL_DCGCGPIO_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d3(&self) -> SYSCTL_DCGCGPIO_D3_R {
        SYSCTL_DCGCGPIO_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d4(&self) -> SYSCTL_DCGCGPIO_D4_R {
        SYSCTL_DCGCGPIO_D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d5(&self) -> SYSCTL_DCGCGPIO_D5_R {
        SYSCTL_DCGCGPIO_D5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcgpio_d0(&mut self) -> SYSCTL_DCGCGPIO_D0_W<DCGCGPIO_SPEC, 0> {
        SYSCTL_DCGCGPIO_D0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcgpio_d1(&mut self) -> SYSCTL_DCGCGPIO_D1_W<DCGCGPIO_SPEC, 1> {
        SYSCTL_DCGCGPIO_D1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcgpio_d2(&mut self) -> SYSCTL_DCGCGPIO_D2_W<DCGCGPIO_SPEC, 2> {
        SYSCTL_DCGCGPIO_D2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcgpio_d3(&mut self) -> SYSCTL_DCGCGPIO_D3_W<DCGCGPIO_SPEC, 3> {
        SYSCTL_DCGCGPIO_D3_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcgpio_d4(&mut self) -> SYSCTL_DCGCGPIO_D4_W<DCGCGPIO_SPEC, 4> {
        SYSCTL_DCGCGPIO_D4_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcgpio_d5(&mut self) -> SYSCTL_DCGCGPIO_D5_W<DCGCGPIO_SPEC, 5> {
        SYSCTL_DCGCGPIO_D5_W::new(self)
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
#[doc = "General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcgpio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcgpio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCGPIO_SPEC;
impl crate::RegisterSpec for DCGCGPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcgpio::R`](R) reader structure"]
impl crate::Readable for DCGCGPIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgcgpio::W`](W) writer structure"]
impl crate::Writable for DCGCGPIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCGPIO to value 0"]
impl crate::Resettable for DCGCGPIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
