#[doc = "Register `SDR` reader"]
pub type R = crate::R<SDR_SPEC>;
#[doc = "Register `SDR` writer"]
pub type W = crate::W<SDR_SPEC>;
#[doc = "Field `I2C_SDR_DATA` reader - Data for Transfer"]
pub type I2C_SDR_DATA_R = crate::FieldReader;
#[doc = "Field `I2C_SDR_DATA` writer - Data for Transfer"]
pub type I2C_SDR_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data for Transfer"]
    #[inline(always)]
    pub fn i2c_sdr_data(&self) -> I2C_SDR_DATA_R {
        I2C_SDR_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data for Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sdr_data(&mut self) -> I2C_SDR_DATA_W<SDR_SPEC, 0> {
        I2C_SDR_DATA_W::new(self)
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
#[doc = "I2C Slave Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDR_SPEC;
impl crate::RegisterSpec for SDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdr::R`](R) reader structure"]
impl crate::Readable for SDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdr::W`](W) writer structure"]
impl crate::Writable for SDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDR to value 0"]
impl crate::Resettable for SDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
