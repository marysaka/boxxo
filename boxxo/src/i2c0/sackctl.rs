#[doc = "Register `SACKCTL` reader"]
pub type R = crate::R<SACKCTL_SPEC>;
#[doc = "Register `SACKCTL` writer"]
pub type W = crate::W<SACKCTL_SPEC>;
#[doc = "Field `I2C_SACKCTL_ACKOEN` reader - I2C Slave ACK Override Enable"]
pub type I2C_SACKCTL_ACKOEN_R = crate::BitReader;
#[doc = "Field `I2C_SACKCTL_ACKOEN` writer - I2C Slave ACK Override Enable"]
pub type I2C_SACKCTL_ACKOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SACKCTL_ACKOVAL` reader - I2C Slave ACK Override Value"]
pub type I2C_SACKCTL_ACKOVAL_R = crate::BitReader;
#[doc = "Field `I2C_SACKCTL_ACKOVAL` writer - I2C Slave ACK Override Value"]
pub type I2C_SACKCTL_ACKOVAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C Slave ACK Override Enable"]
    #[inline(always)]
    pub fn i2c_sackctl_ackoen(&self) -> I2C_SACKCTL_ACKOEN_R {
        I2C_SACKCTL_ACKOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Slave ACK Override Value"]
    #[inline(always)]
    pub fn i2c_sackctl_ackoval(&self) -> I2C_SACKCTL_ACKOVAL_R {
        I2C_SACKCTL_ACKOVAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Slave ACK Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sackctl_ackoen(&mut self) -> I2C_SACKCTL_ACKOEN_W<SACKCTL_SPEC, 0> {
        I2C_SACKCTL_ACKOEN_W::new(self)
    }
    #[doc = "Bit 1 - I2C Slave ACK Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sackctl_ackoval(&mut self) -> I2C_SACKCTL_ACKOVAL_W<SACKCTL_SPEC, 1> {
        I2C_SACKCTL_ACKOVAL_W::new(self)
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
#[doc = "I2C Slave ACK Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sackctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sackctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SACKCTL_SPEC;
impl crate::RegisterSpec for SACKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sackctl::R`](R) reader structure"]
impl crate::Readable for SACKCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sackctl::W`](W) writer structure"]
impl crate::Writable for SACKCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SACKCTL to value 0"]
impl crate::Resettable for SACKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
