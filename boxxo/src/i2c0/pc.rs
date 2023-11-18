#[doc = "Register `PC` reader"]
pub type R = crate::R<PC_SPEC>;
#[doc = "Register `PC` writer"]
pub type W = crate::W<PC_SPEC>;
#[doc = "Field `I2C_PC_HS` reader - High-Speed Capable"]
pub type I2C_PC_HS_R = crate::BitReader;
#[doc = "Field `I2C_PC_HS` writer - High-Speed Capable"]
pub type I2C_PC_HS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - High-Speed Capable"]
    #[inline(always)]
    pub fn i2c_pc_hs(&self) -> I2C_PC_HS_R {
        I2C_PC_HS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High-Speed Capable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pc_hs(&mut self) -> I2C_PC_HS_W<PC_SPEC, 0> {
        I2C_PC_HS_W::new(self)
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
#[doc = "I2C Peripheral Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc::R`](R) reader structure"]
impl crate::Readable for PC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc::W`](W) writer structure"]
impl crate::Writable for PC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
