#[doc = "Register `MBMON` reader"]
pub type R = crate::R<MBMON_SPEC>;
#[doc = "Register `MBMON` writer"]
pub type W = crate::W<MBMON_SPEC>;
#[doc = "Field `I2C_MBMON_SCL` reader - I2C SCL Status"]
pub type I2C_MBMON_SCL_R = crate::BitReader;
#[doc = "Field `I2C_MBMON_SCL` writer - I2C SCL Status"]
pub type I2C_MBMON_SCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MBMON_SDA` reader - I2C SDA Status"]
pub type I2C_MBMON_SDA_R = crate::BitReader;
#[doc = "Field `I2C_MBMON_SDA` writer - I2C SDA Status"]
pub type I2C_MBMON_SDA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C SCL Status"]
    #[inline(always)]
    pub fn i2c_mbmon_scl(&self) -> I2C_MBMON_SCL_R {
        I2C_MBMON_SCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C SDA Status"]
    #[inline(always)]
    pub fn i2c_mbmon_sda(&self) -> I2C_MBMON_SDA_R {
        I2C_MBMON_SDA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C SCL Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mbmon_scl(&mut self) -> I2C_MBMON_SCL_W<MBMON_SPEC, 0> {
        I2C_MBMON_SCL_W::new(self)
    }
    #[doc = "Bit 1 - I2C SDA Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mbmon_sda(&mut self) -> I2C_MBMON_SDA_W<MBMON_SPEC, 1> {
        I2C_MBMON_SDA_W::new(self)
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
#[doc = "I2C Master Bus Monitor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbmon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbmon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MBMON_SPEC;
impl crate::RegisterSpec for MBMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbmon::R`](R) reader structure"]
impl crate::Readable for MBMON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mbmon::W`](W) writer structure"]
impl crate::Writable for MBMON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MBMON to value 0"]
impl crate::Resettable for MBMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
