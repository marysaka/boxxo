#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCR_SPEC>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCR_SPEC>;
#[doc = "Field `I2C_MCR_LPBK` reader - I2C Loopback"]
pub type I2C_MCR_LPBK_R = crate::BitReader;
#[doc = "Field `I2C_MCR_LPBK` writer - I2C Loopback"]
pub type I2C_MCR_LPBK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCR_MFE` reader - I2C Master Function Enable"]
pub type I2C_MCR_MFE_R = crate::BitReader;
#[doc = "Field `I2C_MCR_MFE` writer - I2C Master Function Enable"]
pub type I2C_MCR_MFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCR_SFE` reader - I2C Slave Function Enable"]
pub type I2C_MCR_SFE_R = crate::BitReader;
#[doc = "Field `I2C_MCR_SFE` writer - I2C Slave Function Enable"]
pub type I2C_MCR_SFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MCR_GFE` reader - I2C Glitch Filter Enable"]
pub type I2C_MCR_GFE_R = crate::BitReader;
#[doc = "Field `I2C_MCR_GFE` writer - I2C Glitch Filter Enable"]
pub type I2C_MCR_GFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C Loopback"]
    #[inline(always)]
    pub fn i2c_mcr_lpbk(&self) -> I2C_MCR_LPBK_R {
        I2C_MCR_LPBK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Master Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_mfe(&self) -> I2C_MCR_MFE_R {
        I2C_MCR_MFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Slave Function Enable"]
    #[inline(always)]
    pub fn i2c_mcr_sfe(&self) -> I2C_MCR_SFE_R {
        I2C_MCR_SFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Glitch Filter Enable"]
    #[inline(always)]
    pub fn i2c_mcr_gfe(&self) -> I2C_MCR_GFE_R {
        I2C_MCR_GFE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Loopback"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcr_lpbk(&mut self) -> I2C_MCR_LPBK_W<MCR_SPEC, 0> {
        I2C_MCR_LPBK_W::new(self)
    }
    #[doc = "Bit 4 - I2C Master Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcr_mfe(&mut self) -> I2C_MCR_MFE_W<MCR_SPEC, 4> {
        I2C_MCR_MFE_W::new(self)
    }
    #[doc = "Bit 5 - I2C Slave Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcr_sfe(&mut self) -> I2C_MCR_SFE_W<MCR_SPEC, 5> {
        I2C_MCR_SFE_W::new(self)
    }
    #[doc = "Bit 6 - I2C Glitch Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcr_gfe(&mut self) -> I2C_MCR_GFE_W<MCR_SPEC, 6> {
        I2C_MCR_GFE_W::new(self)
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
#[doc = "I2C Master Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
