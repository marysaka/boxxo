#[doc = "Register `SCSR` reader"]
pub type R = crate::R<SCSR_SPEC>;
#[doc = "Register `SCSR` writer"]
pub type W = crate::W<SCSR_SPEC>;
#[doc = "Field `I2C_SCSR_RREQ` reader - Receive Request"]
pub type I2C_SCSR_RREQ_R = crate::BitReader;
#[doc = "Field `I2C_SCSR_RREQ` writer - Receive Request"]
pub type I2C_SCSR_RREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SCSR_FBR` reader - First Byte Received"]
pub type I2C_SCSR_FBR_R = crate::BitReader;
#[doc = "Field `I2C_SCSR_FBR` writer - First Byte Received"]
pub type I2C_SCSR_FBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SCSR_OAR2SEL` reader - OAR2 Address Matched"]
pub type I2C_SCSR_OAR2SEL_R = crate::BitReader;
#[doc = "Field `I2C_SCSR_OAR2SEL` writer - OAR2 Address Matched"]
pub type I2C_SCSR_OAR2SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    pub fn i2c_scsr_rreq(&self) -> I2C_SCSR_RREQ_R {
        I2C_SCSR_RREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - First Byte Received"]
    #[inline(always)]
    pub fn i2c_scsr_fbr(&self) -> I2C_SCSR_FBR_R {
        I2C_SCSR_FBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OAR2 Address Matched"]
    #[inline(always)]
    pub fn i2c_scsr_oar2sel(&self) -> I2C_SCSR_OAR2SEL_R {
        I2C_SCSR_OAR2SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scsr_rreq(&mut self) -> I2C_SCSR_RREQ_W<SCSR_SPEC, 0> {
        I2C_SCSR_RREQ_W::new(self)
    }
    #[doc = "Bit 2 - First Byte Received"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scsr_fbr(&mut self) -> I2C_SCSR_FBR_W<SCSR_SPEC, 2> {
        I2C_SCSR_FBR_W::new(self)
    }
    #[doc = "Bit 3 - OAR2 Address Matched"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scsr_oar2sel(&mut self) -> I2C_SCSR_OAR2SEL_W<SCSR_SPEC, 3> {
        I2C_SCSR_OAR2SEL_W::new(self)
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
#[doc = "I2C Slave Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scsr::R`](R) reader structure"]
impl crate::Readable for SCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scsr::W`](W) writer structure"]
impl crate::Writable for SCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
