#[doc = "Register `PRI2C` reader"]
pub type R = crate::R<PRI2C_SPEC>;
#[doc = "Register `PRI2C` writer"]
pub type W = crate::W<PRI2C_SPEC>;
#[doc = "Field `SYSCTL_PRI2C_R0` reader - I2C Module 0 Peripheral Ready"]
pub type SYSCTL_PRI2C_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRI2C_R0` writer - I2C Module 0 Peripheral Ready"]
pub type SYSCTL_PRI2C_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRI2C_R1` reader - I2C Module 1 Peripheral Ready"]
pub type SYSCTL_PRI2C_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRI2C_R1` writer - I2C Module 1 Peripheral Ready"]
pub type SYSCTL_PRI2C_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRI2C_R2` reader - I2C Module 2 Peripheral Ready"]
pub type SYSCTL_PRI2C_R2_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRI2C_R2` writer - I2C Module 2 Peripheral Ready"]
pub type SYSCTL_PRI2C_R2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PRI2C_R3` reader - I2C Module 3 Peripheral Ready"]
pub type SYSCTL_PRI2C_R3_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRI2C_R3` writer - I2C Module 3 Peripheral Ready"]
pub type SYSCTL_PRI2C_R3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_pri2c_r0(&self) -> SYSCTL_PRI2C_R0_R {
        SYSCTL_PRI2C_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_pri2c_r1(&self) -> SYSCTL_PRI2C_R1_R {
        SYSCTL_PRI2C_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_pri2c_r2(&self) -> SYSCTL_PRI2C_R2_R {
        SYSCTL_PRI2C_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_pri2c_r3(&self) -> SYSCTL_PRI2C_R3_R {
        SYSCTL_PRI2C_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pri2c_r0(&mut self) -> SYSCTL_PRI2C_R0_W<PRI2C_SPEC, 0> {
        SYSCTL_PRI2C_R0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Module 1 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pri2c_r1(&mut self) -> SYSCTL_PRI2C_R1_W<PRI2C_SPEC, 1> {
        SYSCTL_PRI2C_R1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Module 2 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pri2c_r2(&mut self) -> SYSCTL_PRI2C_R2_W<PRI2C_SPEC, 2> {
        SYSCTL_PRI2C_R2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Module 3 Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pri2c_r3(&mut self) -> SYSCTL_PRI2C_R3_W<PRI2C_SPEC, 3> {
        SYSCTL_PRI2C_R3_W::new(self)
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
#[doc = "Inter-Integrated Circuit Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pri2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pri2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRI2C_SPEC;
impl crate::RegisterSpec for PRI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pri2c::R`](R) reader structure"]
impl crate::Readable for PRI2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pri2c::W`](W) writer structure"]
impl crate::Writable for PRI2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRI2C to value 0"]
impl crate::Resettable for PRI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
