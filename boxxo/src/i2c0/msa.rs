#[doc = "Register `MSA` reader"]
pub type R = crate::R<MSA_SPEC>;
#[doc = "Register `MSA` writer"]
pub type W = crate::W<MSA_SPEC>;
#[doc = "Field `I2C_MSA_RS` reader - Receive not send"]
pub type I2C_MSA_RS_R = crate::BitReader;
#[doc = "Field `I2C_MSA_RS` writer - Receive not send"]
pub type I2C_MSA_RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MSA_SA` reader - I2C Slave Address"]
pub type I2C_MSA_SA_R = crate::FieldReader;
#[doc = "Field `I2C_MSA_SA` writer - I2C Slave Address"]
pub type I2C_MSA_SA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bit 0 - Receive not send"]
    #[inline(always)]
    pub fn i2c_msa_rs(&self) -> I2C_MSA_RS_R {
        I2C_MSA_RS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - I2C Slave Address"]
    #[inline(always)]
    pub fn i2c_msa_sa(&self) -> I2C_MSA_SA_R {
        I2C_MSA_SA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive not send"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_msa_rs(&mut self) -> I2C_MSA_RS_W<MSA_SPEC, 0> {
        I2C_MSA_RS_W::new(self)
    }
    #[doc = "Bits 1:7 - I2C Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_msa_sa(&mut self) -> I2C_MSA_SA_W<MSA_SPEC, 1> {
        I2C_MSA_SA_W::new(self)
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
#[doc = "I2C Master Slave Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSA_SPEC;
impl crate::RegisterSpec for MSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msa::R`](R) reader structure"]
impl crate::Readable for MSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msa::W`](W) writer structure"]
impl crate::Writable for MSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSA to value 0"]
impl crate::Resettable for MSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
