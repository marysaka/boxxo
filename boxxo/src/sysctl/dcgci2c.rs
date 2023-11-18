#[doc = "Register `DCGCI2C` reader"]
pub type R = crate::R<DCGCI2C_SPEC>;
#[doc = "Register `DCGCI2C` writer"]
pub type W = crate::W<DCGCI2C_SPEC>;
#[doc = "Field `SYSCTL_DCGCI2C_D0` reader - I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCI2C_D0` writer - I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCI2C_D1` reader - I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCI2C_D1` writer - I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCI2C_D2` reader - I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCI2C_D2` writer - I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGCI2C_D3` reader - I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCI2C_D3` writer - I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d0(&self) -> SYSCTL_DCGCI2C_D0_R {
        SYSCTL_DCGCI2C_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d1(&self) -> SYSCTL_DCGCI2C_D1_R {
        SYSCTL_DCGCI2C_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d2(&self) -> SYSCTL_DCGCI2C_D2_R {
        SYSCTL_DCGCI2C_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d3(&self) -> SYSCTL_DCGCI2C_D3_R {
        SYSCTL_DCGCI2C_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgci2c_d0(&mut self) -> SYSCTL_DCGCI2C_D0_W<DCGCI2C_SPEC, 0> {
        SYSCTL_DCGCI2C_D0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgci2c_d1(&mut self) -> SYSCTL_DCGCI2C_D1_W<DCGCI2C_SPEC, 1> {
        SYSCTL_DCGCI2C_D1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgci2c_d2(&mut self) -> SYSCTL_DCGCI2C_D2_W<DCGCI2C_SPEC, 2> {
        SYSCTL_DCGCI2C_D2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgci2c_d3(&mut self) -> SYSCTL_DCGCI2C_D3_W<DCGCI2C_SPEC, 3> {
        SYSCTL_DCGCI2C_D3_W::new(self)
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
#[doc = "Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgci2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgci2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCI2C_SPEC;
impl crate::RegisterSpec for DCGCI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgci2c::R`](R) reader structure"]
impl crate::Readable for DCGCI2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgci2c::W`](W) writer structure"]
impl crate::Writable for DCGCI2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCI2C to value 0"]
impl crate::Resettable for DCGCI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
