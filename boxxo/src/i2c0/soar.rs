#[doc = "Register `SOAR` reader"]
pub type R = crate::R<SOAR_SPEC>;
#[doc = "Register `SOAR` writer"]
pub type W = crate::W<SOAR_SPEC>;
#[doc = "Field `I2C_SOAR_OAR` reader - I2C Slave Own Address"]
pub type I2C_SOAR_OAR_R = crate::FieldReader;
#[doc = "Field `I2C_SOAR_OAR` writer - I2C Slave Own Address"]
pub type I2C_SOAR_OAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - I2C Slave Own Address"]
    #[inline(always)]
    pub fn i2c_soar_oar(&self) -> I2C_SOAR_OAR_R {
        I2C_SOAR_OAR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C Slave Own Address"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_soar_oar(&mut self) -> I2C_SOAR_OAR_W<SOAR_SPEC, 0> {
        I2C_SOAR_OAR_W::new(self)
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
#[doc = "I2C Slave Own Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOAR_SPEC;
impl crate::RegisterSpec for SOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soar::R`](R) reader structure"]
impl crate::Readable for SOAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soar::W`](W) writer structure"]
impl crate::Writable for SOAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOAR to value 0"]
impl crate::Resettable for SOAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
