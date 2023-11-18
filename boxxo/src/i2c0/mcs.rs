#[doc = "Register `MCS` reader"]
pub type R = crate::R<MCS_SPEC>;
#[doc = "Register `MCS` writer"]
pub type W = crate::W<MCS_SPEC>;
#[doc = "Field `I2C_MCS_RUN` reader - I2C Master Enable"]
pub type I2C_MCS_RUN_R = crate::BitReader;
#[doc = "Field `I2C_MCS_RUN` writer - I2C Master Enable"]
pub type I2C_MCS_RUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_START` reader - Generate START"]
pub type I2C_MCS_START_R = crate::BitReader;
#[doc = "Field `I2C_MCS_START` writer - Generate START"]
pub type I2C_MCS_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_ADRACK` reader - Acknowledge Address"]
pub type I2C_MCS_ADRACK_R = crate::BitReader;
#[doc = "Field `I2C_MCS_ADRACK` writer - Acknowledge Address"]
pub type I2C_MCS_ADRACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_ACK` reader - Data Acknowledge Enable"]
pub type I2C_MCS_ACK_R = crate::BitReader;
#[doc = "Field `I2C_MCS_ACK` writer - Data Acknowledge Enable"]
pub type I2C_MCS_ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_ARBLST` reader - Arbitration Lost"]
pub type I2C_MCS_ARBLST_R = crate::BitReader;
#[doc = "Field `I2C_MCS_ARBLST` writer - Arbitration Lost"]
pub type I2C_MCS_ARBLST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_IDLE` reader - I2C Idle"]
pub type I2C_MCS_IDLE_R = crate::BitReader;
#[doc = "Field `I2C_MCS_IDLE` writer - I2C Idle"]
pub type I2C_MCS_IDLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCS_CLKTO` reader - Clock Timeout Error"]
pub type I2C_MCS_CLKTO_R = crate::BitReader;
#[doc = "Field `I2C_MCS_CLKTO` writer - Clock Timeout Error"]
pub type I2C_MCS_CLKTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C Master Enable"]
    #[inline(always)]
    pub fn i2c_mcs_run(&self) -> I2C_MCS_RUN_R {
        I2C_MCS_RUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn i2c_mcs_start(&self) -> I2C_MCS_START_R {
        I2C_MCS_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge Address"]
    #[inline(always)]
    pub fn i2c_mcs_adrack(&self) -> I2C_MCS_ADRACK_R {
        I2C_MCS_ADRACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable"]
    #[inline(always)]
    pub fn i2c_mcs_ack(&self) -> I2C_MCS_ACK_R {
        I2C_MCS_ACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn i2c_mcs_arblst(&self) -> I2C_MCS_ARBLST_R {
        I2C_MCS_ARBLST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Idle"]
    #[inline(always)]
    pub fn i2c_mcs_idle(&self) -> I2C_MCS_IDLE_R {
        I2C_MCS_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Timeout Error"]
    #[inline(always)]
    pub fn i2c_mcs_clkto(&self) -> I2C_MCS_CLKTO_R {
        I2C_MCS_CLKTO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Master Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_run(&mut self) -> I2C_MCS_RUN_W<MCS_SPEC, 0> {
        I2C_MCS_RUN_W::new(self)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_start(&mut self) -> I2C_MCS_START_W<MCS_SPEC, 1> {
        I2C_MCS_START_W::new(self)
    }
    #[doc = "Bit 2 - Acknowledge Address"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_adrack(&mut self) -> I2C_MCS_ADRACK_W<MCS_SPEC, 2> {
        I2C_MCS_ADRACK_W::new(self)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_ack(&mut self) -> I2C_MCS_ACK_W<MCS_SPEC, 3> {
        I2C_MCS_ACK_W::new(self)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_arblst(&mut self) -> I2C_MCS_ARBLST_W<MCS_SPEC, 4> {
        I2C_MCS_ARBLST_W::new(self)
    }
    #[doc = "Bit 5 - I2C Idle"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_idle(&mut self) -> I2C_MCS_IDLE_W<MCS_SPEC, 5> {
        I2C_MCS_IDLE_W::new(self)
    }
    #[doc = "Bit 7 - Clock Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcs_clkto(&mut self) -> I2C_MCS_CLKTO_W<MCS_SPEC, 7> {
        I2C_MCS_CLKTO_W::new(self)
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
#[doc = "I2C Master Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCS_SPEC;
impl crate::RegisterSpec for MCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcs::R`](R) reader structure"]
impl crate::Readable for MCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcs::W`](W) writer structure"]
impl crate::Writable for MCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCS to value 0"]
impl crate::Resettable for MCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
