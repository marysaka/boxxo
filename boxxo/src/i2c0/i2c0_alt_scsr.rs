#[doc = "Register `SCSR` reader"]
pub type R = crate::R<I2C0_ALT_SCSR_SPEC>;
#[doc = "Register `SCSR` writer"]
pub type W = crate::W<I2C0_ALT_SCSR_SPEC>;
#[doc = "Field `I2C_SCSR_DA` reader - Device Active"]
pub type I2C_SCSR_DA_R = crate::BitReader;
#[doc = "Field `I2C_SCSR_DA` writer - Device Active"]
pub type I2C_SCSR_DA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SCSR_TREQ` reader - Transmit Request"]
pub type I2C_SCSR_TREQ_R = crate::BitReader;
#[doc = "Field `I2C_SCSR_TREQ` writer - Transmit Request"]
pub type I2C_SCSR_TREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Device Active"]
    #[inline(always)]
    pub fn i2c_scsr_da(&self) -> I2C_SCSR_DA_R {
        I2C_SCSR_DA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Request"]
    #[inline(always)]
    pub fn i2c_scsr_treq(&self) -> I2C_SCSR_TREQ_R {
        I2C_SCSR_TREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device Active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scsr_da(&mut self) -> I2C_SCSR_DA_W<I2C0_ALT_SCSR_SPEC, 0> {
        I2C_SCSR_DA_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Request"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scsr_treq(&mut self) -> I2C_SCSR_TREQ_W<I2C0_ALT_SCSR_SPEC, 1> {
        I2C_SCSR_TREQ_W::new(self)
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
#[doc = "I2C Slave Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_alt_scsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_alt_scsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C0_ALT_SCSR_SPEC;
impl crate::RegisterSpec for I2C0_ALT_SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_alt_scsr::R`](R) reader structure"]
impl crate::Readable for I2C0_ALT_SCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c0_alt_scsr::W`](W) writer structure"]
impl crate::Writable for I2C0_ALT_SCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for I2C0_ALT_SCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
