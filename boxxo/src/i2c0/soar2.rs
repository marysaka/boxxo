#[doc = "Register `SOAR2` reader"]
pub type R = crate::R<SOAR2_SPEC>;
#[doc = "Register `SOAR2` writer"]
pub type W = crate::W<SOAR2_SPEC>;
#[doc = "Field `I2C_SOAR2_OAR2` reader - I2C Slave Own Address 2"]
pub type I2C_SOAR2_OAR2_R = crate::FieldReader;
#[doc = "Field `I2C_SOAR2_OAR2` writer - I2C Slave Own Address 2"]
pub type I2C_SOAR2_OAR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `I2C_SOAR2_OAR2EN` reader - I2C Slave Own Address 2 Enable"]
pub type I2C_SOAR2_OAR2EN_R = crate::BitReader;
#[doc = "Field `I2C_SOAR2_OAR2EN` writer - I2C Slave Own Address 2 Enable"]
pub type I2C_SOAR2_OAR2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - I2C Slave Own Address 2"]
    #[inline(always)]
    pub fn i2c_soar2_oar2(&self) -> I2C_SOAR2_OAR2_R {
        I2C_SOAR2_OAR2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - I2C Slave Own Address 2 Enable"]
    #[inline(always)]
    pub fn i2c_soar2_oar2en(&self) -> I2C_SOAR2_OAR2EN_R {
        I2C_SOAR2_OAR2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C Slave Own Address 2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_soar2_oar2(&mut self) -> I2C_SOAR2_OAR2_W<SOAR2_SPEC, 0> {
        I2C_SOAR2_OAR2_W::new(self)
    }
    #[doc = "Bit 7 - I2C Slave Own Address 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_soar2_oar2en(&mut self) -> I2C_SOAR2_OAR2EN_W<SOAR2_SPEC, 7> {
        I2C_SOAR2_OAR2EN_W::new(self)
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
#[doc = "I2C Slave Own Address 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOAR2_SPEC;
impl crate::RegisterSpec for SOAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soar2::R`](R) reader structure"]
impl crate::Readable for SOAR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soar2::W`](W) writer structure"]
impl crate::Writable for SOAR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOAR2 to value 0"]
impl crate::Resettable for SOAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
