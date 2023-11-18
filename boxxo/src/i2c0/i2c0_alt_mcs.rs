#[doc = "Register `MCS` reader"]
pub type R = crate::R<I2C0_ALT_MCS_SPEC>;
#[doc = "Register `MCS` writer"]
pub type W = crate::W<I2C0_ALT_MCS_SPEC>;
#[doc = "Field `I2C_MCS_BUSY` reader - I2C Busy"]
pub type I2C_MCS_BUSY_R = crate::BitReader;
#[doc = "Field `I2C_MCS_BUSY` writer - I2C Busy"]
pub type I2C_MCS_BUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_ERROR` reader - Error"]
pub type I2C_MCS_ERROR_R = crate::BitReader;
#[doc = "Field `I2C_MCS_ERROR` writer - Error"]
pub type I2C_MCS_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_STOP` reader - Generate STOP"]
pub type I2C_MCS_STOP_R = crate::BitReader;
#[doc = "Field `I2C_MCS_STOP` writer - Generate STOP"]
pub type I2C_MCS_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_DATACK` reader - Acknowledge Data"]
pub type I2C_MCS_DATACK_R = crate::BitReader;
#[doc = "Field `I2C_MCS_DATACK` writer - Acknowledge Data"]
pub type I2C_MCS_DATACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_HS` reader - High-Speed Enable"]
pub type I2C_MCS_HS_R = crate::BitReader;
#[doc = "Field `I2C_MCS_HS` writer - High-Speed Enable"]
pub type I2C_MCS_HS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_BUSBSY` reader - Bus Busy"]
pub type I2C_MCS_BUSBSY_R = crate::BitReader;
#[doc = "Field `I2C_MCS_BUSBSY` writer - Bus Busy"]
pub type I2C_MCS_BUSBSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busy(&self) -> I2C_MCS_BUSY_R {
        I2C_MCS_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn i2c_mcs_error(&self) -> I2C_MCS_ERROR_R {
        I2C_MCS_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn i2c_mcs_stop(&self) -> I2C_MCS_STOP_R {
        I2C_MCS_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    pub fn i2c_mcs_datack(&self) -> I2C_MCS_DATACK_R {
        I2C_MCS_DATACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High-Speed Enable"]
    #[inline(always)]
    pub fn i2c_mcs_hs(&self) -> I2C_MCS_HS_R {
        I2C_MCS_HS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Bus Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busbsy(&self) -> I2C_MCS_BUSBSY_R {
        I2C_MCS_BUSBSY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Busy"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_busy(&mut self) -> I2C_MCS_BUSY_W<I2C0_ALT_MCS_SPEC, 0> {
        I2C_MCS_BUSY_W::new(self)
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_error(&mut self) -> I2C_MCS_ERROR_W<I2C0_ALT_MCS_SPEC, 1> {
        I2C_MCS_ERROR_W::new(self)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_stop(&mut self) -> I2C_MCS_STOP_W<I2C0_ALT_MCS_SPEC, 2> {
        I2C_MCS_STOP_W::new(self)
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_datack(&mut self) -> I2C_MCS_DATACK_W<I2C0_ALT_MCS_SPEC, 3> {
        I2C_MCS_DATACK_W::new(self)
    }
    #[doc = "Bit 4 - High-Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_hs(&mut self) -> I2C_MCS_HS_W<I2C0_ALT_MCS_SPEC, 4> {
        I2C_MCS_HS_W::new(self)
    }
    #[doc = "Bit 6 - Bus Busy"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_busbsy(&mut self) -> I2C_MCS_BUSBSY_W<I2C0_ALT_MCS_SPEC, 6> {
        I2C_MCS_BUSBSY_W::new(self)
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
#[doc = "I2C Master Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_alt_mcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_alt_mcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C0_ALT_MCS_SPEC;
impl crate::RegisterSpec for I2C0_ALT_MCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_alt_mcs::R`](R) reader structure"]
impl crate::Readable for I2C0_ALT_MCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c0_alt_mcs::W`](W) writer structure"]
impl crate::Writable for I2C0_ALT_MCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCS to value 0"]
impl crate::Resettable for I2C0_ALT_MCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
