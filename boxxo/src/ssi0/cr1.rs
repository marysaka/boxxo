#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `SSI_CR1_LBM` reader - SSI Loopback Mode"]
pub type SSI_CR1_LBM_R = crate::BitReader;
#[doc = "Field `SSI_CR1_LBM` writer - SSI Loopback Mode"]
pub type SSI_CR1_LBM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_CR1_SSE` reader - SSI Synchronous Serial Port Enable"]
pub type SSI_CR1_SSE_R = crate::BitReader;
#[doc = "Field `SSI_CR1_SSE` writer - SSI Synchronous Serial Port Enable"]
pub type SSI_CR1_SSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_CR1_MS` reader - SSI Master/Slave Select"]
pub type SSI_CR1_MS_R = crate::BitReader;
#[doc = "Field `SSI_CR1_MS` writer - SSI Master/Slave Select"]
pub type SSI_CR1_MS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_CR1_SOD` reader - SSI Slave Mode Output Disable"]
pub type SSI_CR1_SOD_R = crate::BitReader;
#[doc = "Field `SSI_CR1_SOD` writer - SSI Slave Mode Output Disable"]
pub type SSI_CR1_SOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_CR1_EOT` reader - End of Transmission"]
pub type SSI_CR1_EOT_R = crate::BitReader;
#[doc = "Field `SSI_CR1_EOT` writer - End of Transmission"]
pub type SSI_CR1_EOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    pub fn ssi_cr1_lbm(&self) -> SSI_CR1_LBM_R {
        SSI_CR1_LBM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    pub fn ssi_cr1_sse(&self) -> SSI_CR1_SSE_R {
        SSI_CR1_SSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    pub fn ssi_cr1_ms(&self) -> SSI_CR1_MS_R {
        SSI_CR1_MS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Slave Mode Output Disable"]
    #[inline(always)]
    pub fn ssi_cr1_sod(&self) -> SSI_CR1_SOD_R {
        SSI_CR1_SOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn ssi_cr1_eot(&self) -> SSI_CR1_EOT_R {
        SSI_CR1_EOT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr1_lbm(&mut self) -> SSI_CR1_LBM_W<CR1_SPEC, 0> {
        SSI_CR1_LBM_W::new(self)
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr1_sse(&mut self) -> SSI_CR1_SSE_W<CR1_SPEC, 1> {
        SSI_CR1_SSE_W::new(self)
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr1_ms(&mut self) -> SSI_CR1_MS_W<CR1_SPEC, 2> {
        SSI_CR1_MS_W::new(self)
    }
    #[doc = "Bit 3 - SSI Slave Mode Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr1_sod(&mut self) -> SSI_CR1_SOD_W<CR1_SPEC, 3> {
        SSI_CR1_SOD_W::new(self)
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr1_eot(&mut self) -> SSI_CR1_EOT_W<CR1_SPEC, 4> {
        SSI_CR1_EOT_W::new(self)
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
#[doc = "SSI Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
