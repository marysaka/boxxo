#[doc = "Register `SCGCI2C` reader"]
pub type R = crate::R<SCGCI2C_SPEC>;
#[doc = "Register `SCGCI2C` writer"]
pub type W = crate::W<SCGCI2C_SPEC>;
#[doc = "Field `SYSCTL_SCGCI2C_S0` reader - I2C Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCI2C_S0` writer - I2C Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCI2C_S1` reader - I2C Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCI2C_S1` writer - I2C Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCI2C_S2` reader - I2C Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S2_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCI2C_S2` writer - I2C Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCI2C_S3` reader - I2C Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S3_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCI2C_S3` writer - I2C Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s0(&self) -> SYSCTL_SCGCI2C_S0_R {
        SYSCTL_SCGCI2C_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s1(&self) -> SYSCTL_SCGCI2C_S1_R {
        SYSCTL_SCGCI2C_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s2(&self) -> SYSCTL_SCGCI2C_S2_R {
        SYSCTL_SCGCI2C_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s3(&self) -> SYSCTL_SCGCI2C_S3_R {
        SYSCTL_SCGCI2C_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgci2c_s0(&mut self) -> SYSCTL_SCGCI2C_S0_W<SCGCI2C_SPEC, 0> {
        SYSCTL_SCGCI2C_S0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgci2c_s1(&mut self) -> SYSCTL_SCGCI2C_S1_W<SCGCI2C_SPEC, 1> {
        SYSCTL_SCGCI2C_S1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgci2c_s2(&mut self) -> SYSCTL_SCGCI2C_S2_W<SCGCI2C_SPEC, 2> {
        SYSCTL_SCGCI2C_S2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgci2c_s3(&mut self) -> SYSCTL_SCGCI2C_S3_W<SCGCI2C_SPEC, 3> {
        SYSCTL_SCGCI2C_S3_W::new(self)
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
#[doc = "Inter-Integrated Circuit Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgci2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgci2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCI2C_SPEC;
impl crate::RegisterSpec for SCGCI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgci2c::R`](R) reader structure"]
impl crate::Readable for SCGCI2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgci2c::W`](W) writer structure"]
impl crate::Writable for SCGCI2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCI2C to value 0"]
impl crate::Resettable for SCGCI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
