#[doc = "Register `PPI2C` reader"]
pub type R = crate::R<PPI2C_SPEC>;
#[doc = "Register `PPI2C` writer"]
pub type W = crate::W<PPI2C_SPEC>;
#[doc = "Field `SYSCTL_PPI2C_P0` reader - I2C Module 0 Present"]
pub type SYSCTL_PPI2C_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPI2C_P0` writer - I2C Module 0 Present"]
pub type SYSCTL_PPI2C_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPI2C_P1` reader - I2C Module 1 Present"]
pub type SYSCTL_PPI2C_P1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPI2C_P1` writer - I2C Module 1 Present"]
pub type SYSCTL_PPI2C_P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPI2C_P2` reader - I2C Module 2 Present"]
pub type SYSCTL_PPI2C_P2_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPI2C_P2` writer - I2C Module 2 Present"]
pub type SYSCTL_PPI2C_P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPI2C_P3` reader - I2C Module 3 Present"]
pub type SYSCTL_PPI2C_P3_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPI2C_P3` writer - I2C Module 3 Present"]
pub type SYSCTL_PPI2C_P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPI2C_P4` reader - I2C Module 4 Present"]
pub type SYSCTL_PPI2C_P4_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPI2C_P4` writer - I2C Module 4 Present"]
pub type SYSCTL_PPI2C_P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPI2C_P5` reader - I2C Module 5 Present"]
pub type SYSCTL_PPI2C_P5_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPI2C_P5` writer - I2C Module 5 Present"]
pub type SYSCTL_PPI2C_P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p0(&self) -> SYSCTL_PPI2C_P0_R {
        SYSCTL_PPI2C_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p1(&self) -> SYSCTL_PPI2C_P1_R {
        SYSCTL_PPI2C_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p2(&self) -> SYSCTL_PPI2C_P2_R {
        SYSCTL_PPI2C_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p3(&self) -> SYSCTL_PPI2C_P3_R {
        SYSCTL_PPI2C_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Module 4 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p4(&self) -> SYSCTL_PPI2C_P4_R {
        SYSCTL_PPI2C_P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Module 5 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p5(&self) -> SYSCTL_PPI2C_P5_R {
        SYSCTL_PPI2C_P5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppi2c_p0(&mut self) -> SYSCTL_PPI2C_P0_W<PPI2C_SPEC, 0> {
        SYSCTL_PPI2C_P0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppi2c_p1(&mut self) -> SYSCTL_PPI2C_P1_W<PPI2C_SPEC, 1> {
        SYSCTL_PPI2C_P1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Module 2 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppi2c_p2(&mut self) -> SYSCTL_PPI2C_P2_W<PPI2C_SPEC, 2> {
        SYSCTL_PPI2C_P2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Module 3 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppi2c_p3(&mut self) -> SYSCTL_PPI2C_P3_W<PPI2C_SPEC, 3> {
        SYSCTL_PPI2C_P3_W::new(self)
    }
    #[doc = "Bit 4 - I2C Module 4 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppi2c_p4(&mut self) -> SYSCTL_PPI2C_P4_W<PPI2C_SPEC, 4> {
        SYSCTL_PPI2C_P4_W::new(self)
    }
    #[doc = "Bit 5 - I2C Module 5 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppi2c_p5(&mut self) -> SYSCTL_PPI2C_P5_W<PPI2C_SPEC, 5> {
        SYSCTL_PPI2C_P5_W::new(self)
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
#[doc = "Inter-Integrated Circuit Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppi2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppi2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPI2C_SPEC;
impl crate::RegisterSpec for PPI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppi2c::R`](R) reader structure"]
impl crate::Readable for PPI2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppi2c::W`](W) writer structure"]
impl crate::Writable for PPI2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPI2C to value 0"]
impl crate::Resettable for PPI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
